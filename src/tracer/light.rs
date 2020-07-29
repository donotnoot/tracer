use super::tuple::Tup;

#[derive(Debug)]
pub enum LightKind {
    Point,
    Area {
        corner: Tup,
        vvec: Tup,
        vsteps: u32,
        uvec: Tup,
        usteps: u32,
        samples: u32,
    },
}

#[derive(Debug)]
pub struct Light {
    pub position: Tup,
    pub intensity: Tup,
    pub kind: LightKind,
}

impl Light {
    pub fn new_area(
        color: &Tup,
        corner: &Tup,
        uvec: &Tup,
        usteps: u32,
        vvec: &Tup,
        vsteps: u32,
    ) -> Self {
        Light {
            position: corner + &(uvec / 2.0 + vvec / 2.0),
            intensity: color.clone(),
            kind: LightKind::Area {
                corner: corner.clone(),
                vvec: vvec / vsteps as f32,
                uvec: uvec / usteps as f32,
                vsteps,
                usteps,
                samples: vsteps * usteps,
            },
        }
    }

    pub fn point_on(&self, u: u32, v: u32, off_u: f32, off_v: f32) -> Tup {
        match &self.kind {
            LightKind::Point => self.position.clone(),
            LightKind::Area {
                corner,
                vvec,
                vsteps: _,
                uvec,
                usteps: _,
                samples: _,
            } => corner + &(uvec * (u as f32 * off_u)) + (vvec * (v as f32 * off_v)),
        }
    }
}
