use super::intersections::{hit, Computations, Intersect, Intersection, Intersections};
use super::light::PointLight;
use super::objects::{Object, Plane, Sphere};
use super::ray::Ray;
use super::transformations::{scaling, translation};
use super::tuple::{color, point, vector, Tup};

pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
    pub background_color: Tup,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![
                {
                    let mut s = Sphere::new();
                    s.material.color = color(0.8, 1.0, 0.6);
                    s.material.diffuse = 0.7;
                    s.material.specular = 0.2;
                    Object::Sphere(s)
                },
                {
                    let mut s = Sphere::new();
                    s.transform = scaling(0.5, 0.5, 0.5);
                    Object::Sphere(s)
                },
            ],
            light: PointLight {
                position: point(-10.0, 10.0, -10.0),
                intensity: vector(1.0, 1.0, 1.0),
            },
            background_color: color(0.0, 0.0, 0.0),
        }
    }

    fn intersect(&self, r: &Ray) -> Intersections {
        let mut i: Intersections = vec![];

        for (_, elem) in self.objects.iter().enumerate() {
            i.append(&mut (*elem).intersect(r));
        }

        i.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        i
    }

    fn is_shadowed(&self, p: &Tup) -> bool {
        let v = &(self.light.position) - &p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: p.clone(),
            direction,
        };

        match hit(&self.intersect(&ray)) {
            (hit, _, true) => return hit < distance,
            _ => return false,
        }
    }

    pub fn color_at(&self, r: &Ray, depth_remaining: u64) -> Tup {
        let intersections = self.intersect(r);

        match hit(&intersections) {
            (_, _, false) => return self.background_color.clone(),
            (_, i, true) => {
                return self.shade_hit(&intersections[i].computations(&r), depth_remaining)
            }
        }
    }

    fn shade_hit(&self, c: &Computations, depth_remaining: u64) -> Tup {
        let s = self.is_shadowed(&c.over_point);

        let surface = c.object.material().lighting(
            &(*c.object),
            &self.light,
            c.point.clone(),
            c.eye.clone(),
            c.normal.clone(),
            s,
        );

        let reflected = self.reflected_color(&c, depth_remaining);

        &surface + &reflected
    }

    fn reflected_color(&self, c: &Computations, depth_remaining: u64) -> Tup {
        if depth_remaining < 1 {
            return color(0.0, 0.0, 0.0);
        }
        if (*c.object).material().reflectiveness < std::f64::EPSILON {
            return color(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray {
            origin: c.over_point.clone(),
            direction: c.reflection.clone(),
        };
        let color = self.color_at(&reflect_ray, depth_remaining - 1);

        &color * (*c.object).material().reflectiveness
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersecting_world_with_ray() {
        let w = World::new();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let ixs = w.intersect(&r);

        assert_eq!(ixs.len(), 4);
        assert_eq!(ixs[0].t, 4.0);
        assert_eq!(ixs[1].t, 4.5);
        assert_eq!(ixs[2].t, 5.5);
        assert_eq!(ixs[3].t, 6.0);
    }

    #[test]
    fn reflection_of_non_reflective_material() {
        let mut w = World::new();
        w.objects[1].material().ambient = 1.0;
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            t: 1.0,
            object: Box::new(w.objects[1].clone()),
        };
        let c = i.computations(&r);
        let color = w.reflected_color(&c, 10);

        assert!((color.x).abs() <= std::f64::EPSILON);
        assert!((color.y).abs() <= std::f64::EPSILON);
        assert!((color.z).abs() <= std::f64::EPSILON);
    }

    #[test]
    fn reflection_color_of_reflective_material() {
        let mut w = World::new();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f64.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f64.sqrt(),
            object: Box::new(s),
        };
        let c = i.computations(&r);
        let color = w.reflected_color(&c, 10);

        println!("{}", color);
        assert!((color.x - 0.19032).abs() < 10e-3);
        assert!((color.y - 0.2379).abs() < 10e-3);
        assert!((color.z - 0.14274).abs() < 10e-3);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::new();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f64.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f64.sqrt(),
            object: Box::new(s),
        };
        let c = i.computations(&r);
        let color = w.shade_hit(&c, 10);

        println!("{}", color);
        assert!((color.x - 0.87677).abs() < 10e-3);
        assert!((color.y - 0.92436).abs() < 10e-3);
        assert!((color.z - 0.82918).abs() < 10e-3);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::new();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f64.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -2.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f64.sqrt(),
            object: Box::new(s),
        };
        let c = i.computations(&r);
        let color = w.reflected_color(&c, 0);

        assert_eq!(color.x, 0.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }
}
