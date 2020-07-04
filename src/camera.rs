use super::matrix::{identity, Mat};
use super::ray::Ray;
use super::world::World;
use super::tuple::{point, Tup};
use super::canvas::Canvas;

pub struct Camera {
    pub aspect_ratio: f64,
    pub fov: f64,
    pub h_size: f64,
    pub v_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
    pub transform: Mat,
}

impl Camera {
    pub fn new(h_size: f64, v_size: f64, fov: f64) -> Self {
        let aspect_ratio = h_size / v_size;
        let half = (fov / 2.0).tan();

        let half_width;
        let half_height;
        if aspect_ratio >= 1.0 {
            half_width = half;
            half_height = half / aspect_ratio;
        } else {
            half_width = half * aspect_ratio;
            half_height = half;
        }

        Camera {
            aspect_ratio,
            fov,
            h_size,
            v_size,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / h_size,
            transform: identity(4),
        }
    }

    fn ray(&self, x: f64, y: f64) -> Ray {
        let x_off = (x + 0.5) * self.pixel_size;
        let y_off = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_off;
        let world_y = self.half_height - y_off;

        // TODO: should be pre-baked, it's the same for all rays.
        let transform_inverse = self.transform.inverse();

        let pixel = &transform_inverse * &point(world_x, world_y, -1.0);
        let origin = &transform_inverse * &point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        return Ray { origin, direction };
    }

    pub fn render(&self, w: World, c: &mut impl Canvas) {
        for y in 0..self.v_size as u32 {
            for x in 0..self.h_size as u32 {
                c.write_pixel(x, y, w.color_at(self.ray(x as f64, y as f64)))
            }
        }
    }
}
