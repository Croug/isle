use std::ops::Mul;

use super::vector::Vec3;
pub struct Matrix<const R: usize, const C: usize>([[f32; C]; R]);

pub type Mat4 = Matrix<4, 4>;
pub type Mat3 = Matrix<3, 3>;
pub type Mat2 = Matrix<2, 2>;

impl Matrix<4, 4> {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn perspective_projection(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Self {
        let f = 1.0 / (fovy / 2.0).tan();
        let sx = f / aspect;
        let sy = f;

        let a = -zfar / (zfar - znear);
        let b = -(zfar * znear) / (zfar - znear);

        Matrix([
            [sx  , 0.0 , 0.0 , 0.0 ],
            [0.0 , sy  , 0.0 , 0.0 ],
            [0.0 , 0.0 , a   , b   ],
            [0.0 , 0.0 , -1.0, 0.0 ],
        ])
    }
}

impl<const R: usize, const C: usize, const U: usize> Mul<Matrix<U,C>> for Matrix<R, U> {
    type Output = Matrix<R, C>;

    fn mul(self, rhs: Matrix<U,C>) -> Self::Output {
        let mut result = [[0.0; C]; R];

        for i in 0..R {
            for j in 0..C {
                for k in 0..U {
                    result[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        Matrix(result)
    }
}