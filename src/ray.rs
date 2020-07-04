use super::matrix::Mat;
use super::tuple::Tup;

pub struct Ray {
    pub origin: Tup,
    pub direction: Tup,
}

impl Ray {
    pub fn position(&self, t: f64) -> Tup {
        &self.origin + &(&self.direction * t)
    }

    pub fn transform(&self, m: &Mat) -> Ray {
        Ray {
            origin: m * &self.origin,
            direction: m * &self.direction,
        }
    }
}
