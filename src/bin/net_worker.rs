#![recursion_limit = "1024"]

use clap::{App, Arg};
use futures::{Stream, StreamExt};
use log::*;
use pretty_env_logger;
use rayon::prelude::*;
use rstracer::tracer::*;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::pin::Pin;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status, Streaming};

pub mod net_render {
    tonic::include_proto!("net_render");
}

use net_render::job::Request as JobRequest;
use net_render::worker_server::{Worker, WorkerServer};
use net_render::{Job, Pixel, Pixels};

#[derive(Debug)]
pub struct WorkerService {}

#[tonic::async_trait]
impl Worker for WorkerService {
    type RenderStream = Pin<Box<dyn Stream<Item = Result<Pixels, Status>> + Send + Sync + 'static>>;

    async fn render(
        &self,
        request: Request<Streaming<Job>>,
    ) -> Result<Response<Self::RenderStream>, Status> {
        let remote_address = request.remote_addr().unwrap();
        info!("Accepted request from {}", remote_address);

        let mut stream = request.into_inner();

        let job = stream.next().await.unwrap().unwrap();
        let scene = match job.request.unwrap() {
            JobRequest::Scene(scene) => Ok(scene),
            _ => Err(Status::new(
                Code::InvalidArgument,
                "fist message must be scene!".to_string(),
            )),
        }?;
        info!("Received scene successfully");

        let (world, camera, _) = match scene_parser::from_reader(scene.as_bytes()) {
            Ok((world, camera, rendering_spec)) => Ok((world, camera, rendering_spec)),
            Err(err) => Err(Status::new(
                Code::InvalidArgument,
                format!("invalid rendering spec: {}", err),
            )),
        }?;
        info!("Parsed scene successfully");

        let output = async_stream::try_stream! {
            while let Some(job) = stream.next().await {
                let tile = match job?.request.unwrap() {
                    JobRequest::Tile(tile) => Ok(tile),
                    _ => Err(Status::new(
                        Code::InvalidArgument,
                        "consecutive messages must be tiles!".to_string(),
                    )),
                }?;
                info!("Rendering tile: {:?}", tile);

                let mut pixels: Vec<(u32, u32)> = vec![];
                for x in tile.x..(tile.x + tile.size) {
                    for y in tile.y..(tile.y + tile.size) {
                        pixels.push((x, y));
                    }
                }

                let pixels = pixels
                    .into_par_iter()
                    .with_max_len(1)
                    .map(|(x, y)| {
                        Pixel {
                            x,
                            y,
                            color: tup_to_u32_color(camera.render_pixel(&world, x, y)),
                        }
                    }).collect::<Vec<Pixel>>();

                info!("Rendered tile successfully");

                yield Pixels{pixels}
            }
        };

        Ok(Response::new(Box::pin(output) as Self::RenderStream))
    }
}

fn tup_to_u32_color(t: tuple::Tup) -> u32 {
    let c = |c: f32| -> u8 {
        let unclipped = c * 255.0;
        if unclipped > 255. {
            255u8
        } else if unclipped < 0. {
            0u8
        } else {
            unclipped as u8
        }
    };
    ((c(t.x) as u32) << 16) + ((c(t.y) as u32) << 8) + (c(t.z) as u32)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::formatted_builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    let matches = App::new("Distracer Worker")
        .version("0.1.0")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .default_value("11811")
                .help("Port on which to start serving"),
        )
        .get_matches();

    let port: u16 = matches.value_of("port").unwrap().parse()?;
    info!("Using port {}", port);

    let socket_v4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port);

    let svc = WorkerServer::new(WorkerService {});

    info!("Serving on {}", socket_v4);
    Server::builder()
        .add_service(svc)
        .serve(SocketAddr::V4(socket_v4))
        .await?;

    Ok(())
}
