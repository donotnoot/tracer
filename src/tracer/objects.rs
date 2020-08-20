use super::material::Material;
use super::matrix::{identity, Kind, Mat};
use super::patterns::Pattern;
use super::ray::Ray;
use super::tuple::{cross, dot, point, vector, Tup};

#[derive(Debug, Clone)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
    pub normal_map: Option<Pattern>,
}

#[derive(Debug, Clone)]
pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Tri(Tri),
}

impl Object {
    pub fn new(geometry: Geometry, material: Material, normal_map: Option<Pattern>) -> Self {
        Object {
            geometry,
            material,
            normal_map,
        }
    }

    pub fn normal(&self, p: &Tup, uv: Option<(f32, f32)>) -> Tup {
        let local_point = |transform_inverse: &Mat| transform_inverse * p;

        let (local_normal, transform_inverse) = match &self.geometry {
            Geometry::Sphere(o) => (
                o.normal(&local_point(&o.transform_inverse)),
                &o.transform_inverse,
            ),
            Geometry::Plane(o) => (
                o.normal(&local_point(&o.transform_inverse)),
                &o.transform_inverse,
            ),
            Geometry::Cube(o) => (
                o.normal(&local_point(&o.transform_inverse)),
                &o.transform_inverse,
            ),
            Geometry::Tri(o) => (
                match uv {
                    Some((u, v)) => o.normal(u, v),
                    None => o.normal.clone(),
                },
                &o.transform_inverse,
            ),
        };

        let mut world_normal = &transform_inverse.transpose() * &local_normal;
        world_normal.w = 0.0;

        match &self.normal_map {
            Some(pattern) => {
                let normal_perturb = (pattern.at_object_local(p) * 2.) - vector(1., 1., 1.);
                let t = (&world_normal * &vector(0., 0., 1.)).normalize();
                let b = cross(&world_normal, &t).normalize();
                let tbn = Mat::new(
                    [
                        [t.x, b.x, world_normal.x, 0.],
                        [t.y, b.y, world_normal.y, 0.],
                        [t.z, b.z, world_normal.z, 0.],
                        [0., 0., 0., 0.],
                    ],
                    Kind::General,
                );
                (&tbn * &normal_perturb).normalize()
            }
            None => world_normal,
        }
    }

    pub fn intersect(object: &Self, r: &Ray) -> (Option<f32>, Option<f32>, Option<(f32, f32)>) {
        let common = |ray: &Ray, transform_inverse: &Mat| ray.transform(&transform_inverse);

        match &object.geometry {
            Geometry::Sphere(o) => match o.intersect(&common(r, &o.transform_inverse)) {
                Some((t1, t2)) => (Some(t1), Some(t2), None),
                None => (None, None, None),
            },
            Geometry::Plane(o) => match o.intersect(&common(r, &o.transform_inverse)) {
                Some(t) => (Some(t), None, None),
                None => (None, None, None),
            },
            Geometry::Cube(o) => match o.intersect(&common(r, &o.transform_inverse)) {
                Some((t1, t2)) => (Some(t1), Some(t2), None),
                None => (None, None, None),
            },
            Geometry::Tri(o) => match o.intersect(&common(r, &o.transform_inverse)) {
                Some((t, u, v)) => (Some(t), None, Some((u, v))),
                None => (None, None, None),
            },
        }
    }

    pub fn transformation(&self) -> Mat {
        match &self.geometry {
            Geometry::Sphere(o) => o.transform.clone(),
            Geometry::Plane(o) => o.transform.clone(),
            Geometry::Cube(o) => o.transform.clone(),
            Geometry::Tri(o) => o.transform.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sphere {
    transform: Mat,
    transform_inverse: Mat,
}

impl Sphere {
    pub fn new(transform: Mat) -> Self {
        let transform_inverse = transform.inverse();
        Sphere {
            transform,
            transform_inverse,
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
        Self::new(identity())
    }
}

#[derive(Debug, Clone)]
pub struct Plane {
    transform: Mat,
    transform_inverse: Mat,
}

impl Plane {
    pub fn new(transform: Mat) -> Self {
        let transform_inverse = transform.inverse();
        Plane {
            transform,
            transform_inverse,
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
        Self::new(identity())
    }
}

#[derive(Debug, Clone)]
pub struct Cube {
    transform: Mat,
    transform_inverse: Mat,
}

impl Cube {
    pub fn new(transform: Mat) -> Self {
        let transform_inverse = transform.inverse();
        Cube {
            transform,
            transform_inverse,
        }
    }

    fn normal(&self, point: &Tup) -> Tup {
        let maxc = *[point.x.abs(), point.y.abs(), point.z.abs()]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if maxc == point.x.abs() {
            vector(point.x, 0., 0.)
        } else if maxc == point.y.abs() {
            vector(0., point.y, 0.)
        } else {
            vector(0., 0., point.z)
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let check_axis = |origin: f32, direction: f32| {
            let tmin_numerator = -1. - origin;
            let tmax_numerator = 1. - origin;

            let (mut tmin, mut tmax) = if direction.abs() >= std::f32::EPSILON {
                (tmin_numerator / direction, tmax_numerator / direction)
            } else {
                (
                    tmin_numerator * std::f32::INFINITY,
                    tmax_numerator * std::f32::INFINITY,
                )
            };

            if tmin > tmax {
                std::mem::swap(&mut tmin, &mut tmax);
            }

            (tmin, tmax)
        };

        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

        let tmin = *[xtmin, ytmin, ztmin]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&0.0);
        let tmax = *[xtmax, ytmax, ztmax]
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&0.0);

        if tmin > tmax {
            None
        } else {
            Some((tmin, tmax))
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(identity())
    }
}

#[derive(Debug, Clone)]
pub struct Tri {
    transform: Mat,
    transform_inverse: Mat,
    p1: Tup,
    p2: Tup,
    p3: Tup,
    e1: Tup,
    e2: Tup,
    normal: Tup,
    smooth_normals: Option<(Tup, Tup, Tup)>,
}

impl Tri {
    pub fn new(
        transform: Mat,
        p1: Tup,
        p2: Tup,
        p3: Tup,
        smooth_normals: Option<(Tup, Tup, Tup)>,
    ) -> Self {
        let transform_inverse = transform.inverse();
        let (e1, e2) = ((&p2 - &p1), (&p3 - &p1));
        let normal = cross(&e2, &e1).normalize();
        Tri {
            transform,
            transform_inverse,
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
            smooth_normals,
        }
    }

    fn normal(&self, u: f32, v: f32) -> Tup {
        match &self.smooth_normals {
            Some((n1, n2, n3)) => n2 * u + n3 * v + n1 * (1. - u - v),
            None => self.normal.clone(),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<(f32, f32, f32)> {
        let dir_cross_e2 = cross(&ray.direction, &self.e2);
        let determinant = dot(&self.e1, &dir_cross_e2);
        if determinant.abs() < 10e-4 {
            return None;
        }

        let f = 1. / determinant;
        let p1_to_origin = &ray.origin - &self.p1;
        let u = f * dot(&p1_to_origin, &dir_cross_e2);
        if u < 0. || u > 1. {
            return None;
        }

        let origin_cross_e1 = cross(&p1_to_origin, &self.e1);
        let v = f * dot(&ray.direction, &origin_cross_e1);
        if v < 0. || (v + u) > 1. {
            return None;
        }

        Some((f * dot(&self.e2, &origin_cross_e1), u, v))
    }
}

impl Default for Tri {
    fn default() -> Self {
        Self::new(
            identity(),
            point(0., 0., 0.),
            point(1., 0., 0.),
            point(0., 1., 0.),
            None,
        )
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
        let s = Sphere::new(scaling(2.0, 2.0, 2.0));

        let obj = Object {
            geometry: Geometry::Sphere(s),
            material: Material::new(),
            normal_map: None,
        };
        let ixs = Object::intersect(&obj, &r);

        assert_eq!(3.0, ixs.0.unwrap());
        assert_eq!(7.0, ixs.1.unwrap());
    }

    #[test]
    fn intersecting_translated_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Sphere::new(translation(5.0, 2.0, 2.0));

        let obj = Object {
            geometry: Geometry::Sphere(s),
            material: Material::new(),
            normal_map: None,
        };
        let ixs = Object::intersect(&obj, &r);

        assert_eq!(ixs, (None, None, None));
    }

    #[test]
    fn sphere_normals() {
        {
            // x axis
            let s = Sphere::default();
            let normal = s.normal(&point(1.0, 0.0, 0.0));
            assert!(normal.cmp_epsilon(1.0, 0.0, 0.0, 0.0));
        }
        {
            // y axis
            let s = Sphere::default();
            let normal = s.normal(&point(0.0, 1.0, 0.0));
            assert!(normal.cmp_epsilon(0.0, 1.0, 0.0, 0.0));
        }
        {
            // z axis
            let s = Sphere::default();
            let normal = s.normal(&point(0.0, 0.0, 1.0));
            assert!(normal.cmp_epsilon(0.0, 0.0, 1.0, 0.0));
        }
        {
            // non axis
            let s = Sphere::default();
            let p = 3_f32.sqrt() / 3.0;
            let normal = s.normal(&point(p, p, p));
            assert!(normal.cmp_epsilon(p, p, p, 0.0));
        }
        {
            // translated
            let s = Sphere::new(translation(0.0, 1.0, 0.0));

            let obj = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
                normal_map: None,
            };
            let normal = obj.normal(&point(0.0, 1.70711, -0.70711), None);

            assert_eq!(0.0, normal.x);
            assert!((normal.y - 0.70711).abs() < 10e-5);
            assert!((normal.z - -0.70711).abs() < 10e-5);
        }
        {
            // transformed
            let s = Sphere::new(scaling(1.0, 0.5, 1.0) * rotate_z(std::f32::consts::PI / 5.0));

            let p = 2.0_f32.sqrt() / 2.0;
            let obj = Object {
                geometry: Geometry::Sphere(s),
                material: Material::new(),
                normal_map: None,
            };
            let normal = obj.normal(&point(0.0, p, -p), None);

            assert!(normal.x.abs() < 10e-5);
            assert!((normal.y - 0.97014).abs() < 10e-5);
            assert!((normal.z - -0.24254).abs() < 10e-5);
        }
    }

    #[test]
    fn sphere_normals_should_be_normalised() {
        let s = Sphere::default();
        let p = 3_f32.sqrt() / 3.0;
        let normal = s.normal(&point(p, p, p));
        let normalized = normal.normalize();
        assert!(normal.cmp_epsilon(normalized.x, normalized.y, normalized.z, 0.0));
    }

    #[test]
    fn normal_of_a_plane_is_constant() {
        let p = Plane::default();
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
            let p = Plane::default();
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
            let p = Plane::default();
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
            let p = Plane::default();
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
            let p = Plane::default();
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

    #[test]
    fn ray_intersects_cube() {
        let cube = Cube::default();

        let test = |name: String, ray: Ray, expected: Option<(f32, f32)>| {
            let result = cube.intersect(&ray);
            println!("{:?} {:?} {:?}", name, result, expected);
            assert_eq!(result, expected);
        };

        let cases = vec![
            (
                "+x",
                Ray {
                    origin: point(5.0, 0.5, 0.0),
                    direction: vector(-1.0, 0.0, 0.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "-x",
                Ray {
                    origin: point(-5.0, 0.5, 0.0),
                    direction: vector(1.0, 0.0, 0.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "+y",
                Ray {
                    origin: point(0.5, 5.0, 0.0),
                    direction: vector(0.0, -1.0, 0.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "-y",
                Ray {
                    origin: point(0.5, -5.0, 0.0),
                    direction: vector(0.0, 1.0, 0.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "+z",
                Ray {
                    origin: point(0.5, 0.0, 5.0),
                    direction: vector(0.0, 0.0, -1.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "-z",
                Ray {
                    origin: point(0.5, 0.0, -5.0),
                    direction: vector(0.0, 0.0, 1.0),
                },
                Some((4f32, 6f32)),
            ),
            (
                "inside",
                Ray {
                    origin: point(0.0, 0.5, 0.0),
                    direction: vector(0.0, 0.0, 1.0),
                },
                Some((-1f32, 1f32)),
            ),
        ];

        cases.into_iter().for_each(|(name, ray, expected)| {
            test(name.to_string(), ray, expected);
        });
    }

    #[test]
    fn ray_misses_cube() {
        let cube = Cube::default();

        let test = |ray: Ray| {
            let result = cube.intersect(&ray);
            println!("{:?}", result);
            assert_eq!(result, None);
        };

        vec![
            Ray {
                origin: point(2., 0., 2.),
                direction: vector(0., 0., -1.),
            },
            Ray {
                origin: point(0., 2., 2.),
                direction: vector(0., -1., 0.),
            },
            Ray {
                origin: point(2., 2., 0.),
                direction: vector(-1., 0., 0.),
            },
        ]
        .into_iter()
        .for_each(|ray| {
            test(ray);
        });
    }

    #[test]
    fn cube_normal() {
        let cube = Cube::default();

        let test = |point: Tup, expected_normal: Tup| {
            let result = cube.normal(&point);
            assert_eq!(result, expected_normal)
        };

        vec![
            (point(1., 0.5, -0.8), vector(1., 0., 0.)),
            (point(-1., -0.2, 0.9), vector(-1., 0., 0.)),
            (point(0.4, 1., 0.6), vector(0., 1., 0.)),
            (point(-0.5, -1., 0.7), vector(0., -1., 0.)),
            (point(0.2, 0.8, 1.), vector(0., 0., 1.)),
            (point(-0.7, 0.4, -1.), vector(0., 0., -1.)),
            (point(1., 1., 1.), vector(1., 0., 0.)),
            (point(-1., -1., -1.), vector(-1., 0., 0.)),
        ]
        .into_iter()
        .for_each(|elem| {
            test(elem.0, elem.1);
        });
    }

    #[test]
    fn tri_constructor_precalculations() {
        let tri = Tri::new(
            identity(),
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
            None,
        );

        assert_eq!(tri.e1, vector(-1., -1., 0.));
        assert_eq!(tri.e2, vector(1., -1., 0.));
        assert_eq!(tri.normal, vector(0., 0., -1.));
    }

    #[test]
    fn tri_returns_precomputed_normal() {
        let tri = Tri::default();

        assert_eq!(tri.normal, tri.normal(0., 0.));
    }

    #[test]
    fn tri_none_with_parallel_ray() {
        let tri = Tri::new(
            identity(),
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
            None,
        );
        let ray = Ray {
            origin: point(0., -1., -2.),
            direction: vector(0., 1., 0.),
        };

        assert_eq!(tri.intersect(&ray), None);
    }

    #[test]
    fn tri_none_with_edge_misses() {
        let tri = Tri::new(
            identity(),
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
            None,
        );

        // p1-p3 edge
        let ray = Ray {
            origin: point(1., 1., -2.),
            direction: vector(0., 0., 1.),
        };
        assert_eq!(tri.intersect(&ray), None);

        // p1-p2 edge
        let ray = Ray {
            origin: point(-1., 1., -2.),
            direction: vector(0., 0., 1.),
        };
        assert_eq!(tri.intersect(&ray), None);

        // p2-p3 edge
        let ray = Ray {
            origin: point(0., -1., -2.),
            direction: vector(0., 0., 1.),
        };
        assert_eq!(tri.intersect(&ray), None);
    }

    #[test]
    fn tri_intersection() {
        let tri = Tri::new(
            identity(),
            point(0., 1., 0.),
            point(-1., 0., 0.),
            point(1., 0., 0.),
            None,
        );
        let ray = Ray {
            origin: point(0., 0.5, -2.),
            direction: vector(0., 0., 1.),
        };

        assert_eq!(tri.intersect(&ray), Some((2., 0.25, 0.25)));
    }
}
