use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rstracer::tracer::canvas::Pixel;
use rstracer::tracer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (world, camera, rendering_spec) = scene_parser::from_reader(std::io::stdin())?;

    let (tx, rx): (Sender<Pixel>, Receiver<Pixel>) = mpsc::channel();

    let mut locations: Vec<(u32, u32, Sender<Pixel>)> = vec![];

    match rendering_spec.partial_render {
        None => {
            for y in 0..camera.v_size as u32 {
                for x in 0..camera.h_size as u32 {
                    locations.push((x, y, tx.clone()));
                }
            }
        }
        Some(pixels) => {
            pixels
                .into_iter()
                .for_each(|(x, y)| locations.push((x, y, tx.clone())));
        }
    }
    let num_pixels = locations.len();

    thread::spawn(move || {
        locations.par_iter_mut().for_each(|(x, y, tx)| {
            tx.send(Pixel {
                x: *x,
                y: *y,
                p: camera.render_pixel(&world, *x, *y),
            })
            .unwrap();
        });
    });

    let mut completed = 0;
    while let Ok(px) = rx.recv() {
        println!("{} {} {} {} {}", px.x, px.y, px.p.x, px.p.y, px.p.z);
        completed += 1;
        if completed == num_pixels {
            break
        }
    }

    Ok(())
}
