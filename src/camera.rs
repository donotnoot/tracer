use std::sync::mpsc::Sender;

use super::canvas::Pixel;
use super::matrix::{identity, Mat};
use super::ray::Ray;

use super::tuple::point;
use super::world::World;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

pub struct Camera {
    pub aspect_ratio: f32,
    pub fov: f32,
    pub h_size: f32,
    pub v_size: f32,
    pub half_width: f32,
    pub half_height: f32,
    pub pixel_size: f32,

    transform: Mat,
    transform_inverse: Mat,
}

impl Camera {
    pub fn new(h_size: f32, v_size: f32, fov: f32) -> Self {
        let aspect_ratio = h_size / v_size;
        let half = (fov / 2.0).tan();

        let (half_width, half_height) = if aspect_ratio >= 1.0 {
            (half, half / aspect_ratio)
        } else {
            (half * aspect_ratio, half)
        };

        Camera {
            aspect_ratio,
            fov,
            h_size,
            v_size,
            half_width,
            half_height,
            pixel_size: (half_width * 2.0) / h_size,
            transform: identity(4),
            transform_inverse: identity(4),
        }
    }

    pub fn set_transform(&mut self, transform: Mat) {
        self.transform = transform.clone();
        self.transform_inverse = transform.inverse();
    }

    fn ray(&self, x: f32, y: f32) -> Ray {
        let x_off = (x + 0.5) * self.pixel_size;
        let y_off = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_off;
        let world_y = self.half_height - y_off;

        let pixel = &self.transform_inverse * &point(world_x, world_y, -1.0);
        let origin = &self.transform_inverse * &point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, w: World, tx: Sender<Pixel>, shuffle: bool, reflection_limit: u64) {
        let mut locations: Vec<(u32, u32, Sender<Pixel>)> = vec![];

        for y in 0..self.v_size as u32 {
            for x in 0..self.h_size as u32 {
                locations.push((x, y, tx.clone()));
            }
        }

        if shuffle {
            locations.shuffle(&mut thread_rng());
        }

        locations.par_iter_mut().for_each(|(x, y, tx)| {
            let p = w.color_at(&self.ray(*x as f32, *y as f32), reflection_limit);
            tx.send(Pixel { x: *x, y: *y, p }).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::super::transformations::{rotate_y, translation};
    use super::*;

    #[test]
    fn pixel_size_horizontal_camera() {
        let c = Camera::new(200.0, 125.0, std::f32::consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn pixel_size_vertical_camera() {
        let c = Camera::new(125.0, 200.0, std::f32::consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201.0, 101.0, std::f32::consts::PI / 2.0);
        let r = c.ray(100.0, 50.0);

        assert_eq!(point(0.0, 0.0, 0.0), r.origin);

        assert!(r.direction.x.abs() <= std::f32::EPSILON);
        assert!(r.direction.y.abs() <= std::f32::EPSILON);
        assert!((r.direction.z - -1.0).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201.0, 101.0, std::f32::consts::PI / 2.0);
        let r = c.ray(0.0, 0.0);

        assert_eq!(point(0.0, 0.0, 0.0), r.origin);

        assert!((r.direction.x - 0.66519).abs() <= 0.001);
        assert!((r.direction.y - 0.33259).abs() <= 0.001);
        assert!((r.direction.z - -0.66851).abs() <= 0.001);
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201.0, 101.0, std::f32::consts::PI / 2.0);
        c.set_transform(&rotate_y(std::f32::consts::PI / 4.0) * &translation(0.0, -2.0, 5.0));
        let r = c.ray(100.0, 50.0);

        assert_eq!(point(0.0, 2.0, -5.0), r.origin);

        let p = 2.0f32.sqrt() / 2.0;
        assert!((r.direction.x - p).abs() <= 0.001);
        assert!(r.direction.y <= 0.001);
        assert!((r.direction.z - -p).abs() <= 0.001);
    }
}
