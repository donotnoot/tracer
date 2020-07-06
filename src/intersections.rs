use super::objects::{Normal, Object, Sphere};
use super::ray::Ray;
use super::transformations::translation;
use super::tuple::{dot, point, vector, Tup};

pub type Intersections = Vec<Intersection>;

pub fn hit(i: &Intersections) -> (f64, usize, bool) {
    let mut min = std::f64::INFINITY;
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
    pub t: f64,
    pub object: Box<Object>,
    pub inside: bool,
    pub point: Tup,
    pub eye: Tup,
    pub normal: Tup,
    pub over_point: Tup,
}

#[derive(Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<Object>,
}

impl Intersection {
    pub fn computations(&self, r: &Ray) -> Computations {
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

        let over_point = &point + &(&normal * 10e-10);

        Computations {
            t: self.t,
            object: Box::new(*self.object.clone()),
            point,
            eye,
            normal,
            over_point,
            inside,
        }
    }
}

pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> Intersections;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getting_the_hit_when_all_are_positive() {
        let ixs: Intersections = vec![
            Intersection {
                t: 1.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Box::new(Object::Sphere(Sphere::new())),
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
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 1.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Box::new(Object::Sphere(Sphere::new())),
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
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: -2.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
        ];

        let (min, index, hit) = hit(&ixs);

        assert_eq!(min, std::f64::INFINITY);
        assert_eq!(index, 0);
        assert!(!hit);
    }

    #[test]
    fn the_hit_is_always_the_lowest_positive_intersection() {
        let ixs: Intersections = vec![
            Intersection {
                t: -1.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 1.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: -2.0,
                object: Box::new(Object::Sphere(Sphere::new())),
            },
            Intersection {
                t: 2.0,
                object: Box::new(Object::Sphere(Sphere::new())),
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
            object: Box::new(s),
        };

        let c = i.computations(&r);

        assert_eq!(i.t, c.t);

        assert!((c.point.x).abs() <= std::f64::EPSILON);
        assert!((c.point.y).abs() <= std::f64::EPSILON);
        assert!((c.point.z - -1.0).abs() <= std::f64::EPSILON);

        assert!((c.eye.x).abs() <= std::f64::EPSILON);
        assert!((c.eye.y).abs() <= std::f64::EPSILON);
        assert!((c.eye.z - -1.0).abs() <= std::f64::EPSILON);

        assert!((c.normal.x).abs() <= std::f64::EPSILON);
        assert!((c.normal.y).abs() <= std::f64::EPSILON);
        assert!((c.normal.z - -1.0).abs() <= std::f64::EPSILON);
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
            object: Box::new(s),
        };
        let c = i.computations(&r);

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
            object: Box::new(s),
        };
        let c = i.computations(&r);

        assert!(c.inside);

        assert!((c.point.x).abs() <= std::f64::EPSILON);
        assert!((c.point.y).abs() <= std::f64::EPSILON);
        assert!((c.point.z - 1.0).abs() <= std::f64::EPSILON);

        assert!((c.eye.x).abs() <= std::f64::EPSILON);
        assert!((c.eye.y).abs() <= std::f64::EPSILON);
        assert!((c.eye.z - -1.0).abs() <= std::f64::EPSILON);

        assert!((c.normal.x).abs() <= std::f64::EPSILON);
        assert!((c.normal.y).abs() <= std::f64::EPSILON);
        assert!((c.normal.z - -1.0).abs() <= std::f64::EPSILON);
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = translation(0.0, 0.0, 1.0);
        let s = Object::Sphere(s);
        let i = Intersection {
            t: 5.0,
            object: Box::new(s),
        };
        let c = i.computations(&r);

        assert!(c.over_point.z < (-std::f64::EPSILON) / 2.0);
        assert!(c.point.z > c.over_point.z);
    }
}
