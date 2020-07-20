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
use tuple::{color, color_u8, Tup};
mod patterns;
use patterns::Pattern;

mod world;

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1920;

fn main() {
    // color palette
    let P_DARK: Tup = color_u8(0x10, 0x10, 0x10);
    let P_MED_DARK: Tup = color_u8(0x30, 0x30, 0x30);
    let P_MED: Tup = color_u8(0x43, 0x92, 0xF1);
    let P_MED_LIGHT: Tup = color_u8(0xDC, 0x49, 0x3A);
    let P_LIGHT: Tup = color_u8(0xE3, 0xEB, 0xFF);

    let mut world = world::World::new();
    world.objects = vec![];
    world.background_color = color(1.0, 1.0, 1.0);

    {
        let mut floor = objects::Plane::new();
        floor.material.color = color(1.0, 1.0, 1.0);
        floor.material.specular = 0.0;
        floor.material.reflectiveness = 0.1;
        floor.material.pattern = Some(Pattern::Ring(P_DARK.clone(), P_MED_DARK.clone(), None));
        world.objects.push(objects::Object::Plane(floor));
    }

    {
        let mut backdrop = objects::Plane::new();
        backdrop.transform = translation(0.0, 0.0, 3.0) * rotate_x(std::f32::consts::PI / 2.0);
        backdrop.material.color = color(1.0, 1.0, 1.0);
        backdrop.material.specular = 0.0;
        backdrop.material.pattern =
            Some(Pattern::Checker(P_LIGHT.clone(), P_MED_LIGHT.clone(), None));
        world.objects.push(objects::Object::Plane(backdrop));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(-3.5, 1.0, 0.0);// * scaling(2.0,2.0,2.0);
        object.material.reflectiveness = 0.05;
        object.material.pattern = Some(Pattern::Stripe(
            P_MED.clone(),
            P_MED_LIGHT.clone(),
            Some(translation(1.25, 1.0, 1.0) * rotate_z(3.14/1.2) * scaling(0.3, 1.0, 1.0)),
        ));
        world.objects.push(objects::Object::Sphere(object));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(0.0, 1.0, 0.0);
        object.material.color = color(1.0, 1.0, 1.0);
        object.material.reflectiveness = 0.85;
        object.material.specular = 0.99;
        object.material.ambient = 0.0;
        object.material.diffuse = 0.0;
        object.material.shininess = 200.0;
        world.objects.push(objects::Object::Sphere(object));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(2.5, 1.0, 0.0);
        object.material.pattern = Some(Pattern::Checker(
            P_MED.clone(),
            P_MED_DARK.clone(),
            Some(rotate_x(std::f32::consts::PI / 1.3)),
        ));
        world.objects.push(objects::Object::Sphere(object));
    }

    world.light = light::PointLight {
        intensity: tuple::color(1.0, 1.0, 1.0),
        position: tuple::point(-10.0, 10.0, -10.0),
    };

    let mut camera = camera::Camera::new(WIDTH as f32, HEIGHT as f32, std::f32::consts::PI / 2.0);
    camera.set_transform(transformations::view(
        tuple::point(1.0, 3.5, -5.0),
        tuple::point(0.0, 1.0, 0.0),
        tuple::vector(0.0, 1.0, 0.0),
    ));

    let mut canvas = canvas::OpenGLCanvas::new(
        WIDTH,
        HEIGHT,
        "OpenGL Canvas".to_string(),
        world.background_color.clone(),
    );

    let (tx, rx): (Sender<canvas::Pixel>, Receiver<canvas::Pixel>) = mpsc::channel();

    thread::spawn(move || {
        camera.render(world, tx, false, 32);
    });

    canvas.run(rx);
}
