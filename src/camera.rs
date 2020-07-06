use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Once;

use super::canvas::{Canvas, Pixel};
use super::matrix::{identity, Mat};
use super::ray::Ray;
use super::transformations::{rotate_y, translation};
use super::tuple::{point, vector, Tup};
use super::world::World;

use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub fov: f64,
    pub h_size: f64,
    pub v_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,

    transform: Mat,
    transform_inverse: Mat,
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
            transform_inverse: identity(4).inverse(),
        }
    }

    pub fn set_transform(&mut self, transform: Mat) {
        self.transform = transform.clone();
        self.transform_inverse = transform.inverse();
    }

    fn ray(&self, x: f64, y: f64) -> Ray {
        let x_off = (x + 0.5) * self.pixel_size;
        let y_off = (y + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_off;
        let world_y = self.half_height - y_off;

        let pixel = &self.transform_inverse * &point(world_x, world_y, -1.0);
        let origin = &self.transform_inverse * &point(0.0, 0.0, 0.0);
        let direction = (&pixel - &origin).normalize();

        return Ray { origin, direction };
    }

    pub fn render(&self, w: World, tx: Sender<Pixel>, shuffle: bool) {
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
            let p = w.color_at(self.ray(*x as f64, *y as f64));
            tx.send(Pixel { x: *x, y: *y, p }).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_size_horizontal_camera() {
        let c = Camera::new(200.0, 125.0, std::f64::consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() <= std::f64::EPSILON);
    }

    #[test]
    fn pixel_size_vertical_camera() {
        let c = Camera::new(125.0, 200.0, std::f64::consts::PI / 2.0);
        assert!((c.pixel_size - 0.01).abs() <= std::f64::EPSILON);
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201.0, 101.0, std::f64::consts::PI / 2.0);
        let r = c.ray(100.0, 50.0);

        assert_eq!(point(0.0, 0.0, 0.0), r.origin);

        assert!(r.direction.x.abs() <= std::f64::EPSILON);
        assert!(r.direction.y.abs() <= std::f64::EPSILON);
        assert!((r.direction.z - -1.0).abs() <= std::f64::EPSILON);
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201.0, 101.0, std::f64::consts::PI / 2.0);
        let r = c.ray(0.0, 0.0);

        assert_eq!(point(0.0, 0.0, 0.0), r.origin);

        assert!((r.direction.x - 0.66519).abs() <= 0.001);
        assert!((r.direction.y - 0.33259).abs() <= 0.001);
        assert!((r.direction.z - -0.66851).abs() <= 0.001);
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201.0, 101.0, std::f64::consts::PI / 2.0);
        c.transform = &rotate_y(std::f64::consts::PI / 4.0) * &translation(0.0, -2.0, 5.0);
        let r = c.ray(100.0, 50.0);

        assert_eq!(point(0.0, 2.0, -5.0), r.origin);

        let p = 2.0f64.sqrt() / 2.0;
        assert!((r.direction.x - p).abs() <= 0.001);
        assert!(r.direction.y <= 0.001);
        assert!((r.direction.z - -p).abs() <= 0.001);
    }
}