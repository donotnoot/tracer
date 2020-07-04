use super::objects::{Object};
use super::light::{PointLight};
use super::intersections::{hit, Intersect, Intersections, Computations};
use super::material::{HasMaterial};
use super::ray::{Ray};
use super::tuple::{Tup, point, vector, color};

pub struct World {
    pub objects: Vec<Object>,
    pub light: PointLight,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: PointLight{
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

        i
    }

    fn is_shadowed(&self, p: Tup) -> bool {
        let v = &(self.light.position) - &p;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray{
            origin: p,
            direction,
        };

        match hit(&self.intersect(&ray)) {
        (hit, _, true) => return hit < distance,
        _ => return false
        }
    }

    pub fn color_at(&self, r: Ray) -> Tup {
        let intersections = self.intersect(&r);

        match hit(&intersections) {
        (_,_,false) => return color(0.0,0.0,0.0),
        (_, i, true) => return self.shade_hit(intersections[i].computations(&r)),
        }
    }

    fn shade_hit(&self, c: Computations) -> Tup {
        let s = self.is_shadowed(c.over_point);
        c.object.material().lighting(&self.light, c.point, c.eye, c.normal, s)
    }
}
