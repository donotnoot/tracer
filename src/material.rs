use super::light::PointLight;
use super::tuple::{color, dot, Tup};

#[derive(Clone)]
pub struct Material {
    pub color: Tup,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(&self, l: &PointLight, p: Tup, eye: Tup, normal: Tup, inShadow: bool) -> Tup {
        let effective_color = &self.color * &l.intensity;
        let ambient = &effective_color * self.ambient;

        if inShadow {
            return ambient;
        }

        let light = (&l.position - &p).normalize();
        let light_normal_dot = dot(&light, &normal);

        if light_normal_dot < 0.0 {
            let diffuse = color(0.0, 0.0, 0.0);
            let specular = color(0.0, 0.0, 0.0);
            return &ambient + &(&diffuse + &specular);
        } else {
            let diffuse = &effective_color * (&self.diffuse * &light_normal_dot);
            let reflect = &(-&light).reflect(&normal);
            let reflect_dot_eye = dot(&reflect, &eye);

            if reflect_dot_eye <= 0.0 {
                let specular = color(0.0, 0.0, 0.0);
                return &ambient + &(&diffuse + &specular);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                let specular = &l.intensity * (&self.specular * factor);
                return &ambient + &(&diffuse + &specular);
            }
        }
    }
}

pub trait HasMaterial {
    fn material(&self) -> Material;
}
