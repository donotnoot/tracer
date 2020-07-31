use super::intersections::{Intersection, Intersections};
use super::material::Material;
use super::matrix::{identity, Mat};
use super::ray::Ray;

use super::tuple::{dot, point, vector, Tup};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Object {
    pub fn normal(&self, p: &Tup) -> Tup {
        let local_point = |transform: &Mat| &transform.inverse() * p;

        let (local_normal, shape_transform) = match &self.geometry {
            Geometry::Sphere(o) => (o.normal(&local_point(&o.transform)), &o.transform),
            Geometry::Plane(o) => (o.normal(&local_point(&o.transform)), &o.transform),
        };

        let mut world_normal = &shape_transform.inverse().transpose() * &local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }

    pub fn intersect(object: &Self, r: &Ray) -> Intersections {
        let common = |ray: &Ray, transform: &Mat| ray.transform(&transform.inverse());

        let mut v: Vec<Intersection> = Vec::with_capacity(2);

        let ref_clone = Rc::new(object.clone());

        match &object.geometry {
            Geometry::Sphere(o) => {
                if let Some((t1, t2)) = o.intersect(&common(r, &o.transform)) {
                    v.push(Intersection {
                        t: t1,
                        object: Rc::clone(&ref_clone),
                    });
                    v.push(Intersection {
                        t: t2,
                        object: Rc::clone(&ref_clone),
                    });
                }
            }
            Geometry::Plane(o) => {
                if let Some(t1) = o.intersect(&common(r, &o.transform)) {
                    v.push(Intersection {
                        t: t1,
                        object: Rc::clone(&ref_clone),
                    });
                }
            }
        };

        v
    }

    pub fn transformation(&self) -> Mat {
        match &self.geometry {
            Geometry::Sphere(o) => o.transform.clone(),
            Geometry::Plane(o) => o.transform.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub transform: Mat,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: identity(4),
        }
    }

    fn normal(&self, p: &Tup) -> Tup {
        p - &point(0.0, 0.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let sphere_to_ray = &ray.origin - &point(0.0, 0.0, 0.0);

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let mut t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t1 > t2 {
            std::mem::swap(&mut t2, &mut t1);
        }

        Some((t1, t2))
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub transform: Mat,
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            transform: identity(4),
        }
    }

    fn normal(&self, _: &Tup) -> Tup {
        vector(0.0, 1.0, 0.0)
    }

    fn intersect(&self, ray: &Ray) -> Option<f32> {
        if ray.direction.y.abs() < 10e-5 {
            None
        } else {
            Some((-ray.origin.y) / ray.direction.y)
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
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

        let obj = Object {
            geometry: Geometry::Sphere(s),
            material: Material::new(),
        };
        let ixs = Object::intersect(&obj, &r);

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

        let obj = Object {
            geometry: Geometry::Sphere(s),
            material: Material::new(),
        };
        let ixs = Object::intersect(&obj, &r);

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

            let obj = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
            };
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
            let obj = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
            };
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

            assert!(match xs {
                None => true,
                _ => false,
            });
        }
        {
            // coplanar
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, 0.0, 0.0),
                direction: vector(0.0, 0.0, 1.0),
            };
            let xs = p.intersect(&r);

            assert!(match xs {
                None => true,
                _ => false,
            });
        }
        {
            // from above
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, 1.0, 0.0),
                direction: vector(0.0, -1.0, 0.0),
            };
            let xs = p.intersect(&r);

            assert!(match xs {
                Some(1.0) => true,
                _ => false,
            });
        }
        {
            // from below
            let p = Plane::new();
            let r = Ray {
                origin: point(0.0, -1.0, 0.0),
                direction: vector(0.0, 1.0, 0.0),
            };
            let xs = p.intersect(&r);

            assert!(match xs {
                Some(1.0) => true,
                _ => false,
            });
        }
    }
}
