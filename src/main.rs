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
    let P_DARK: Tup = color_u8(0x0A, 0x11, 0x28);
    let P_MED_DARK: Tup = color_u8(0x00, 0x1F, 0x54);
    let P_MED: Tup = color_u8(0x3, 0x40, 0x78);
    let P_MED_LIGHT: Tup = color_u8(0x12, 0x82, 0xA2);
    let P_LIGHT: Tup = color_u8(0xFE, 0xFC, 0xFB);

    let mut world = world::World::new();
    world.objects = vec![];
    world.background_color = color(0.8, 0.8, 0.85);

    {
        let mut floor = objects::Plane::new();
        floor.material.color = color(1.0, 0.9, 0.9);
        floor.material.specular = 0.0;
        floor.material.reflectiveness = 0.3;
        floor.material.pattern = Some(Pattern::Checker(P_DARK.clone(), P_MED_DARK.clone(), None));
        world.objects.push(objects::Object::Plane(floor));
    }

    {
        let mut backdrop = objects::Plane::new();
        backdrop.transform = translation(0.0, 0.0, 3.0) * rotate_x(std::f32::consts::PI / 2.0);
        backdrop.material.color = color(1.0, 0.9, 0.9);
        backdrop.material.specular = 0.0;
        backdrop.material.pattern =
            Some(Pattern::Checker(P_LIGHT.clone(), P_MED_LIGHT.clone(), None));
        world.objects.push(objects::Object::Plane(backdrop));
    }

    {
        let mut frontdrop = objects::Plane::new();
        frontdrop.transform =
            translation(0.0, 0.0, -10.0) * rotate_x(3.0 * std::f32::consts::PI / 2.0);
        frontdrop.material.color = color(1.0, 1.0, 1.0);
        frontdrop.material.specular = 0.0;
        frontdrop.material.pattern =
            Some(Pattern::Checker(P_MED_LIGHT.clone(), P_LIGHT.clone(), None));
        // world.objects.push(objects::Object::Plane(frontdrop));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(-2.5, 1.0, 0.0) * scaling(1.5, 1.5, 1.5);
        object.material.reflectiveness = 0.8;
        object.material.pattern = Some(Pattern::Stripe(
            P_MED.clone(),
            P_MED_LIGHT.clone(),
            Some(scaling(0.1, 0.1, 0.1)),
        ));
        world.objects.push(objects::Object::Sphere(object));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(0.0, 1.0, 0.0);
        object.material.color = color(1.0, 1.0, 1.0);
        object.material.reflectiveness = 0.99;
        object.material.specular = 0.99;
        object.material.ambient = 0.01;
        object.material.diffuse = 0.1;
        object.material.shininess = 500.0;
        world.objects.push(objects::Object::Sphere(object));
    }

    {
        let mut object = objects::Sphere::new();
        object.transform = translation(2.5, 1.0, 0.0);
        object.material.pattern = Some(Pattern::Checker(
            P_MED.clone(),
            P_MED_DARK.clone(),
            Some(scaling(0.1, 0.1, 0.1)),
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
        camera.render(world, tx, true, 4096);
    });

    canvas.run(rx);
}
