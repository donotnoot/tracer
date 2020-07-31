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
// TODO: Turn this into a monad (View(Mat)) for maximum FP points.
#[derive(Copy, Debug, Clone)]
pub enum Kind {
    General = 0,
    Transform = 1,
    TransformNoScale = 2,
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
        let (det, is_inversible) = self.is_inversible();
        if !is_inversible {
            std::panic!("cannot inverse matrix: {:?}", self);
        }

        let mut res = self.clone();

        for row in 0..self.size as usize {
            for col in 0..self.size as usize {
                let cof = self.cofactor(row, col);
                res.mat[col][row] = cof / det;
            }
        }

        res
    }

    pub fn inverse(&self) -> Mat {
        match self.kind {
            Kind::TransformNoScale => self.inverse_no_scale(),
            // Kind::Transform => self.inverse_with_scale(),
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
        unsafe { matrix_matrix_mul_avx2(self, rhs) }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn matrix_matrix_mul_avx2(lhs: &Mat, rhs: &Mat) -> Mat {
    use core::arch::x86_64::*;

    let mut m = mat(lhs.size);
    m.kind = lhs.kind.worst(&rhs.kind);

    for row in 0..rhs.size {
        for col in (0..lhs.size).step_by(2) {
            let next_col = col + 1;

            let result: [f32; 8] = {
                let lhs = _mm256_set_ps(
                    lhs.mat[row][0],
                    lhs.mat[row][1],
                    lhs.mat[row][2],
                    lhs.mat[row][3],
                    lhs.mat[row][0],
                    lhs.mat[row][1],
                    lhs.mat[row][2],
                    lhs.mat[row][3],
                );
                let rhs = _mm256_set_ps(
                    rhs.mat[0][next_col],
                    rhs.mat[1][next_col],
                    rhs.mat[2][next_col],
                    rhs.mat[3][next_col],
                    rhs.mat[0][col],
                    rhs.mat[1][col],
                    rhs.mat[2][col],
                    rhs.mat[3][col],
                );
                let result = _mm256_mul_ps(lhs, rhs);
                let mut unpacked: [f32; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
                _mm256_storeu_ps(&mut unpacked[0], result);
                unpacked
            };
            m.mat[row][col] = result[0..4].iter().sum::<f32>();
            m.mat[row][next_col] = result[4..].iter().sum::<f32>();
        }
    }

    m
}

/// Matrix multiplication (move)
impl std::ops::Mul<Mat> for Mat {
    type Output = Mat;

    fn mul(self, r: Mat) -> Mat {
        &self * &r
    }
}

/// Matrix multiplication by a tuple
impl<'a, 'b> std::ops::Mul<&'b tuple::Tup> for &'a Mat {
    type Output = tuple::Tup;

    fn mul(self, r: &'b tuple::Tup) -> tuple::Tup {
        unsafe { matrix_tup_mul_avx2(self, r) }
    }
}

#[target_feature(enable = "avx2")]
unsafe fn matrix_tup_mul_avx2(lhs: &Mat, rhs: &Tup) -> Tup {
    use core::arch::x86_64::*;
    let (x, y, z, w) = {
        let rhs = _mm256_set_ps(rhs.w, rhs.z, rhs.y, rhs.x, rhs.w, rhs.z, rhs.y, rhs.x);

        let lhs0 = _mm256_loadu_ps(&lhs.mat[0][0] as *const f32);
        let result0 = _mm256_mul_ps(lhs0, rhs);

        let lhs1 = _mm256_loadu_ps(&lhs.mat[2][0] as *const f32);
        let result1 = _mm256_mul_ps(lhs1, rhs);

        let result = _mm256_hadd_ps(result0, result1);
        let result = _mm256_hadd_ps(result, _mm256_set1_ps(0.0));

        let mut unpacked: [f32; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        _mm256_storeu_ps(&mut unpacked[0] as *mut f32, result);

        (unpacked[0], unpacked[4], unpacked[1], unpacked[5])
    };
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
            kind: Kind::TransformNoScale,
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
