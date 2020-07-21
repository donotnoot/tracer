use super::objects::{Object};
use super::ray::Ray;

use super::tuple::{dot, Tup};
use std::sync::Arc;

pub type Intersections = Vec<Intersection>;

pub fn hit(i: &Intersections) -> (f32, usize, bool) {
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

pub struct Computations {
    pub t: f32,
    pub object: Arc<Object>,
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

#[derive(Debug)]
pub struct Intersection {
    pub t: f32,
    pub object: Arc<Object>,
}

impl Intersection {
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
            object: self.object.clone(),
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

    fn calculate_refractions(&self, xs: &Intersections) -> (f32, f32) {
        let mut n1: f32 = 1.0;
        let mut n2: f32 = 1.0;

        if xs.is_empty() {
            return (n1, n2);
        }

        // todo: what if no hit?
        let mut containers: Vec<&Arc<Object>> = vec![];

        for i in xs.iter() {
            let is_hit = std::ptr::eq(self, i);

            if is_hit {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index;
                }
            }

            if let Some(idx) = containers.iter().position(|&e| Arc::ptr_eq(e, &i.object)) {
                containers.remove(idx);
            } else {
                containers.push(&i.object);
            }

            if is_hit {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
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
    use super::*;
    use super::super::tuple::{point, vector};
    use super::super::transformations::{translation, scaling};
    use super::super::objects::{Object, Sphere, Plane};

    #[test]
    fn getting_the_hit_when_all_are_positive() {
        let ixs: Intersections = vec![
            Intersection {
                t: 1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, 1.0);
        assert_eq!(index, 0);
        assert!(hit);
    }

    #[test]
    fn getting_the_hit_when_some_pos_some_neg() {
        let ixs: Intersections = vec![
            Intersection {
                t: -1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, 1.0);
        assert_eq!(index, 1);
        assert!(hit);
    }

    #[test]
    fn getting_the_hit_when_all_are_neg() {
        let ixs: Intersections = vec![
            Intersection {
                t: -1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: -2.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, std::f32::INFINITY);
        assert_eq!(index, 0);
        assert!(!hit);
    }

    #[test]
    fn the_hit_is_always_the_lowest_positive_intersection() {
        let ixs: Intersections = vec![
            Intersection {
                t: -1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 1.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: -2.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Arc::new(Object::Sphere(Sphere::new())),
            },
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
        let s = Object::Sphere(Sphere::new());
        let i = Intersection {
            t: 4.0,
            object: Arc::new(s),
        };

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
        let s = Object::Sphere(Sphere::new());
        let i = Intersection {
            t: 4.0,
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);

        assert!(!c.inside);
    }

    #[test]
    fn hit_intersection_on_the_inside() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object::Sphere(Sphere::new());
        let i = Intersection {
            t: 1.0,
            object: Arc::new(s),
        };
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
        let mut s = Sphere::new();
        s.transform = translation(0.0, 0.0, 1.0);
        let s = Object::Sphere(s);
        let i = Intersection {
            t: 5.0,
            object: Arc::new(s),
        };
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
        let mut s = Sphere::new();
        s.transform = translation(0.0, 0.0, 1.0);
        let s = Object::Sphere(s);
        let i = Intersection {
            t: 5.0,
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);

        assert!(c.under_point.z < 10e-4);
        assert!(c.point.z < c.under_point.z);
    }

    #[test]
    fn reflection_vector() {
        let s = Object::Plane(Plane::new());
        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 1.0, -1.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: p,
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);

        assert_eq!(c.reflection.x, 0.0);
        assert!((c.reflection.y - p).abs() <= std::f32::EPSILON);
        assert!((c.reflection.z - p).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = {
            let mut sphere = Sphere::new_glass();
            sphere.transform = scaling(2.0, 2.0, 2.0);
            sphere.material.refractive_index = 1.5;
            Arc::new(Object::Sphere(sphere))
        };
        let b = {
            let mut sphere = Sphere::new_glass();
            sphere.transform = translation(0.0, 0.0, -0.25);
            sphere.material.refractive_index = 2.0;
            Arc::new(Object::Sphere(sphere))
        };
        let c = {
            let mut sphere = Sphere::new_glass();
            sphere.transform = translation(0.0, 0.0, 0.25);
            sphere.material.refractive_index = 2.5;
            Arc::new(Object::Sphere(sphere))
        };

        let r = Ray {
            origin: point(0.0, 0.0, -4.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: 2.0,
                object: Arc::clone(&a),
            },
            Intersection {
                t: 2.75,
                object: Arc::clone(&b),
            },
            Intersection {
                t: 3.25,
                object: Arc::clone(&c),
            },
            Intersection {
                t: 4.75,
                object: Arc::clone(&b),
            },
            Intersection {
                t: 5.25,
                object: Arc::clone(&c),
            },
            Intersection {
                t: 6.0,
                object: Arc::clone(&a),
            },
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
}
