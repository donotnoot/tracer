use super::matrix::Mat;

use super::tuple::Tup;

#[derive(Debug)]
pub struct Ray {
    pub origin: Tup,
    pub direction: Tup,
}

impl Ray {
    pub fn position(&self, t: f32) -> Tup {
        &self.origin + &(&self.direction * t)
    }

    pub fn transform(&self, m: &Mat) -> Ray {
        Ray {
            origin: m * &self.origin,
            direction: m * &self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::material::Material;
    use super::super::objects::{Geometry, Object, Sphere};
    use super::super::transformations::{scaling, translation};
    use super::super::tuple::{point, vector};
    use super::*;

    #[test]
    fn computing_point_from_distance() {
        let r = Ray {
            origin: point(2.0, 3.0, 4.0),
            direction: vector(1.0, 0.0, 0.0),
        };
        assert!(r.position(0.0).cmp_epsilon(2.0, 3.0, 4.0, 1.0));
        assert!(r.position(1.0).cmp_epsilon(3.0, 3.0, 4.0, 1.0));
        assert!(r.position(-1.0).cmp_epsilon(1.0, 3.0, 4.0, 1.0));
        assert!(r.position(2.5).cmp_epsilon(4.5, 3.0, 4.0, 1.0));
    }

    #[test]
    fn ray_intersects_sphere_two_points() {
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        };
        let ixs = Object::intersect(&s, &r);

        assert_eq!(ixs.0.unwrap(), 4.0);
        assert_eq!(ixs.1.unwrap(), 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray {
            origin: point(0.0, 1.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        };
        let ixs = Object::intersect(&s, &r);

        assert_eq!(ixs.0.unwrap(), 5.0);
        assert_eq!(ixs.1.unwrap(), 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray {
            origin: point(0.0, 2.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        };
        let ixs = Object::intersect(&s, &r);

        assert_eq!(ixs, (None, None, None));
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        };
        let ixs = Object::intersect(&s, &r);

        assert_eq!(ixs.0.unwrap(), -1.0);
        assert_eq!(ixs.1.unwrap(), 1.0);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray {
            origin: point(0.0, 0.0, 5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let s = Object {
            geometry: Geometry::Sphere(Sphere::default()),
            material: Material::new(),
        };
        let ixs = Object::intersect(&s, &r);

        assert_eq!(ixs.0.unwrap(), -6.0);
        assert_eq!(ixs.1.unwrap(), -4.0);
    }

    // skipped interset sets the object on the intersesectino

    #[test]
    fn translating_a_ray() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let r = r.transform(&translation(3.0, 4.0, 5.0));

        assert!(r.origin.cmp_epsilon(4.0, 6.0, 8.0, 1.0));
        assert!(r.direction.cmp_epsilon(0.0, 1.0, 0.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray {
            origin: point(1.0, 2.0, 3.0),
            direction: vector(0.0, 1.0, 0.0),
        };
        let r = r.transform(&scaling(2.0, 3.0, 4.0));

        assert!(r.origin.cmp_epsilon(2.0, 6.0, 12.0, 1.0));
        assert!(r.direction.cmp_epsilon(0.0, 3.0, 0.0, 0.0));
    }
}
