mod camera;
mod canvas;
mod intersections;
mod light;
mod material;
mod matrix;
mod objects;
mod ray;
mod transformations;
mod tuple;
mod world;

const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

fn main() {
    let mut world = world::World::new();
    world
        .objects
        .push(objects::Object::Sphere(objects::Sphere::new()));
    world.light = light::PointLight {
        intensity: tuple::color(1.0, 1.0, 1.0),
        position: tuple::point(-10.0, 10.0, -10.0),
    };

    let mut camera = camera::Camera::new(HEIGHT as f64, WIDTH as f64, std::f64::consts::PI / 2.0);
    camera.transform = transformations::view(
        tuple::point(0.0, 1.5, -5.0),
        tuple::point(0.0, 1.0, 0.0),
        tuple::vector(0.0, 1.0, 0.0),
    );

    let mut canvas = canvas::OpenGLCanvas::new(HEIGHT, WIDTH, "OpenGL Canvas".to_string());

    // this should somehow run concurrently
    camera.render(world, &mut canvas);

    // with this. this should also run in the main thread because opengl needs some thread-local
    // state.
    canvas.run();
}
