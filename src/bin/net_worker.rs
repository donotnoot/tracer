use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use rstracer::tracer::*;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

pub mod net_render {
    tonic::include_proto!("net_render");
}

use net_render::worker_server::{Worker, WorkerServer};
use net_render::{Job, JobResult, Pixel, Spec};

#[derive(Debug)]
pub struct WorkerService {
    pub scenes: Arc<Mutex<HashMap<String, String>>>,
}

#[tonic::async_trait]
impl Worker for WorkerService {
    async fn scene(&self, request: Request<Spec>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let mut lock = self.scenes.lock().unwrap();
        lock.insert(request.id, request.spec);

        Ok(Response::new(()))
    }

    type WorkStream = mpsc::Receiver<Result<JobResult, Status>>;
    async fn work(&self, request: Request<Job>) -> Result<Response<Self::WorkStream>, Status> {
        println!("new request from {}", request.remote_addr().unwrap());

        let request = request.into_inner();

        let (mut tx, rx) = mpsc::channel(32);
        let (u_tx, mut u_rx) = mpsc::unbounded_channel::<Pixel>();

        let lock = self.scenes.lock().unwrap();
        let spec = match lock.get(&request.scene_id) {
            Some(spec_id) => spec_id,
            None => {
                return Err(Status::new(
                    Code::InvalidArgument,
                    "invalid rendering spec".to_string(),
                ))
            }
        };

        let (world, camera, rendering_spec) = match scene_parser::from_reader(spec.as_bytes()) {
            Ok((world, camera, rendering_spec)) => (world, camera, rendering_spec),
            Err(err) => {
                return Err(Status::new(
                    Code::InvalidArgument,
                    format!("invalid rendering spec: {}", err),
                ))
            }
        };

        println!("{} tile(s)", request.tiles.len());
        let mut pixels: Vec<(u32, u32, mpsc::UnboundedSender<Pixel>)> = vec![];
        for tile in request.tiles {
            println!("x: {}, y: {}, size: {}", tile.x, tile.y, tile.size);

            for x in tile.x..(tile.x + tile.size) {
                for y in tile.y..(tile.y + tile.size) {
                    pixels.push((x, y, u_tx.clone()));
                }
            }
        }

        if rendering_spec.randomize_rays {
            println!("shuffling pixels");
            pixels.shuffle(&mut thread_rng());
        }

        println!("processing {} pixels", pixels.len());
        let worker = thread::spawn(move || -> Result<(), Status> {
            pixels
                .into_par_iter()
                .with_max_len(1)
                .map(|(x, y, tx)| {
                    let pixel = Pixel {
                        x,
                        y,
                        color: tup_to_u32_color(camera.render_pixel(&world, x, y)),
                    };
                    match tx.send(pixel) {
                        Ok(()) => Ok(()),
                        Err(err) => Err(Status::new(
                            Code::Internal,
                            format!("could not send pixel between threads: {}", err),
                        )),
                    }
                })
                .collect::<Result<(), Status>>()
        });

        tokio::spawn(async move {
            let mut batch = vec![];

            while let Some(msg) = u_rx.recv().await {
                batch.push(msg);

                if batch.len() > 100 {
                    let job = Ok(JobResult {
                        pixels: batch.clone(),
                    });

                    if let Err(err) = tx.send(job).await {
                        return Err(Status::new(
                            Code::Internal,
                            format!("could not send pixel batch to client: {}", err),
                        ));
                    }
                    batch = vec![];
                }
            }

            let job = Ok(JobResult {
                pixels: batch.clone(),
            });

            if let Err(err) = tx.send(job).await {
                return Err(Status::new(
                    Code::Internal,
                    format!("could not send pixel batch to client: {}", err),
                ));
            }

            println!("done!");

            match worker.join().unwrap() {
                Ok(()) => Ok(()),
                Err(err) => Err(Status::new(Code::Internal, format!("{}", err))),
            }
        });

        Ok(Response::new(rx))
    }
}

fn tup_to_u32_color(t: tuple::Tup) -> u32 {
    let c = |c: f32| -> u8 { (c * 255.0) as u8 };
    ((c(t.x) as u32) << 16) + ((c(t.y) as u32) << 8) + (c(t.z) as u32)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:9000".parse().unwrap();
    let svc = WorkerServer::new(WorkerService {
        scenes: Arc::new(Mutex::new(HashMap::new())),
    });
    println!("serving on {}", addr);
    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}
