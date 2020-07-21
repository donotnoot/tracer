use super::intersections::{hit, Computations, Intersect, Intersections};
use super::light::PointLight;
use super::objects::{Object, Sphere};
use super::ray::Ray;
use super::transformations::scaling;
use super::tuple::{color, dot, point, vector, Tup};

pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
    pub background_color: Tup,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: PointLight {
                position: point(-10.0, 10.0, -10.0),
                intensity: vector(1.0, 1.0, 1.0),
            },
            background_color: color(0.0, 0.0, 0.0),
        }
    }

    pub fn new_with_stuff() -> Self {
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
        let v = &(self.light.position) - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: p.clone(),
            direction,
        };

        match hit(&self.intersect(&ray)) {
            (hit, _, true) => hit < distance,
            _ => false,
        }
    }

    pub fn color_at(&self, r: &Ray, depth_remaining: u64) -> Tup {
        let intersections = self.intersect(r);

        match hit(&intersections) {
            (_, _, false) => self.background_color.clone(),
            (_, i, true) => {
                self.shade_hit(
                    &intersections[i].computations(&r, Some(&intersections)),
                    depth_remaining,
                )
            }
        }
    }

    fn shade_hit(&self, c: &Computations, depth_remaining: u64) -> Tup {
        let s = self.is_shadowed(&c.over_point);

        let surface = c.object.material().lighting(
            &(*c.object),
            &self.light,
            c.over_point.clone(),
            c.eye.clone(),
            c.normal.clone(),
            s,
        );

        let reflected = self.reflected_color(&c, depth_remaining);

        &surface + &reflected
    }

    fn reflected_color(&self, c: &Computations, depth_remaining: u64) -> Tup {
        if depth_remaining == 0 {
            return color(0.0, 0.0, 0.0);
        }
        if (*c.object).material().reflectiveness < std::f32::EPSILON {
            return color(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray {
            origin: c.over_point.clone(),
            direction: c.reflection.clone(),
        };
        let color = self.color_at(&reflect_ray, depth_remaining - 1);

        &color * (*c.object).material().reflectiveness
    }

    fn refracted_color(&self, c: &Computations, depth_remaining: u64) -> Tup {
        if depth_remaining == 0 {
            return color(0.0, 0.0, 0.0);
        }
        if c.object.material().transparency == 0.0 {
            println!("wtd");
            return color(0.0, 0.0, 0.0);
        }

        let n_ratio = c.n1 / c.n2;
        let cos_i = dot(&c.eye, &c.normal);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            // total internal reflection
            return color(0.0, 0.0, 0.0);
        }

        color(1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::super::intersections::Intersection;
    use super::super::objects::Plane;
    use super::super::transformations::translation;
    use super::*;
    use std::sync::Arc;

    #[test]
    fn intersecting_world_with_ray() {
        let w = World::new_with_stuff();
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
        let mut w = World::new_with_stuff();
        w.objects[1].material().ambient = 1.0;
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            t: 1.0,
            object: Arc::new(w.objects[1].clone()),
        };
        let c = i.computations(&r, None);
        let color = w.reflected_color(&c, 10);

        assert!((color.x).abs() <= std::f32::EPSILON);
        assert!((color.y).abs() <= std::f32::EPSILON);
        assert!((color.z).abs() <= std::f32::EPSILON);
    }

    #[test]
    fn reflection_color_of_reflective_material() {
        let mut w = World::new_with_stuff();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);
        let color = w.reflected_color(&c, 10);

        println!("{}", color);
        assert!((color.x - 0.19032).abs() < 10e-3);
        assert!((color.y - 0.2379).abs() < 10e-3);
        assert!((color.z - 0.14274).abs() < 10e-3);
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::new_with_stuff();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);
        let color = w.shade_hit(&c, 10);

        println!("{}", color);
        assert!((color.x - 0.87677).abs() < 10e-3);
        assert!((color.y - 0.92436).abs() < 10e-3);
        assert!((color.z - 0.82918).abs() < 10e-3);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::new_with_stuff();

        let mut p = Plane::new();
        p.material.reflectiveness = 0.5;
        p.transform = translation(0.0, -1.0, 0.0);
        let s = Object::Plane(p);
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -2.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: Arc::new(s),
        };
        let c = i.computations(&r, None);
        let color = w.reflected_color(&c, 0);

        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn the_refracted_color_with_opaque_surface() {
        let w = World::new_with_stuff();
        let shape = Arc::new(w.objects[0].clone());
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: 4.0,
                object: Arc::clone(&shape),
            },
            Intersection {
                t: 6.0,
                object: Arc::clone(&shape),
            },
        ];
        let comps = xs[0].computations(&r, Some(&xs));

        let color = w.refracted_color(&comps, 5);

        assert_eq!(color.x, 0.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn refracted_color_at_max_recursive_depth() {
        let w = World::new_with_stuff();
        let shape = Arc::new(w.objects[0].clone());
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: 4.0,
                object: Arc::clone(&shape),
            },
            Intersection {
                t: 6.0,
                object: Arc::clone(&shape),
            },
        ];
        let comps = xs[0].computations(&r, Some(&xs));

        let color = w.refracted_color(&comps, 0);

        assert_eq!(color.x, 0.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn refracted_color_total_internal_reflection() {
        let mut w = World::new_with_stuff();
        w.objects[0] = {
            let mut o = Sphere::new();
            o.material.transparency = 1.0;
            o.material.refractive_index = 1.5;
            Object::Sphere(o)
        };

        let shape = Arc::new(w.objects[0].clone());
        let p = 2f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, p),
            direction: vector(0.0, 1.0, 0.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: -p,
                object: Arc::clone(&shape),
            },
            Intersection {
                t: p,
                object: Arc::clone(&shape),
            },
        ];
        let comps = xs[1].computations(&r, Some(&xs));

        let color = w.refracted_color(&comps, 5);

        assert_eq!(color.x, 0.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }
}
