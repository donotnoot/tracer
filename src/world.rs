use super::intersections::{hit, Computations, Intersect, Intersections};
use super::light::PointLight;
use super::material::HasMaterial;
use super::objects::{Object, Sphere};
use super::ray::Ray;
use super::transformations::scaling;
use super::tuple::{color, point, vector, Tup};

pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
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
                intensity: vector(-10.0, 10.0, -10.0),
            },
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

    fn is_shadowed(&self, p: Tup) -> bool {
        let v = &(self.light.position) - &p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: p,
            direction,
        };

        match hit(&self.intersect(&ray)) {
            (hit, _, true) => return hit < distance,
            _ => return false,
        }
    }

    pub fn color_at(&self, r: Ray) -> Tup {
        let intersections = self.intersect(&r);

        match hit(&intersections) {
            (_, _, false) => return color(0.0, 0.0, 0.0),
            (_, i, true) => return self.shade_hit(intersections[i].computations(&r)),
        }
    }

    fn shade_hit(&self, c: Computations) -> Tup {
        let s = self.is_shadowed(c.over_point);
        c.object
            .material()
            .lighting(&self.light, c.point, c.eye, c.normal, s)
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

        ixs.iter().for_each(|e| println!("{:?}", e));

        assert_eq!(ixs.len(), 4);
        assert_eq!(ixs[0].t, 4.0);
        assert_eq!(ixs[1].t, 4.5);
        assert_eq!(ixs[2].t, 5.5);
        assert_eq!(ixs[3].t, 6.0);
    }
}
