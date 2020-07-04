use super::intersections::{Intersect, Intersection, Intersections};
use super::material;
use super::material::HasMaterial;
use super::matrix::{identity, Mat};
use super::ray::Ray;
use super::tuple::{dot, point, Tup};

#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
}

impl Normal for Object {
    fn normal(&self, p: &Tup) -> Tup {
        match self {
            Object::Sphere(s) => s.normal(p),
        }
    }
}

impl Intersect for Object {
    fn intersect(&self, r: &Ray) -> Intersections {
        match self {
            Object::Sphere(s) => s.intersect(r),
        }
    }
}

impl HasMaterial for Object {
    fn material(&self) -> material::Material {
        match self {
            Object::Sphere(s) => s.material(),
        }
    }
}

pub trait Normal {
    fn normal(&self, p: &Tup) -> Tup;
}

#[derive(Clone)]
pub struct Sphere {
    pub transform: Mat,
    pub material: material::Material,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: identity(4),
            material: material::Material::new(),
        }
    }
}

impl HasMaterial for Sphere {
    fn material(&self) -> material::Material {
        self.material.clone()
    }
}

impl Normal for Sphere {
    fn normal(&self, p: &Tup) -> Tup {
        let transform_inverse = self.transform.inverse();
        let object_point = &transform_inverse * p;
        let object_normal = &object_point - &point(0.0, 0.0, 0.0);
        let mut world_normal = &transform_inverse.transpose() * &object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let transformed = ray.transform(&self.transform);
        let sphere_to_ray = &transformed.origin - &point(0.0, 0.0, 0.0);

        let a = dot(&transformed.direction, &transformed.direction);
        let b = 2.0 * dot(&transformed.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            let v: Vec<Intersection> = vec![];
            return v;
        }

        let mut t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let mut t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        let v: Vec<Intersection> = vec![
            Intersection {
                t: t1,
                object: Box::new(Object::Sphere(self.clone())),
            },
            Intersection {
                t: t2,
                object: Box::new(Object::Sphere(self.clone())),
            },
        ];
        v
    }
}
