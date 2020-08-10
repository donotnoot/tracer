use super::tuple;
use super::tuple::vector;
use super::tuple::Tup;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Mat {
    size: usize,
    pub mat: [[f32; 4]; 4],
    pub kind: Kind,
}

/// Used to determine whether the matrix can be inverted quickly ussing different approaches.
/// Ordered from slowest to fastest.
#[derive(Copy, Debug, Clone)]
pub enum Kind {
    General = 0,
    Transform = 1,
    TransformNoScale = 2,
    Identity = 3,
}

impl Kind {
    fn worst(&self, rhs: &Kind) -> Kind {
        if (*self) as i32 > (*rhs) as i32 {
            *rhs
        } else {
            *self
        }
    }
}

impl Mat {
    pub fn is_inversible(&self) -> (f32, bool) {
        let det = self.determinant();
        (det, det != 0.0)
    }

    pub fn cofactor(&self, row_to_remove: usize, col_to_remove: usize) -> f32 {
        let det = self.submatrix(row_to_remove, col_to_remove).determinant();

        if (row_to_remove + col_to_remove) & 1 == 0 {
            det
        } else {
            -det
        }
    }

    pub fn determinant(&self) -> f32 {
        match self.size {
            2 => self.mat[0][0] * self.mat[1][1] - self.mat[1][0] * self.mat[0][1],
            3 | 4 => {
                let mut result: f32 = 0.0;
                for col in 0..self.size as usize {
                    let cf = self.cofactor(0, col);
                    result += self.mat[0][col] * cf;
                }
                result
            }
            _ => std::panic!("unsupported matrix size"),
        }
    }

    pub fn submatrix(&self, row_to_remove: usize, col_to_remove: usize) -> Mat {
        match self.size {
            4 | 3 => {
                let mut out = mat(self.size - 1);

                let mut out_row: usize = 0;
                for row in 0..self.size {
                    if row == row_to_remove {
                        continue;
                    }

                    let mut out_col: usize = 0;
                    for col in 0..self.size {
                        if col == col_to_remove {
                            continue;
                        }

                        out.mat[out_row][out_col] = self.mat[row][col];
                        out_col += 1;
                    }
                    out_row += 1;
                }

                out
            }
            _ => std::panic!("unsupported matrix size"),
        }
    }

    pub fn transpose(&self) -> Mat {
        let mut m = self.clone();

        m.mat[0][0] = self.mat[0][0];
        m.mat[0][1] = self.mat[1][0];
        m.mat[0][2] = self.mat[2][0];
        m.mat[0][3] = self.mat[3][0];

        m.mat[1][0] = self.mat[0][1];
        m.mat[1][1] = self.mat[1][1];
        m.mat[1][2] = self.mat[2][1];
        m.mat[1][3] = self.mat[3][1];

        m.mat[2][0] = self.mat[0][2];
        m.mat[2][1] = self.mat[1][2];
        m.mat[2][2] = self.mat[2][2];
        m.mat[2][3] = self.mat[3][2];

        m.mat[3][0] = self.mat[0][3];
        m.mat[3][1] = self.mat[1][3];
        m.mat[3][2] = self.mat[2][3];
        m.mat[3][3] = self.mat[3][3];

        m
    }

    fn inverse_no_scale(&self) -> Mat {
        let mut res = self.clone();

        res.mat[0][0] = self.mat[0][0];
        res.mat[0][1] = self.mat[1][0];
        res.mat[0][2] = self.mat[2][0];
        res.mat[1][0] = self.mat[0][1];
        res.mat[1][1] = self.mat[1][1];
        res.mat[1][2] = self.mat[2][1];
        res.mat[2][0] = self.mat[0][2];
        res.mat[2][1] = self.mat[1][2];
        res.mat[2][2] = self.mat[2][2];

        // Transform translation
        let trans = vector(-self.mat[0][3], -self.mat[1][3], -self.mat[2][3]);
        let trans = &res * &trans;
        res.mat[0][3] = trans.x;
        res.mat[1][3] = trans.y;
        res.mat[2][3] = trans.z;
        res.mat[3][3] = trans.w;

        // Bottom row
        res.mat[3][0] = 0.0;
        res.mat[3][1] = 0.0;
        res.mat[3][2] = 0.0;
        res.mat[3][3] = 1.0;

        res
    }

    fn inverse_general(&self) -> Mat {
        let s0 = self.mat[0][0] * self.mat[1][1] - self.mat[1][0] * self.mat[0][1];
        let s1 = self.mat[0][0] * self.mat[1][2] - self.mat[1][0] * self.mat[0][2];
        let s2 = self.mat[0][0] * self.mat[1][3] - self.mat[1][0] * self.mat[0][3];
        let s3 = self.mat[0][1] * self.mat[1][2] - self.mat[1][1] * self.mat[0][2];
        let s4 = self.mat[0][1] * self.mat[1][3] - self.mat[1][1] * self.mat[0][3];
        let s5 = self.mat[0][2] * self.mat[1][3] - self.mat[1][2] * self.mat[0][3];

        let c5 = self.mat[2][2] * self.mat[3][3] - self.mat[3][2] * self.mat[2][3];
        let c4 = self.mat[2][1] * self.mat[3][3] - self.mat[3][1] * self.mat[2][3];
        let c3 = self.mat[2][1] * self.mat[3][2] - self.mat[3][1] * self.mat[2][2];
        let c2 = self.mat[2][0] * self.mat[3][3] - self.mat[3][0] * self.mat[2][3];
        let c1 = self.mat[2][0] * self.mat[3][2] - self.mat[3][0] * self.mat[2][2];
        let c0 = self.mat[2][0] * self.mat[3][1] - self.mat[3][0] * self.mat[2][1];

        let invdet = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let mut result = self.clone();

        result.mat[0][0] =
            (self.mat[1][1] * c5 - self.mat[1][2] * c4 + self.mat[1][3] * c3) * invdet;
        result.mat[0][1] =
            (-self.mat[0][1] * c5 + self.mat[0][2] * c4 - self.mat[0][3] * c3) * invdet;
        result.mat[0][2] =
            (self.mat[3][1] * s5 - self.mat[3][2] * s4 + self.mat[3][3] * s3) * invdet;
        result.mat[0][3] =
            (-self.mat[2][1] * s5 + self.mat[2][2] * s4 - self.mat[2][3] * s3) * invdet;

        result.mat[1][0] =
            (-self.mat[1][0] * c5 + self.mat[1][2] * c2 - self.mat[1][3] * c1) * invdet;
        result.mat[1][1] =
            (self.mat[0][0] * c5 - self.mat[0][2] * c2 + self.mat[0][3] * c1) * invdet;
        result.mat[1][2] =
            (-self.mat[3][0] * s5 + self.mat[3][2] * s2 - self.mat[3][3] * s1) * invdet;
        result.mat[1][3] =
            (self.mat[2][0] * s5 - self.mat[2][2] * s2 + self.mat[2][3] * s1) * invdet;

        result.mat[2][0] =
            (self.mat[1][0] * c4 - self.mat[1][1] * c2 + self.mat[1][3] * c0) * invdet;
        result.mat[2][1] =
            (-self.mat[0][0] * c4 + self.mat[0][1] * c2 - self.mat[0][3] * c0) * invdet;
        result.mat[2][2] =
            (self.mat[3][0] * s4 - self.mat[3][1] * s2 + self.mat[3][3] * s0) * invdet;
        result.mat[2][3] =
            (-self.mat[2][0] * s4 + self.mat[2][1] * s2 - self.mat[2][3] * s0) * invdet;

        result.mat[3][0] =
            (-self.mat[1][0] * c3 + self.mat[1][1] * c1 - self.mat[1][2] * c0) * invdet;
        result.mat[3][1] =
            (self.mat[0][0] * c3 - self.mat[0][1] * c1 + self.mat[0][2] * c0) * invdet;
        result.mat[3][2] =
            (-self.mat[3][0] * s3 + self.mat[3][1] * s1 - self.mat[3][2] * s0) * invdet;
        result.mat[3][3] =
            (self.mat[2][0] * s3 - self.mat[2][1] * s1 + self.mat[2][2] * s0) * invdet;

        result
    }

    pub fn inverse(&self) -> Mat {
        match self.kind {
            Kind::Identity => self.clone(),
            Kind::TransformNoScale => self.inverse_no_scale(),
            _ => self.inverse_general(),
        }
    }
}

impl std::cmp::PartialEq<Mat> for Mat {
    fn eq(&self, b: &Mat) -> bool {
        if self.size != b.size {
            return false;
        }

        // TODO: epsilon cmp, SIMD?
        match self.size {
            4 => self.mat == b.mat,
            3 => {
                (self.mat[0] == b.mat[0]) && (self.mat[1] == b.mat[1]) && (self.mat[2] == b.mat[2])
            }
            2 => (self.mat[0] == b.mat[0]) && (self.mat[1] == b.mat[1]),
            _ => std::panic!("unsupported matrix size"),
        }
    }
}

/// Matrix multiplication (borrow)
impl<'a, 'b> std::ops::Mul<&'b Mat> for &'a Mat {
    type Output = Mat;

    fn mul(self, rhs: &'b Mat) -> Mat {
        mat_mat_mul_brute(self, rhs)
    }
}

fn mat_mat_mul_brute(lhs: &Mat, rhs: &Mat) -> Mat {
    let mut result = mat(4);
    result.kind = lhs.kind.worst(&rhs.kind);

    macro_rules! mat_mul {
        ($row: expr, $col: expr) => {
            result.mat[$row][$col] = lhs.mat[$row][0] * rhs.mat[0][$col]
                + lhs.mat[$row][1] * rhs.mat[1][$col]
                + lhs.mat[$row][2] * rhs.mat[2][$col]
                + lhs.mat[$row][3] * rhs.mat[3][$col];
        };
    }

    mat_mul!(0, 0);
    mat_mul!(0, 1);
    mat_mul!(0, 2);
    mat_mul!(0, 3);

    mat_mul!(1, 0);
    mat_mul!(1, 1);
    mat_mul!(1, 2);
    mat_mul!(1, 3);

    mat_mul!(2, 0);
    mat_mul!(2, 1);
    mat_mul!(2, 2);
    mat_mul!(2, 3);

    mat_mul!(3, 0);
    mat_mul!(3, 1);
    mat_mul!(3, 2);
    mat_mul!(3, 3);

    result
}

/// Matrix multiplication (move)
impl std::ops::Mul<Mat> for Mat {
    type Output = Mat;

    fn mul(self, r: Mat) -> Mat {
        &self * &r
    }
}

/// Matrix multiplication by a tuple
impl<'a, 'b> std::ops::Mul<&'b Tup> for &'a Mat {
    type Output = Tup;

    fn mul(self, rhs: &'b Tup) -> Tup {
        mat_tup_mul_brute(self, rhs)
    }
}

pub fn mat_tup_mul_brute(lhs: &Mat, rhs: &Tup) -> Tup {
    let x = lhs.mat[0][0] * rhs.x
        + lhs.mat[0][1] * rhs.y
        + lhs.mat[0][2] * rhs.z
        + lhs.mat[0][3] * rhs.w;
    let y = lhs.mat[1][0] * rhs.x
        + lhs.mat[1][1] * rhs.y
        + lhs.mat[1][2] * rhs.z
        + lhs.mat[1][3] * rhs.w;
    let z = lhs.mat[2][0] * rhs.x
        + lhs.mat[2][1] * rhs.y
        + lhs.mat[2][2] * rhs.z
        + lhs.mat[2][3] * rhs.w;
    let w = lhs.mat[3][0] * rhs.x
        + lhs.mat[3][1] * rhs.y
        + lhs.mat[3][2] * rhs.z
        + lhs.mat[3][3] * rhs.w;

    Tup { x, y, z, w }
}

pub fn mat(size: usize) -> Mat {
    match size {
        2 | 3 | 4 => Mat {
            size,
            kind: Kind::TransformNoScale,
            mat: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        },
        _ => std::panic!("unsupported matrix size"),
    }
}

pub fn identity(size: usize) -> Mat {
    match size {
        2 | 3 | 4 => Mat {
            size,
            kind: Kind::Identity,
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        },
        _ => std::panic!("unsupported matrix size"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_equality() {
        let a = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let b = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let b = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [-2.0, 1.0, 2.0, 3.0],
                [3.0, 2.0, 1.0, -1.0],
                [4.0, 3.0, 6.0, 5.0],
                [1.0, 2.0, 7.0, 8.0],
            ],
        };
        let r = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ],
        };

        assert_eq!(r, &a * &b);
    }

    #[test]
    fn matrix_tuple_multiplication() {
        let a = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 4.0, 2.0],
                [8.0, 6.0, 4.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let b = tuple::Tup {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };
        let r = tuple::Tup {
            x: 18.0,
            y: 24.0,
            z: 33.0,
            w: 1.0,
        };

        assert_eq!(r, &a * &b);
    }

    #[test]
    fn multiplying_by_the_identity_matrix_returns_the_original_matrix() {
        let m = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };

        let result = &m * &identity(4);
        assert_eq!(result, m)
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 8.0, 7.0, 6.0],
                [5.0, 4.0, 3.0, 2.0],
            ],
        };
        let expected = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 5.0, 9.0, 5.0],
                [2.0, 6.0, 8.0, 4.0],
                [3.0, 7.0, 7.0, 3.0],
                [4.0, 8.0, 6.0, 2.0],
            ],
        };

        assert_eq!(expected, m.transpose())
    }

    #[test]
    fn determinant_2x2_matrix() {
        let m = Mat {
            size: 2,
            kind: Kind::TransformNoScale,
            mat: [
                [1.0, 5.0, 0.0, 0.0],
                [-3.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        };

        assert_eq!(17.0, m.determinant())
    }

    #[test]
    fn submatrices() {
        {
            let m4 = Mat {
                size: 4,
                kind: Kind::TransformNoScale,
                mat: [
                    [1.0, 5.0, 0.0, 0.0],
                    [-3.0, 2.0, 0.0, 0.0],
                    [0.0, 0.0, 5.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ],
            };
            let m4sub = m4.submatrix(0, 0);

            assert_eq!(3, m4sub.size);
            assert_eq!(2.0, m4sub.mat[0][0]);
            assert_eq!(5.0, m4sub.mat[1][1]);
        }

        {
            let m3 = Mat {
                size: 3,
                kind: Kind::TransformNoScale,
                mat: [
                    [1.0, 5.0, 0.0, 0.0],
                    [-3.0, 2.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ],
            };
            let m3sub = m3.submatrix(2, 2);

            assert_eq!(2, m3sub.size);
            assert_eq!(1.0, m3sub.mat[0][0]);
            assert_eq!(2.0, m3sub.mat[1][1]);
        }
    }

    #[test]
    fn cofactrors() {
        {
            let m = Mat {
                size: 3,
                kind: Kind::TransformNoScale,
                mat: [
                    [3.0, 5.0, 0.0, 0.0],
                    [2.0, -1.0, -7.0, 0.0],
                    [6.0, -1.0, 5.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ],
            };

            assert_eq!(-12.0, m.cofactor(0, 0));
            assert_eq!(-25.0, m.cofactor(1, 0));
        }
    }

    #[test]
    fn determinants() {
        {
            let m = Mat {
                size: 3,
                kind: Kind::TransformNoScale,
                mat: [
                    [1.0, 2.0, 6.0, 0.0],
                    [-5.0, 8.0, -4.0, 0.0],
                    [2.0, 6.0, 4.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ],
            };

            assert_eq!(-196.0, m.determinant());
        }

        {
            let m = Mat {
                size: 4,
                kind: Kind::TransformNoScale,
                mat: [
                    [-2.0, -8.0, 3.0, 5.0],
                    [-3.0, 1.0, 7.0, 3.0],
                    [1.0, 2.0, -9.0, 6.0],
                    [-6.0, 7.0, 7.0, -9.0],
                ],
            };

            assert_eq!(-4071.0, m.determinant());
        }
    }

    #[test]
    fn invertability() {
        {
            let m = Mat {
                size: 4,
                kind: Kind::TransformNoScale,
                mat: [
                    [6.0, 4.0, 4.0, 4.0],
                    [5.0, 5.0, 7.0, 6.0],
                    [4.0, -9.0, 3.0, -7.0],
                    [9.0, 1.0, 7.0, -6.0],
                ],
            };

            assert_eq!(true, m.is_inversible().1)
        }

        {
            let m = Mat {
                size: 4,
                kind: Kind::TransformNoScale,
                mat: [
                    [-4.0, 2.0, -2.0, -3.0],
                    [9.0, 6.0, 2.0, 6.0],
                    [0.0, -5.0, 1.0, -5.0],
                    [0.0, 0.0, 0.0, 0.0],
                ],
            };

            assert_eq!(false, m.is_inversible().1)
        }
    }

    #[test]
    fn inverse_of_matrix() {
        let m = Mat {
            size: 4,
            kind: Kind::TransformNoScale,
            mat: [
                [-5.0, 2.0, 6.0, -8.0],
                [1.0, -5.0, 1.0, 8.0],
                [7.0, 7.0, -6.0, -7.0],
                [1.0, -3.0, 7.0, 4.0],
            ],
        };
        let det = m.determinant();
        let cof_a = m.cofactor(2, 3);
        let cof_b = m.cofactor(3, 2);

        let inv = m.inverse();

        assert_eq!(cof_a / det, inv.mat[3][2]);
        assert_eq!(cof_b / det, inv.mat[2][3]);
    }

    #[test]
    fn multiplication_keeps_kind_of_rhs() {
        let mut lhs = identity(4);
        lhs.kind = Kind::TransformNoScale;

        let mut rhs = identity(4);
        rhs.kind = Kind::General;

        {
            // Borrow
            let res = &lhs * &rhs;

            match res.kind {
                Kind::General => (),
                _ => panic!("borrow: kind is not expected"),
            };
        }

        {
            // Move
            let res = lhs * rhs;

            match res.kind {
                Kind::General => (),
                _ => panic!("move: kind is not expected"),
            };
        }
    }
}
