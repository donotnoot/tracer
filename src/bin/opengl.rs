use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rstracer::tracer::*;
use rstracer::tracer::canvas::Pixel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (world, camera, rendering_spec) = scene_parser::from_reader(std::io::stdin())?;

    let mut canvas = canvas::OpenGLCanvas::new(
        camera.h_size as u32,
        camera.v_size as u32,
        "OpenGL Canvas".to_string(),
        world.background_color.clone(),
    );

    let (tx, rx): (Sender<Pixel>, Receiver<Pixel>) = mpsc::channel();

    thread::spawn(move || {
        camera.render(
            world,
            tx,
            rendering_spec.randomize_rays,
        );
    });

    canvas.run(rx);

    Ok(())
}
