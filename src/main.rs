use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod camera;
mod canvas;
mod intersections;
mod light;
mod material;
mod matrix;
mod objects;
mod ray;
mod transformations;
use transformations::{rotate_x, rotate_y, rotate_z, scaling, shearing, translation, view};
mod tuple;
use tuple::{color, point, vector};
mod world;

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

fn main() {
    let mut world = world::World::new();
    world.objects = vec![];

    {
        let mut floor = objects::Sphere::new();
        floor.transform = scaling(10.0, 0.01, 10.0);
        floor.material.color = color(1.0, 0.9, 0.9);
        floor.material.specular = 0.0;
        world.objects.push(objects::Object::Sphere(floor));
    }

    {
        let mut left_wall = objects::Sphere::new();
        left_wall.transform = &translation(0.0, 0.0, 5.0)
            * &(&rotate_y(-(std::f64::consts::PI / 4.0))
                * &(&rotate_x(std::f64::consts::PI / 2.0) * &scaling(10.0, 0.01, 10.0)));
        left_wall.material.color = color(1.0, 0.9, 0.9);
        left_wall.material.specular = 0.0;
        world.objects.push(objects::Object::Sphere(left_wall));
    }

    {
        let mut right_wall = objects::Sphere::new();
        right_wall.transform = &translation(0.0, 0.0, 5.0)
            * &(&rotate_y(std::f64::consts::PI / 4.0)
                * &(&rotate_x(std::f64::consts::PI / 2.0) * &scaling(10.0, 0.01, 10.0)));
        right_wall.material.color = color(1.0, 0.9, 0.9);
        right_wall.material.specular = 0.0;
        world.objects.push(objects::Object::Sphere(right_wall));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(1.5, 0.0, 0.0);
        world.objects.push(objects::Object::Sphere(object));
    }

    world.light = light::PointLight {
        intensity: tuple::color(1.0, 1.0, 1.0),
        position: tuple::point(-10.0, 10.0, -10.0),
    };

    let mut camera = camera::Camera::new(HEIGHT as f64, WIDTH as f64, std::f64::consts::PI / 3.0);
    camera.set_transform(transformations::view(
        tuple::point(0.0, 1.5, -5.0),
        tuple::point(0.0, 1.0, 0.0),
        tuple::vector(0.0, 1.0, 0.0),
    ));

    let mut canvas = canvas::OpenGLCanvas::new(HEIGHT, WIDTH, "OpenGL Canvas".to_string());

    let (tx, rx): (Sender<canvas::Pixel>, Receiver<canvas::Pixel>) = mpsc::channel();

    thread::spawn(move || {
        camera.render(world, tx, true);
    });

    canvas.run(rx);
}
