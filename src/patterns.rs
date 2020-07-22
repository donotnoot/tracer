use super::matrix::{identity, Mat};
use super::objects::Object;

use super::tuple::{color, Tup};

use num_complex::Complex;

#[derive(Debug, Clone)]
pub enum Pattern {
    Stripe(Tup, Tup, Option<Mat>),
    Gradient(Tup, Tup, Option<Mat>),
    Checker(Tup, Tup, Option<Mat>),
    Ring(Tup, Tup, Option<Mat>),
    Mandelbrot(Tup, Option<Mat>),
}

impl Pattern {
    pub fn at(&self, p: &Tup) -> Tup {
        match self {
            Pattern::Stripe(a, b, _) => Pattern::stripe(p, a.clone(), b.clone()),
            Pattern::Gradient(a, b, _) => Pattern::gradient(p, a.clone(), b.clone()),
            Pattern::Checker(a, b, _) => Pattern::checker(p, a.clone(), b.clone()),
            Pattern::Ring(a, b, _) => Pattern::ring(p, a.clone(), b.clone()),
            Pattern::Mandelbrot(a, _) => Pattern::mandelbrot(p, a.clone()),
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
            _ => identity(4),
        };
        let pattern_space = &transform.inverse() * &object_space;

        match self {
            Pattern::Stripe(a, b, _) => Pattern::stripe(&pattern_space, a.clone(), b.clone()),
            Pattern::Gradient(a, b, _) => Pattern::gradient(&pattern_space, a.clone(), b.clone()),
            Pattern::Checker(a, b, _) => Pattern::checker(&pattern_space, a.clone(), b.clone()),
            Pattern::Ring(a, b, _) => Pattern::ring(&pattern_space, a.clone(), b.clone()),
            Pattern::Mandelbrot(a, _) => Pattern::mandelbrot(&pattern_space, a.clone()),
        }
    }

    fn mandelbrot(p: &Tup, a: Tup) -> Tup {
        let max_iterations = 256u16;
        let img_side = 800u32;
        let cxmin = -2f32;
        let cxmax = 1f32;
        let cymin = -1.5f32;
        let cymax = 1.5f32;
        let _scalex = (cxmax - cxmin) / img_side as f32;
        let _scaley = (cymax - cymin) / img_side as f32;

        let cx = cxmin + p.x as f32;
        let cy = cymin + p.z as f32;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);

        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }

        if i >= max_iterations - 1 {
            color(0.0, 0.0, 0.0)
        } else {
            a
        }
    }

    fn stripe(p: &Tup, a: Tup, b: Tup) -> Tup {
        if p.x.floor() as i64 % 2 == 0 {
            a
        } else {
            b
        }
    }

    fn checker(p: &Tup, a: Tup, b: Tup) -> Tup {
        if (p.x.floor() + p.y.floor() + p.z.floor()) as i64 % 2 == 0 {
            a
        } else {
            b
        }
    }

    fn ring(p: &Tup, a: Tup, b: Tup) -> Tup {
        if ((p.x.powi(2) + p.z.powi(2)).sqrt()).floor() as i64 % 2 == 0 {
            a
        } else {
            b
        }
    }

    fn gradient(p: &Tup, a: Tup, b: Tup) -> Tup {
        let dist = &b - &a;
        let frac = p.x - p.x.floor();
        &a + &(&dist * frac)
    }
}

#[cfg(test)]
mod tests {
    use super::super::objects::Sphere;
    use super::super::transformations::{scaling, translation};
    use super::super::tuple::point;
    use super::*;

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
            let mut obj = Sphere::new();
            obj.transform = scaling(2.0, 2.0, 2.0);
            let stripe = Pattern::Stripe(color(1.0, 1.0, 1.0), color(0.0, 0.0, 0.0), None);
            let c = stripe.at_object(&Object::Sphere(obj), &point(1.5, 0.0, 0.0));
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
        {
            // pattern transform
            let obj = Sphere::new();
            let stripe = Pattern::Stripe(
                color(1.0, 1.0, 1.0),
                color(0.0, 0.0, 0.0),
                Some(scaling(2.0, 2.0, 2.0)),
            );
            let c = stripe.at_object(&Object::Sphere(obj), &point(1.5, 0.0, 0.0));
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
        {
            // both
            let mut obj = Sphere::new();
            obj.transform = scaling(2.0, 2.0, 2.0);
            let stripe = Pattern::Stripe(
                color(1.0, 1.0, 1.0),
                color(0.0, 0.0, 0.0),
                Some(translation(0.5, 0.0, 0.0)),
            );
            let c = stripe.at_object(&Object::Sphere(obj), &point(2.5, 0.0, 0.0));
            assert_eq!(c, color(1.0, 1.0, 1.0));
        }
    }
}
