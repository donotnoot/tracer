use super::intersections::{hit, Computations, Intersection, Intersections};
use super::light::*;
use super::material::Material;
use super::objects::{Geometry, Object, Sphere};
use super::ray::Ray;
use super::transformations::scaling;
use super::tuple::{color, dot, point, vector, Tup};
use rand::Rng;

#[derive(Debug)]
pub struct World {
    pub objects: Vec<Object>,
    pub light: Light,
    pub background_color: Tup,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: Light {
                position: point(-10.0, 10.0, -10.0),
                intensity: vector(1.0, 1.0, 1.0),
                kind: LightKind::Point,
            },
            background_color: color(0.0, 0.0, 0.0),
        }
    }

    pub fn new_with_stuff() -> Self {
        World {
            objects: vec![
                {
                    let mut object = Object {
                        geometry: Geometry::Sphere(Sphere::new()),
                        material: Material::new(),
                    };
                    object.material.color = color(0.8, 1.0, 0.6);
                    object.material.diffuse = 0.7;
                    object.material.specular = 0.2;
                    object
                },
                {
                    let mut geometry = Sphere::new();
                    geometry.transform = scaling(0.5, 0.5, 0.5);
                    Object {
                        geometry: Geometry::Sphere(geometry),
                        material: Material::new(),
                    }
                },
            ],
            light: Light {
                position: point(-10.0, 10.0, -10.0),
                intensity: vector(1.0, 1.0, 1.0),
                kind: LightKind::Point,
            },
            background_color: color(0.0, 0.0, 0.0),
        }
    }

    fn intersect(&self, r: &Ray) -> Intersections {
        // Generally, objects will return at most 2 intersections, so make space for them.
        let mut i: Intersections = Vec::with_capacity(self.objects.len() * 2);

        for object in self.objects.iter() {
            match Object::intersect(&object, r) {
                (None, None) => (),
                (Some(t1), Some(t2)) => {
                    i.push(Intersection{t: t1, object});
                    i.push(Intersection{t: t2, object});
                },
                (Some(t), None) => {
                    i.push(Intersection{t, object});
                },
                _ => panic!("Object::intersect returned invalid intersections."),
            }
        }

        i.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());

        i
    }

    fn shadow_at_point(&self, p: &Tup) -> f32 {
        match &self.light.kind {
            LightKind::Point => self.point_shadow_intensity(&self.light.position, p),
            LightKind::Area {
                corner: _,
                vvec: _,
                vsteps,
                uvec: _,
                usteps,
                samples,
            } => self.area_light_shadow_intensity(p, *vsteps, *usteps, *samples),
        }
    }

    fn point_shadow_intensity(&self, light: &Tup, p: &Tup) -> f32 {
        let v = light - p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: p.clone(),
            direction,
        };

        let intersections = &self.intersect(&ray);

        match hit(intersections) {
            (hit, idx, true) => {
                if hit < distance {
                    // Make the intensity of the shadow dependant on how transparent the object is.
                    1.0 - intersections[idx].object.material.transparency
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    fn area_light_shadow_intensity(&self, p: &Tup, vsteps: u32, usteps: u32, samples: u32) -> f32 {
        let mut total = 0.0;
        let mut rng = rand::thread_rng();

        for v in 0..vsteps {
            for u in 0..usteps {
                let light_position =
                    self.light
                        .point_on(u, v, rng.gen_range(0., 1.), rng.gen_range(0., 1.));

                total += self.point_shadow_intensity(&light_position, p)
            }
        }

        total / samples as f32
    }

    pub fn color_at(&self, r: &Ray, depth_remaining: u32) -> Tup {
        let intersections = self.intersect(r);

        match hit(&intersections) {
            (_, _, false) => self.background_color.clone(),
            (_, i, true) => self.shade_hit(
                &intersections[i].computations(&r, Some(&intersections)),
                depth_remaining,
            ),
        }
    }

    fn shade_hit(&self, c: &Computations, depth_remaining: u32) -> Tup {
        let reflectiveness = c.object.material.reflectiveness;
        let transparency = c.object.material.transparency;

        let shadow_strength = self.shadow_at_point(&c.over_point) - transparency;
        let shadow_strength = if shadow_strength > 1.0 {
            1.0
        } else if shadow_strength < 0.0 {
            0.0
        } else {
            shadow_strength
        };

        let surface = c.object.material.lighting(
            &(*c.object),
            &self.light,
            c.over_point.clone(),
            c.eye.clone(),
            c.normal.clone(),
            shadow_strength,
        );

        let refracted = self.refracted_color(&c, depth_remaining);
        let reflected = self.reflected_color(&c, depth_remaining);

        if reflectiveness > 0.0 && transparency > 0.0 {
            let reflectance = c.schlick();
            surface + (reflected * reflectance) + (refracted * (1.0 - reflectance))
        } else {
            surface + reflected + refracted
        }
    }

    fn reflected_color(&self, c: &Computations, depth_remaining: u32) -> Tup {
        if depth_remaining == 0 {
            return color(0.0, 0.0, 0.0);
        }
        if (*c.object).material.reflectiveness < std::f32::EPSILON {
            return color(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray {
            origin: c.over_point.clone(),
            direction: c.reflection.clone(),
        };
        let color = self.color_at(&reflect_ray, depth_remaining - 1);

        &color * (*c.object).material.reflectiveness
    }

    fn refracted_color(&self, c: &Computations, depth_remaining: u32) -> Tup {
        if depth_remaining == 0 {
            return color(0.0, 0.0, 0.0);
        }
        if c.object.material.transparency == 0.0 {
            return color(0.0, 0.0, 0.0);
        }

        let n_ratio = c.n1 / c.n2;
        if n_ratio.is_infinite() {
            // ray doesn't hit anything, so use the BG color.
            return self.background_color.clone();
        }

        let cos_i = dot(&c.eye, &c.normal);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            // total internal reflection
            return color(0.0, 0.0, 0.0);
        }

        let cos_t = (1. - sin2_t).sqrt();

        let direction = &(&c.normal * (n_ratio * cos_i - cos_t)) - &(&c.eye * n_ratio);
        let refracted_ray = Ray {
            origin: c.under_point.clone(),
            direction,
        };

        &self.color_at(&refracted_ray, depth_remaining - 1) * c.object.material.transparency
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::intersections::Intersection;
    use super::super::objects::Plane;
    use super::super::transformations::translation;
    use super::*;
    use std::rc::Rc;

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
        w.objects[1].material.ambient = 1.0;
        let r = Ray {
            origin: point(0.0, 0.0, 0.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            t: 1.0,
            object: &w.objects[1],
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

        let mut plane = Plane::new();
        plane.transform = translation(0.0, -1.0, 0.0);
        let mut material = Material::new();
        material.reflectiveness = 0.5;
        let s = Object {
            geometry: Geometry::Plane(plane),
            material,
        };
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: &s,
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

        let mut plane = Plane::new();
        plane.transform = translation(0.0, -1.0, 0.0);
        let mut material = Material::new();
        material.reflectiveness = 0.5;
        let s = Object {
            geometry: Geometry::Plane(plane),
            material,
        };
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -3.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: &s,
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

        let mut plane = Plane::new();
        plane.transform = translation(0.0, -1.0, 0.0);
        let mut material = Material::new();
        material.reflectiveness = 0.5;
        let s = Object {
            geometry: Geometry::Plane(plane),
            material,
        };
        w.objects.push(s.clone());

        let p = 2.0f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, -2.0),
            direction: vector(0.0, -p, p),
        };
        let i = Intersection {
            t: 2.0f32.sqrt(),
            object: &s,
        };
        let c = i.computations(&r, None);
        let color = w.reflected_color(&c, 0);

        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn the_refracted_color_with_opaque_surface() {
        let w = World::new_with_stuff();
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: 4.0,
                object: &w.objects[0],
            },
            Intersection {
                t: 6.0,
                object: &w.objects[0],
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
        let r = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector(0.0, 0.0, 1.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: 4.0,
                object: &w.objects[0],
            },
            Intersection {
                t: 6.0,
                object: &w.objects[0],
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
            let mut material = Material::new();
            material.transparency = 1.0;
            material.refractive_index = 1.5;
            Object {
                geometry: Geometry::Sphere(Sphere::new()),
                material,
            }
        };

        let p = 2f32.sqrt() / 2.0;
        let r = Ray {
            origin: point(0.0, 0.0, p),
            direction: vector(0.0, 1.0, 0.0),
        };
        let xs: Intersections = vec![
            Intersection {
                t: -p,
                object: &w.objects[0],
            },
            Intersection {
                t: p,
                object: &w.objects[0],
            },
        ];
        let comps = xs[1].computations(&r, Some(&xs));

        let color = w.refracted_color(&comps, 5);

        assert_eq!(color.x, 0.0);
        assert_eq!(color.y, 0.0);
        assert_eq!(color.z, 0.0);
    }

    #[test]
    fn shade_hit_reflective_transparent_material() {
        let mut w = World::new_with_stuff();

        let mut floor = Plane::new();
        floor.transform = translation(0.0, -1.0, 0.0);
        let mut material = Material::new();
        material.reflectiveness = 0.5;
        material.transparency = 0.5;
        material.refractive_index = 1.5;
        let s = Object {
            geometry: Geometry::Plane(floor),
            material,
        };
        w.objects.push(s.clone());

        let mut ball = Sphere::new();
        ball.transform = translation(0., -3.5, -0.5);
        let mut material = Material::new();
        material.color = color(1.0, 0., 0.);
        material.ambient = 0.5;
        w.objects.push(Object {
            geometry: Geometry::Sphere(ball),
            material,
        });

        let xs: Intersections = vec![Intersection {
            t: 2.0f32.sqrt(),
            object: &s,
        }];

        let p = (2f32).sqrt() / 2.;
        let r = Ray {
            origin: point(0., 0., -3.),
            direction: vector(0., -p, p),
        };
        let comps = xs[0].computations(&r, Some(&xs));
        let color = w.shade_hit(&comps, 5);

        println!("{}", color);
        assert!((color.x - 1.1149998).abs() < 10e-3);
        assert!((color.y - 0.696432).abs() < 10e-3);
        assert!((color.z - 0.6924281).abs() < 10e-3);
    }
}
