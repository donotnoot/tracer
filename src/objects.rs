use super::intersections::{Intersect, Intersection, Intersections};
use super::material;
use super::material::{HasMaterial, Material};
use super::matrix::{identity, Mat};
use super::ray::Ray;
use super::transformations::{rotate_z, scaling, translation};
use super::tuple::{dot, point, vector, Tup};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        let transformed = ray.transform(&self.transform.inverse());
        let sphere_to_ray = &transformed.origin - &point(0.0, 0.0, 0.0);

        let a = dot(&transformed.direction, &transformed.direction);
        let b = 2.0 * dot(&transformed.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            let v: Vec<Intersection> = vec![];
            return v;
        }

        let mut t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let mut t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            std::mem::swap(&mut t2, &mut t1);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersecting_scaled_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = scaling(2.0, 2.0, 2.0);
        let ixs = s.intersect(&r);

        assert_eq!(3.0, ixs[0].t);
        assert_eq!(7.0, ixs[1].t);
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = translation(5.0, 2.0, 2.0);
        let ixs = s.intersect(&r);

        assert_eq!(ixs.len(), 0);
    }

    #[test]
    fn sphere_normals() {
        {
            // x axis
            let s = Sphere::new();
            let normal = s.normal(&point(1.0, 0.0, 0.0));
            assert!(normal.cmp_epsilon(1.0, 0.0, 0.0, 0.0));
        }
        {
            // y axis
            let s = Sphere::new();
            let normal = s.normal(&point(0.0, 1.0, 0.0));
            assert!(normal.cmp_epsilon(0.0, 1.0, 0.0, 0.0));
        }
        {
            // z axis
            let s = Sphere::new();
            let normal = s.normal(&point(0.0, 0.0, 1.0));
            assert!(normal.cmp_epsilon(0.0, 0.0, 1.0, 0.0));
        }
        {
            // non axis
            let s = Sphere::new();
            let p = 3_f64.sqrt() / 3.0;
            let normal = s.normal(&point(p, p, p));
            assert!(normal.cmp_epsilon(p, p, p, 0.0));
        }
        {
            // translated
            let mut s = Sphere::new();
            s.transform = translation(0.0, 1.0, 0.0);

            let normal = s.normal(&point(0.0, 1.70711, -0.70711));

            assert_eq!(0.0, normal.x);
            assert!((normal.y - 0.70711).abs() < 10e-5);
            assert!((normal.z - -0.70711).abs() < 10e-5);
        }
        {
            // transformed
            let mut s = Sphere::new();
            s.transform = &scaling(1.0, 0.5, 1.0) * &rotate_z(std::f64::consts::PI / 5.0);

            let p = 2.0_f64.sqrt() / 2.0;
            let normal = s.normal(&point(0.0, p, -p));

            assert_eq!(0.0, normal.x);
            assert!((normal.y - 0.97014).abs() < 10e-5);
            assert!((normal.z - -0.24254).abs() < 10e-5);
        }
    }

    #[test]
    fn sphere_normals_should_be_normalised() {
        let s = Sphere::new();
        let p = 3_f64.sqrt() / 3.0;
        let normal = s.normal(&point(p, p, p));
        let normalized = normal.normalize();
        assert!(normal.cmp_epsilon(normalized.x, normalized.y, normalized.z, 0.0));
    }
}
