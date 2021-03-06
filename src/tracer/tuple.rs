#[derive(Default, Debug, Clone)]
#[repr(C)]
pub struct Tup {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

/// New point tuple
pub fn point(x: f32, y: f32, z: f32) -> Tup {
    Tup { x, y, z, w: 1.0 }
}

/// New vector tuple
pub fn vector(x: f32, y: f32, z: f32) -> Tup {
    Tup { x, y, z, w: 0.0 }
}

/// New color tuple
pub fn color(r: f32, g: f32, b: f32) -> Tup {
    Tup {
        x: r,
        y: g,
        z: b,
        w: 0.0,
    }
}

pub fn color_u8(r: u8, g: u8, b: u8) -> Tup {
    let c = |c: u8| -> f32 { c as f32 / 255.0 };
    Tup {
        x: c(r),
        y: c(g),
        z: c(b),
        w: 0.0,
    }
}

impl Tup {
    pub fn new() -> Self {
        Tup {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn with_x(mut self, v: f32) -> Self {
        self.x = v;
        self
    }

    pub fn with_y(mut self, v: f32) -> Self {
        self.y = v;
        self
    }

    pub fn with_z(mut self, v: f32) -> Self {
        self.z = v;
        self
    }

    pub fn with_w(mut self, v: f32) -> Self {
        self.w = v;
        self
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Reflect on a normal
    pub fn reflect(&self, normal: &Tup) -> Tup {
        self - &(&(normal * 2.0) * dot(self, normal))
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tup {
        let mag = self.magnitude();
        Tup {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn cmp_epsilon(&self, x: f32, y: f32, z: f32, w: f32) -> bool {
        (self.x - x).abs() < std::f32::EPSILON
            && (self.y - y).abs() < std::f32::EPSILON
            && (self.z - z).abs() < std::f32::EPSILON
            && (self.w - w).abs() < std::f32::EPSILON
    }
}

/// Adds two tuples
impl<'a, 'b> std::ops::Add<&'b Tup> for &'a Tup {
    type Output = Tup;

    fn add(self, r: &'b Tup) -> Tup {
        Tup {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
            w: self.w + r.w,
        }
    }
}

/// Adds two tuples(move)
impl std::ops::Add<Tup> for Tup {
    type Output = Tup;

    fn add(self, r: Tup) -> Tup {
        &self + &r
    }
}

/// Scalar division
impl<'a> std::ops::Div<f32> for &'a Tup {
    type Output = Tup;

    fn div(self, f: f32) -> Tup {
        Tup {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
            w: self.w / f,
        }
    }
}

/// Scalar division (move)
impl std::ops::Div<f32> for Tup {
    type Output = Tup;

    fn div(self, f: f32) -> Tup {
        &self / f
    }
}

/// Scalar multiplication
impl<'a> std::ops::Mul<f32> for &'a Tup {
    type Output = Tup;

    fn mul(self, f: f32) -> Tup {
        Tup {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f,
        }
    }
}

/// Scalar multiplication (move)
impl std::ops::Mul<f32> for Tup {
    type Output = Tup;

    fn mul(self, f: f32) -> Tup {
        &self * f
    }
}

/// Hadamard product
impl<'a, 'b> std::ops::Mul<&'b Tup> for &'a Tup {
    type Output = Tup;

    fn mul(self, rhs: &'b Tup) -> Tup {
        Tup {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

/// Hadamard product (move)
impl std::ops::Mul<Tup> for Tup {
    type Output = Tup;

    fn mul(self, rhs: Tup) -> Tup {
        &self * &rhs
    }
}

/// Subtracts two tuples
impl<'a, 'b> std::ops::Sub<&'b Tup> for &'a Tup {
    type Output = Tup;

    fn sub(self, rhs: &'b Tup) -> Tup {
        Tup {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// Subtracts two tuples (move)
impl std::ops::Sub<Tup> for Tup {
    type Output = Tup;

    fn sub(self, f: Tup) -> Tup {
        &self - &f
    }
}

/// Negates a tuple
impl<'a> std::ops::Neg for &'a Tup {
    type Output = Tup;

    fn neg(self) -> Tup {
        Tup {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl std::cmp::PartialEq<Tup> for Tup {
    fn eq(&self, b: &Tup) -> bool {
        self.x == b.x && self.y == b.y && self.z == b.z && self.w == b.w
    }
}

impl std::fmt::Display for Tup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl std::iter::Sum<Tup> for Tup {
    fn sum<I>(iter: I) -> Tup
    where
        I: Iterator<Item = Tup>,
    {
        let mut result = Tup::new();
        for v in iter {
            result = result + v;
        }
        result
    }
}

/// Cross product
pub fn cross(a: &Tup, b: &Tup) -> Tup {
    vector(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

/// Dot product
pub fn dot(a: &Tup, b: &Tup) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_a_tup_with_w1_is_a_point() {
        assert_eq!(Tup::new().with_w(1.0).is_point(), true)
    }

    #[test]
    fn test_a_tup_with_w0_is_a_vector() {
        assert_eq!(Tup::new().with_w(0.0).is_vector(), true)
    }

    #[test]
    fn adding_two_tuples() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let b = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = &a + &b;

        assert_eq!(r.x, 2.0);
        assert_eq!(r.y, 4.0);
        assert_eq!(r.z, 6.0);
        assert_eq!(r.w, 8.0);
    }

    #[test]
    fn subtracting_two_tuples() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let b = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = &a - &b;

        assert_eq!(r.x, 0.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.z, 0.0);
        assert_eq!(r.w, 0.0);
    }

    #[test]
    fn subtracting_two_points_should_yield_vector() {
        let a = point(3.0, 2.0, 1.0);
        let b = point(5.0, 6.0, 7.0);
        let r = &a - &b;

        assert_eq!(r.is_vector(), true);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_should_yield_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let r = &p - &v;

        assert_eq!(r.is_point(), true);
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = -&a;

        assert_eq!(r.x, -1.00);
        assert_eq!(r.y, -2.00);
        assert_eq!(r.z, -3.00);
        assert_eq!(r.w, -4.00);
    }

    #[test]
    fn multiplying_a_tuple_by_scalar() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = &a * 2.0;

        assert_eq!(r.x, 2.00);
        assert_eq!(r.y, 4.00);
        assert_eq!(r.z, 6.00);
        assert_eq!(r.w, 8.00);
    }

    #[test]
    fn multiplying_a_tuple_by_a_tuple() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let b = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = &a * &b;

        assert_eq!(r.x, 1.00);
        assert_eq!(r.y, 4.00);
        assert_eq!(r.z, 9.00);
        assert_eq!(r.w, 16.00);
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        let r = &a / 2.0;

        assert_eq!(r.x, 0.5);
        assert_eq!(r.y, 1.0);
        assert_eq!(r.z, 1.5);
        assert_eq!(r.w, 2.0);
    }

    macro_rules! vector_magnitude {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, input.magnitude());
            }
        )*
        }
    }

    vector_magnitude! {
        vector_magnitude_100: (vector(1.0,0.0,0.0), 1.0),
        vector_magnitude_010: (vector(0.0,1.0,0.0), 1.0),
        vector_magnitude_001: (vector(0.0,0.0,1.0), 1.0),
        vector_magnitude_123: (vector(1.0,2.0,3.0), (14.0 as f32).sqrt()),
        vector_magnitude_123_neg: (vector(-1.0,-2.0,-3.0), (14.0 as f32).sqrt()),
    }

    macro_rules! vector_normalizing {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, input.normalize());
            }
        )*
        }
    }

    vector_normalizing! {
        vector_normalizing_400: (vector(4.0, 0.0,0.0), vector(1.0,0.0,0.0)),
        vector_normalizing_004: (vector(0.0, 0.0,4.0), vector(0.0,0.0,1.0)),
    }

    #[test]
    fn dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let dot = dot(&a, &b);

        assert_eq!(dot, 20.0)
    }

    #[test]
    fn cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        let crossab = cross(&a, &b);
        let crossba = cross(&b, &a);

        assert_eq!(crossab.x, -1.0);
        assert_eq!(crossab.y, 2.0);
        assert_eq!(crossab.z, -1.0);
        assert_eq!(crossba.x, 1.0);
        assert_eq!(crossba.y, -2.0);
        assert_eq!(crossba.z, 1.0);
    }

    #[test]
    fn reflecting_a_vector_off_45_deg_normal() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector(0.0, 1.0, 0.0);
        let r = v.reflect(&n);

        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, 1.0);
        assert_eq!(r.z, 0.0);
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = vector(0.0, -1.0, 0.0);
        let p = (2 as f32).sqrt() / 2.0;
        let n = vector(p, p, 0.0);
        let r = v.reflect(&n);

        // TODO: compare these properly?
        assert!((r.x - 1.0).abs() < 10e-4);
        assert!((r.y - 0.0).abs() < 10e-4);
        assert!((r.z - 0.0).abs() < 10e-4);
    }
}
