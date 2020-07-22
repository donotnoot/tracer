use super::tuple::Tup;

pub enum LightKind {
    Point,
}

pub struct Light {
    pub position: Tup,
    pub intensity: Tup,
    pub kind: LightKind,
}
