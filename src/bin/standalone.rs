use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use rstracer::tracer::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (world, camera, rendering_spec) = scene_parser::stdin_world()?;

    let mut canvas = canvas::OpenGLCanvas::new(
        camera.h_size as u32,
        camera.v_size as u32,
        "OpenGL Canvas".to_string(),
        world.background_color.clone(),
    );

    let (tx, rx): (Sender<canvas::Pixel>, Receiver<canvas::Pixel>) = mpsc::channel();

    thread::spawn(move || {
        camera.render(
            world,
            tx,
            rendering_spec.randomize_rays,
            rendering_spec.max_bounces,
        );
    });

    canvas.run(rx);

    Ok(())
}
