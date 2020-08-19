use super::objects::Object;
use super::ray::Ray;

use super::tuple::{dot, Tup};

pub type Intersections<'a> = Vec<Intersection<'a>>;

pub fn hit(i: &[Intersection]) -> (f32, usize, bool) {
    let mut min = std::f32::INFINITY;
    let mut index: usize = 0;
    let mut hit = false;

    for (i, elem) in i.iter().enumerate() {
        if elem.t < 0.0 {
            continue;
        }
        if elem.t < min {
            min = elem.t;
            hit = true;
            index = i;
        }
    }

    (min, index, hit)
}

#[derive(Debug)]
pub struct Computations<'a> {
    pub t: f32,
    pub object: &'a Object,
    pub inside: bool,
    pub point: Tup,
    pub eye: Tup,
    pub normal: Tup,
    pub reflection: Tup,
    pub over_point: Tup,
    pub under_point: Tup,
    pub n1: f32,
    pub n2: f32,
}

impl<'a> Computations<'a> {
    pub fn schlick(&self) -> f32 {
        let mut cos = dot(&self.eye, &self.normal);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;

            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    pub object: &'a Object,
    pub uv: Option<(f32, f32)>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a Object, uv: Option<(f32, f32)>) -> Self {
        Intersection { t, object, uv }
    }

    pub fn computations(&self, r: &Ray, xs: Option<&Intersections>) -> Computations {
        let point = r.position(self.t);
        let eye = -&r.direction;

        let (inside, normal) = {
            let normal = self.object.normal(&point);
            if dot(&normal, &eye) < 0.0 {
                (true, -&normal)
            } else {
                (false, normal)
            }
        };

        let reflection = r.direction.reflect(&normal);
        let over_point = &point + &(&normal * 10e-5);
        let under_point = &point - &(&normal * 10e-5);

        let (n1, n2) = match xs {
            Some(xs) => self.calculate_refractions(xs),
            None => (1.0, 1.0),
        };

        Computations {
            t: self.t,
            object: self.object,
            point,
            eye,
            normal,
            over_point,
            under_point,
            reflection,
            inside,
            n1,
            n2,
        }
    }

    fn calculate_refractions(&self, xs: &[Intersection]) -> (f32, f32) {
        let mut n1: f32 = 1.0;
        let mut n2: f32 = 1.0;

        if xs.is_empty() {
            return (n1, n2);
        }

        // todo: what if no hit?
        let mut containers: Vec<&Object> = Vec::with_capacity(xs.len());

        for i in xs.iter() {
            let is_hit = std::ptr::eq(self, i);

            if is_hit {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material.refractive_index;
                }
            }

            if let Some(idx) = containers.iter().position(|&e| std::ptr::eq(e, i.object)) {
                containers.remove(idx);
            } else {
                containers.push(&i.object);
            }

            if is_hit {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material.refractive_index;
                }
                break;
            }
        }

        (n1, n2)
    }
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Intersections;
}

#[cfg(test)]
mod tests {
    use super::super::material::Material;
    use super::super::objects::{Geometry, Object, Plane, Sphere};
    use super::super::transformations::{scaling, translation};
    use super::super::tuple::{point, vector};
    use super::*;

    fn make_sphere() -> Object {
        Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        }
    }

    fn make_glass_sphere() -> Object {
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material,
        }
    }

    #[test]
    fn getting_the_hit_when_all_are_positive() {
        let sphere = make_sphere();
        let ixs: Intersections = vec![
            Intersection::new(1.0, &sphere, None),
            Intersection::new(2.0, &sphere, None),
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, 1.0);
        assert_eq!(index, 0);
        assert!(hit);
    }

    #[test]
    fn getting_the_hit_when_some_pos_some_neg() {
        let sphere = make_sphere();
        let ixs: Intersections = vec![
            Intersection::new(-1.0, &sphere, None),
            Intersection::new(1.0, &sphere, None),
            Intersection::new(2.0, &sphere, None),
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, 1.0);
        assert_eq!(index, 1);
        assert!(hit);
    }

    #[test]
    fn getting_the_hit_when_all_are_neg() {
        let sphere = make_sphere();
        let ixs: Intersections = vec![
            Intersection::new(-1.0, &sphere, None),
            Intersection::new(-2.0, &sphere, None),
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, std::f32::INFINITY);
        assert_eq!(index, 0);
        assert!(!hit);
    }

    #[test]
    fn the_hit_is_always_the_lowest_positive_intersection() {
        let sphere = make_sphere();
        let ixs: Intersections = vec![
            Intersection::new(-1.0, &sphere, None),
            Intersection::new(1.0, &sphere, None),
            Intersection::new(-2.0, &sphere, None),
            Intersection::new(2.0, &sphere, None),
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, 1.0);
        assert_eq!(index, 1);
        assert!(hit);
    }

    #[test]
    fn preomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = &make_sphere();
        let i = Intersection::new(4.0, s, None);

        let c = i.computations(&r, None);

        assert_eq!(i.t, c.t);

        assert!((c.point.x).abs() <= std::f32::EPSILON);
        assert!((c.point.y).abs() <= std::f32::EPSILON);
        assert!((c.point.z - -1.0).abs() <= std::f32::EPSILON);

        assert!((c.eye.x).abs() <= std::f32::EPSILON);
        assert!((c.eye.y).abs() <= std::f32::EPSILON);
        assert!((c.eye.z - -1.0).abs() <= std::f32::EPSILON);

        assert!((c.normal.x).abs() <= std::f32::EPSILON);
        assert!((c.normal.y).abs() <= std::f32::EPSILON);
        assert!((c.normal.z - -1.0).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn hit_intersection_on_the_outside() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = &make_sphere();
        let i = Intersection::new(4.0, s, None);
        let c = i.computations(&r, None);

        assert!(!c.inside);
    }

    #[test]
    fn hit_intersection_on_the_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = &make_sphere();
        let i = Intersection::new(1.0, s, None);
        let c = i.computations(&r, None);

        assert!(c.inside);

        assert!((c.point.x).abs() <= std::f32::EPSILON);
        assert!((c.point.y).abs() <= std::f32::EPSILON);
        assert!((c.point.z - 1.0).abs() <= std::f32::EPSILON);

        assert!((c.eye.x).abs() <= std::f32::EPSILON);
        assert!((c.eye.y).abs() <= std::f32::EPSILON);
        assert!((c.eye.z - -1.0).abs() <= std::f32::EPSILON);

        assert!((c.normal.x).abs() <= std::f32::EPSILON);
        assert!((c.normal.y).abs() <= std::f32::EPSILON);
        assert!((c.normal.z - -1.0).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn over_point() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = &Object {
            geometry: Geometry::Sphere(Sphere::new(translation(0.0, 0.0, 1.0))),
            material: Material::new(),
        };
        let i = Intersection::new(5.0, s, None);
        let c = i.computations(&r, None);

        assert!(c.over_point.z < 10e-4);
        assert!(c.point.z > c.over_point.z);
    }

    #[test]
    fn under_point() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = &Object {
                geometry: Geometry::Sphere(Sphere::new(translation(0.0, 0.0, 1.0))),
                material: Material::new(),
            };
        let i = Intersection::new(
            5.0,
            s,
            None,
        );
        let c = i.computations(&r, None);

        assert!(c.under_point.z < 10e-4);
        assert!(c.point.z < c.under_point.z);
    }

    #[test]
    fn reflection_vector() {
        let plane = Object {
            geometry: Geometry::Plane(Plane::default()),
            material: Material::new(),
        };
        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 1.0, -1.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection::new(p, &plane, None);
        let c = i.computations(&r, None);

        assert_eq!(c.reflection.x, 0.0);
        assert!((c.reflection.y - p).abs() <= std::f32::EPSILON);
        assert!((c.reflection.z - p).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = {
            let s = Sphere::new(scaling(2.0, 2.0, 2.0));
            let mut s = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
            };
            s.material.refractive_index = 1.5;
            s
        };
        let b = {
            let s = Sphere::new(translation(0.0, 0.0, -0.25));
            let mut s = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
            };
            s.material.refractive_index = 2.0;
            s
        };
        let c = {
            let s = Sphere::new(translation(0.0, 0.0, 0.25));
            let mut s = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
            };
            s.material.refractive_index = 2.5;
            s
        };

        let r = Ray {
            origin: point(0.0, 0.0, -4.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection::new(2.0, &a, None),
            Intersection::new(2.75, &b, None),
            Intersection::new(3.25, &c, None),
            Intersection::new(4.75, &b, None),
            Intersection::new(5.25, &c, None),
            Intersection::new(6.0, &a, None),
        ];

        let comps = |idx: usize, n1: f32, n2: f32| {
            let c = xs[idx].computations(&r, Some(&xs));
            println!("{}\nn1: {} -- {}\nn2: {} -- {}\n", idx, c.n1, n1, c.n2, n2);
            assert!((c.n1 - n1).abs() < 10e-8);
            assert!((c.n2 - n2).abs() < 10e-8);
        };

        let cases: Vec<(usize, f32, f32)> = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];

        cases.iter().for_each(|case| {
            comps(case.0, case.1, case.2);
        });
    }

    fn glass_sphere() -> Object {
        let mut material = Material::new();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material,
        }
    }

    #[test]
    fn schlick_total_internal_reflection() {
        let shape = &glass_sphere();
        let p = 2f32.sqrt() / 2.0;
        let ray = Ray {
            origin: point(0.0, 0.0, p),
            direction: vector(0.0, 1.0, 0.0),
        };
        let xs: Intersections = vec![
            Intersection::new(-p, &shape, None),
            Intersection::new(p, &shape, None),
        ];
        let comps = xs[1].computations(&ray, Some(&xs));
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_perpendicular_angle() {
        let shape = &make_glass_sphere();
        let ray = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let xs: Intersections = vec![
            Intersection::new(-1., &shape, None),
            Intersection::new(1., &shape, None),
        ];
        let comps = xs[1].computations(&ray, Some(&xs));
        let reflectance = comps.schlick();
        println!("{}", reflectance);
        assert!((reflectance - 0.04).abs() < 10e-4);
    }

    #[test]
    fn schlick_small_angle_n2_gt_n1() {
        let shape = &make_glass_sphere();
        let ray = Ray {
            origin: point(0.0, 0.99, -2.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![Intersection::new(1.8589, &shape, None)];
        let comps = xs[0].computations(&ray, Some(&xs));
        let reflectance = comps.schlick();
        assert!((reflectance - 0.48873).abs() < 10e-4);
    }
}
