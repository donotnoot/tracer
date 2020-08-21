use super::tuple::Tup;

#[derive(Debug)]
pub enum Light {
    Point(PointLight),
    Area(AreaLight),
}

#[derive(Debug)]
pub struct PointLight {
    pub position: Tup,
    pub color: Tup,
}

#[derive(Debug)]
pub struct AreaLight {
    pub position: Tup,
    pub color: Tup,
    pub corner: Tup,
    pub vvec: Tup,
    pub vsteps: u32,
    pub uvec: Tup,
    pub usteps: u32,
    pub samples: u32,
}

impl AreaLight {
    pub fn point_on(&self, u: u32, v: u32, off_u: f32, off_v: f32) -> Tup {
        &self.corner + &(&self.uvec * (u as f32 * off_u)) + (&self.vvec * (v as f32 * off_v))
    }
}

impl Light {
    pub fn new_point(position: Tup, color: Tup) -> Self {
        Light::Point(PointLight {
            position,
            color
        })
    }

    pub fn new_area(
        position: Tup,
        color: Tup,
        u_size: Tup,
        u_steps: u32,
        v_size: Tup,
        v_steps: u32,
    ) -> Self {
        let corner = &position - &(&u_size / 2.0 + &v_size / 2.0);
        Light::Area(AreaLight {
            position,
            color,
            corner,
            vvec: v_size / v_steps as f32,
            uvec: u_size / u_steps as f32,
            vsteps: v_steps,
            usteps: u_steps,
            samples: v_steps * u_steps,
        })
    }

    pub fn color(&self) -> &Tup {
        match &self {
            Light::Point(light) => &light.color,
            Light::Area(light) => &light.color,
        }
    }

    pub fn position(&self) -> &Tup {
        match &self {
            Light::Point(light) => &light.position,
            Light::Area(light) => &light.position,
        }
    }
}
