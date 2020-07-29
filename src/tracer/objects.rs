use super::intersections::{Intersect, Intersection, Intersections};
use super::material;
use super::material::HasMaterial;
use super::matrix::{identity, Mat};
use super::ray::Ray;

use super::tuple::{dot, point, vector, Tup};

use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn normal(&self, p: &Tup) -> Tup {
        let local_point = |transform: &Mat| &transform.inverse() * p;

        let (local_normal, shape_transform) = match self {
            Object::Sphere(o) => (o.normal(&local_point(&o.transform)), &o.transform),
            Object::Plane(o) => (o.normal(&local_point(&o.transform)), &o.transform),
        };

        let mut world_normal = &shape_transform.inverse().transpose() * &local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn material(&self) -> Material {
        match self {
            Object::Sphere(o) => o.material(),
            Object::Plane(o) => o.material(),
        }
    }

    // returns the transparency of the material
    pub fn material_transparency(&self) -> f32 {
    pub fn intersect(&self, r: &Ray) -> Intersections {
        let common = |ray: &Ray, transform: &Mat| ray.transform(&transform.inverse());

        match self {
            Object::Sphere(o) => o.intersect(&common(r, &o.transform)),
            Object::Plane(o) => o.intersect(&common(r, &o.transform)),
            Object::Sphere(o) => o.material.transparency,
            Object::Plane(o) => o.material.transparency,
        }
    }

    }

    pub fn transformation(&self) -> Mat {
        match self {
            Object::Sphere(o) => o.transform.clone(),
            Object::Plane(o) => o.transform.clone(),
        }
    }
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

    /// Creates a new sphere with a material that resembles glass.
    pub fn new_glass() -> Self {
        Sphere {
            transform: identity(4),
            material: {
                let mut m = material::Material::new();
                m.transparency = 1.0;
                m.refractive_index = 1.5;
                m
            },
        }
    }

    fn material(&self) -> material::Material {
        self.material.clone()
    }

    fn normal(&self, p: &Tup) -> Tup {
        p - &point(0.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = &ray.origin - &point(0.0, 0.0, 0.0);

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
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

        let arc = Arc::new(Object::Sphere(self.clone()));

        let v: Vec<Intersection> = vec![
            Intersection {
                t: t1,
                object: Arc::clone(&arc),
            },
            Intersection {
                t: t2,
                object: Arc::clone(&arc),
            },
        ];
        v
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub transform: Mat,
    pub material: material::Material,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: identity(4),
            material: material::Material::new(),
        }
    }

    fn material(&self) -> material::Material {
        self.material.clone()
    }

    fn normal(&self, _: &Tup) -> Tup {
        vector(0.0, 1.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Intersections {
        if ray.direction.y.abs() < 10e-5 {
            let v: Vec<Intersection> = vec![];
            v
        } else {
            vec![Intersection {
                object: Arc::new(Object::Plane(self.clone())),
                t: (-ray.origin.y) / ray.direction.y,
            }]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::transformations::{rotate_z, scaling, translation};
    use super::*;

    #[test]
    fn intersecting_scaled_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let mut s = Sphere::new();
        s.transform = scaling(2.0, 2.0, 2.0);

        let obj = Object::Sphere(s);
        let ixs = obj.intersect(&r);

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

        let obj = Object::Sphere(s);
        let ixs = obj.intersect(&r);

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
            let p = 3_f32.sqrt() / 3.0;
            let normal = s.normal(&point(p, p, p));
            assert!(normal.cmp_epsilon(p, p, p, 0.0));
        }
        {
            // translated
            let mut s = Sphere::new();
            s.transform = translation(0.0, 1.0, 0.0);

            let obj = Object::Sphere(s);
            let normal = obj.normal(&point(0.0, 1.70711, -0.70711));

            assert_eq!(0.0, normal.x);
            assert!((normal.y - 0.70711).abs() < 10e-5);
            assert!((normal.z - -0.70711).abs() < 10e-5);
        }
        {
            // transformed
            let mut s = Sphere::new();
            s.transform = scaling(1.0, 0.5, 1.0) * rotate_z(std::f32::consts::PI / 5.0);

            let p = 2.0_f32.sqrt() / 2.0;
            let obj = Object::Sphere(s);
            let normal = obj.normal(&point(0.0, p, -p));

            assert!(normal.x.abs() < 10e-5);
            assert!((normal.y - 0.97014).abs() < 10e-5);
            assert!((normal.z - -0.24254).abs() < 10e-5);
        }
    }

    #[test]
    fn sphere_normals_should_be_normalised() {
        let s = Sphere::new();
        let p = 3_f32.sqrt() / 3.0;
        let normal = s.normal(&point(p, p, p));
        let normalized = normal.normalize();
        assert!(normal.cmp_epsilon(normalized.x, normalized.y, normalized.z, 0.0));
    }

    #[test]
    fn normal_of_a_plane_is_constant() {
        let p = Plane::new();
        let n1 = p.normal(&point(0.0, 0.0, 0.0));
        let n2 = p.normal(&point(100.0, 0.0, 0.0));
        let n3 = p.normal(&point(0.0, 1000.0, 20.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn plane_intersections() {
        {
            // parallel to the plane
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, 10.0, 0.0),
                direction: vector(0.0, 0.0, 1.0),
            };
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
        {
            // coplanar
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(0.0, 0.0, 1.0),
            };
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 0);
        }
        {
            // from above
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, 1.0, 0.0),
                direction: vector(0.0, -1.0, 0.0),
            };
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 1.0);
        }
        {
            // from below
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, -1.0, 0.0),
                direction: vector(0.0, 1.0, 0.0),
            };
            let xs = p.intersect(&r);
            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 1.0);
        }
    }
}
