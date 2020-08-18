use super::matrix::{identity, Mat};
use super::objects::Object;
use super::tuple::{color, color_u8, vector, Tup};
use num_complex::Complex;
use std::io;

#[derive(Debug, Clone)]
pub enum Pattern {
    Stripe(Tup, Tup, Option<Mat>),
    Gradient(Tup, Tup, Option<Mat>),
    Checker(Tup, Tup, Option<Mat>),
    UV(UVMapping, UVPattern),
    Ring(Tup, Tup, Option<Mat>),
    Mandelbrot(Tup, Option<Mat>),
}

#[derive(Debug, Clone)]
pub enum UVPattern {
    Checker(Tup, Tup, f32, f32),
    Image(Texture),
    CubeImage {
        top: Texture,
        bottom: Texture,
        left: Texture,
        right: Texture,
        front: Texture,
        back: Texture,
    },
}

#[derive(Debug, Clone)]
pub enum UVMapping {
    Spherical,
    Planar,
    Cubical,
}

#[derive(Debug)]
enum CubeFace {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

#[derive(Debug, PartialEq)]
enum TwoColors {
    ColorA,
    ColorB,
}

impl Pattern {
    pub fn at(&self, p: &Tup) -> Tup {
        match self {
            Pattern::Stripe(a, b, _) => match Pattern::stripe(p) {
                TwoColors::ColorA => a.clone(),
                TwoColors::ColorB => b.clone(),
            },
            Pattern::Gradient(a, b, _) => Pattern::gradient(p, a, b),
            Pattern::Checker(a, b, _) => match Pattern::checker(p) {
                TwoColors::ColorA => a.clone(),
                TwoColors::ColorB => b.clone(),
            },
            Pattern::Ring(a, b, _) => match Pattern::ring(p) {
                TwoColors::ColorA => a.clone(),
                TwoColors::ColorB => b.clone(),
            },
            Pattern::Mandelbrot(a, _) => Pattern::mandelbrot(p, a.clone()),
            Pattern::UV(mapping, pattern) => {
                let ((u, v), face) = match mapping {
                    UVMapping::Spherical => (Pattern::spherical_map(p), None),
                    UVMapping::Planar => (Pattern::planar_map(p), None),
                    UVMapping::Cubical => {
                        let face = Pattern::cube_face_at_point(p);
                        let (u, v) = Pattern::cube_map(p, &face);
                        ((u, v), Some(face))
                    }
                };
                match pattern {
                    UVPattern::Checker(color_a, color_b, width, height) => {
                        match Pattern::uv_checker(*width, *height, u, v) {
                            TwoColors::ColorA => color_a.clone(),
                            TwoColors::ColorB => color_b.clone(),
                        }
                    }
                    UVPattern::Image(texture) => Pattern::uv_image(texture, u, v),
                    UVPattern::CubeImage {
                        top,
                        bottom,
                        left,
                        right,
                        front,
                        back,
                    } => match face {
                        None => color(0., 0., 0.),
                        Some(CubeFace::Top) => Pattern::uv_image(top, u, v),
                        Some(CubeFace::Bottom) => Pattern::uv_image(bottom, u, v),
                        Some(CubeFace::Left) => Pattern::uv_image(left, u, v),
                        Some(CubeFace::Right) => Pattern::uv_image(right, u, v),
                        Some(CubeFace::Front) => Pattern::uv_image(front, u, v),
                        Some(CubeFace::Back) => Pattern::uv_image(back, u, v),
                    },
                }
            }
        }
    }

    pub fn at_object(&self, o: &Object, p: &Tup) -> Tup {
        let object_space = &o.transformation().inverse() * p;
        let transform = match self {
            Pattern::Stripe(_, _, Some(t)) => t.clone(),
            Pattern::Gradient(_, _, Some(t)) => t.clone(),
            Pattern::Checker(_, _, Some(t)) => t.clone(),
            Pattern::Ring(_, _, Some(t)) => t.clone(),
            Pattern::Mandelbrot(_, Some(t)) => t.clone(),
            _ => identity(),
        };
        let pattern_space = &transform.inverse() * &object_space;

        self.at(&pattern_space)
    }

    fn uv_checker(width: f32, height: f32, u: f32, v: f32) -> TwoColors {
        let (u, v) = ((u * width).floor() as u32, (v * height).floor() as u32);

        if (u + v) % 2 == 0 {
            TwoColors::ColorA
        } else {
            TwoColors::ColorB
        }
    }

    fn uv_image(texture: &Texture, u: f32, v: f32) -> Tup {
        let v = 1. - v;

        let x = u * (texture.width - 1) as f32;
        let y = v * (texture.height - 1) as f32;

        texture
            .color_at(x.round() as u32, y.round() as u32)
            .unwrap_or(color(1., 0., 1.))
    }

    fn spherical_map(p: &Tup) -> (f32, f32) {
        let tetha = p.x.atan2(p.z);
        let vec = vector(p.x, p.y, p.z);
        let radius = vec.magnitude();
        let phi = (p.y / radius).acos();
        let raw_u = tetha / (std::f32::consts::PI * 2.);
        let u = 1. - (raw_u + 0.5);
        let v = 1. - phi / std::f32::consts::PI;
        (u, v)
    }

    fn planar_map(p: &Tup) -> (f32, f32) {
        (p.x.rem_euclid(1.), p.z.rem_euclid(1.))
    }

    fn cube_face_at_point(p: &Tup) -> CubeFace {
        let (absx, absy, absz) = (p.x.abs(), p.y.abs(), p.z.abs());
        let coord = *[absx, absy, absz]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if coord == p.x {
            return CubeFace::Right;
        }
        if coord == -p.x {
            return CubeFace::Left;
        }
        if coord == p.y {
            return CubeFace::Top;
        }
        if coord == -p.y {
            return CubeFace::Bottom;
        }
        if coord == p.z {
            return CubeFace::Front;
        }

        return CubeFace::Back;
    }

    fn cube_map(p: &Tup, face: &CubeFace) -> (f32, f32) {
        match face {
            CubeFace::Front => (
                (p.x + 1.).rem_euclid(2.) / 2.,
                (p.y + 1.).rem_euclid(2.) / 2.,
            ),
            CubeFace::Back => (
                (1. - p.x).rem_euclid(2.) / 2.,
                (p.y + 1.).rem_euclid(2.) / 2.,
            ),
            CubeFace::Left => (
                (p.z + 1.).rem_euclid(2.) / 2.,
                (p.y + 1.).rem_euclid(2.) / 2.,
            ),
            CubeFace::Right => (
                (1. - p.z).rem_euclid(2.) / 2.,
                (p.y + 1.).rem_euclid(2.) / 2.,
            ),
            CubeFace::Top => (
                (p.x + 1.).rem_euclid(2.) / 2.,
                (1. - p.z).rem_euclid(2.) / 2.,
            ),
            CubeFace::Bottom => (
                (p.x + 1.).rem_euclid(2.) / 2.,
                (p.z + 1.).rem_euclid(2.) / 2.,
            ),
        }
    }

    fn mandelbrot(point: &Tup, a: Tup) -> Tup {
        let max_iterations = 256u16;
        let img_side = 800u32;
        let cxmin = -2f32;
        let cxmax = 1f32;
        let cymin = -1.5f32;
        let cymax = 1.5f32;
        let _scalex = (cxmax - cxmin) / img_side as f32;
        let _scaley = (cymax - cymin) / img_side as f32;

        let cx = cxmin + point.x as f32;
        let cy = cymin + point.z as f32;

        let complex = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);

        let mut escape_t = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + complex;
            escape_t = t;
        }

        if escape_t >= max_iterations - 1 {
            color(0.0, 0.0, 0.0)
        } else {
            a
        }
    }

    fn stripe(p: &Tup) -> TwoColors {
        if p.x.floor() as i64 % 2 == 0 {
            TwoColors::ColorA
        } else {
            TwoColors::ColorB
        }
    }

    fn checker(p: &Tup) -> TwoColors {
        if (p.x.floor() + p.y.floor() + p.z.floor()) as i64 % 2 == 0 {
            TwoColors::ColorA
        } else {
            TwoColors::ColorB
        }
    }

    fn ring(p: &Tup) -> TwoColors {
        if ((p.x.powi(2) + p.z.powi(2)).sqrt()).floor() as i64 % 2 == 0 {
            TwoColors::ColorA
        } else {
            TwoColors::ColorB
        }
    }

    fn gradient(p: &Tup, a: &Tup, b: &Tup) -> Tup {
        let dist = b - a;
        let frac = p.x - p.x.floor();
        a + &(dist * frac)
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    data: Vec<u8>,
    height: u32,
    width: u32,
    bytes_per_px: usize,
}

impl Texture {
    pub fn read(r: impl io::Read) -> io::Result<Self> {
        let decoder = png::Decoder::new(r);
        let (info, mut reader) = decoder.read_info()?;
        let mut data: Vec<u8> = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)?;
        let info = reader.info();
        Ok(Texture {
            data,
            height: info.height,
            width: info.width,
            bytes_per_px: reader.info().bytes_per_pixel(),
        })
    }

    pub fn color_at(&self, x: u32, y: u32) -> Option<Tup> {
        let idx = (x + y * self.width) as usize * self.bytes_per_px;

        let red = self.data.get(idx)?;
        let green = self.data.get(idx + 1)?;
        let blue = self.data.get(idx + 2)?;

        Some(color_u8(*red, *green, *blue))
    }
}

#[cfg(test)]
mod tests {
    use super::super::material::Material;
    use super::super::objects::{Geometry, Object, Sphere};
    use super::super::transformations::{scaling, translation};
    use super::super::tuple::point;
    use super::*;

    #[test]
    fn uv_checker() {
        vec![
            (0., 0., TwoColors::ColorA),
            (0., 0.5, TwoColors::ColorB),
            (0.5, 0., TwoColors::ColorB),
            (0.5, 0.5, TwoColors::ColorA),
            (1., 1., TwoColors::ColorA),
        ]
        .into_iter()
        .for_each(|(u, v, expected)| {
            let result = Pattern::uv_checker(2., 2., u, v);
            assert_eq!(result, expected);
        });
    }

    #[test]
    fn spherical_map() {
        let p = 2f32.sqrt() / 2.;
        vec![
            (point(0., 0., -1.), 0., 0.5),
            (point(1., 0., 0.), 0.25, 0.5),
            (point(0., 0., 1.), 0.5, 0.5),
            (point(-1., 0., 0.), 0.75, 0.5),
            (point(0., 1., 0.), 0.5, 1.),
            (point(0., -1., 0.), 0.5, 0.),
            (point(p, p, 0.), 0.25, 0.75),
        ]
        .into_iter()
        .for_each(|(p, ex_u, ex_v)| {
            let (u, v) = Pattern::spherical_map(&p);
            assert_eq!(u, ex_u);
            assert_eq!(v, ex_v);
        });
    }

    #[test]
    fn planar_map() {
        vec![
            (point(0.25, 0., 0.5), 0.25, 0.5),
            (point(0.25, 0., -0.25), 0.25, 0.75),
            (point(0.25, 0.5, -0.25), 0.25, 0.75),
            (point(1.25, 0., -1.75), 0.25, 0.25),
            (point(1., 0., -1.), 0.0, 0.0),
            (point(0., 0., 0.), 0.0, 0.0),
        ]
        .into_iter()
        .for_each(|(p, ex_u, ex_v)| {
            let (u, v) = Pattern::planar_map(&p);
            assert_eq!(u, ex_u);
            assert_eq!(v, ex_v);
        });
    }

    #[test]
    fn stripe_pattern() {
        let stripe = Pattern::Stripe(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0), None);
        {
            // constant in y
            for y in 0..100 {
                assert_eq!(stripe.at(&point(0.0, y as f32, 0.0)), color(1.0, 1.0, 1.0));
            }
        }
        {
            // constant in z
            for z in 0..100 {
                assert_eq!(stripe.at(&point(0.0, 0.0, z as f32)), color(1.0, 1.0, 1.0));
            }
        }
        {
            // alternates in x
            let mut last_was = stripe.at(&point(0.0, 0.0, 0.0));
            for x in 1..100 {
                let this = stripe.at(&point(x as f32, 0.0, 0.0));
                assert_ne!(this, last_was);
                last_was = stripe.at(&point(x as f32, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn stripes_with_transforms() {
        {
            // object transform
            let obj = Sphere::new(scaling(2.0, 2.0, 2.0));
            let stripe = Pattern::Stripe(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0), None);
            let c = stripe.at_object(
                &Object {
                    geometry: Geometry::Sphere(obj),
                    material: Material::new(),
                },
                &point(1.5, 0.0, 0.0),
            );
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
        {
            // pattern transform
            let obj = Sphere::default();
            let stripe = Pattern::Stripe(
                color(1.0, 1.0, 1.0),
                color(0.0, 0.0, 0.0),
                Some(scaling(2.0, 2.0, 2.0)),
            );
            let c = stripe.at_object(
                &Object {
                    geometry: Geometry::Sphere(obj),
                    material: Material::new(),
                },
                &point(1.5, 0.0, 0.0),
            );
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
        {
            // both
            let obj = Sphere::new(scaling(2.0, 2.0, 2.0));
            let stripe = Pattern::Stripe(
                color(1.0, 1.0, 1.0),
                color(0.0, 0.0, 0.0),
                Some(translation(0.5, 0.0, 0.0)),
            );
            let c = stripe.at_object(
                &Object {
                    geometry: Geometry::Sphere(obj),
                    material: Material::new(),
                },
                &point(2.5, 0.0, 0.0),
            );
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
    }
}
