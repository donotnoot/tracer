use super::objects::{Normal, Object};
use super::ray::Ray;
use super::tuple::{dot, Tup};

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

pub struct Intersection {
    pub t: f64,
    pub object: Box<Object>,
}

impl Intersection {
    pub fn computations(&self, r: &Ray) -> Computations {
        let point = r.position(self.t);
        let eye = -&r.direction;
        let mut normal = self.object.normal(&point);
        let over_point = &point + &(&normal * 10e-10 as f64);

        let inside: bool = {
            if dot(&normal, &eye) < 0.0 {
                normal = -&normal;
                true
            } else {
                false
            }
        };

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
