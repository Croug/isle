use std::ops::Mul;

use crate::rotation::Rotation;

use super::vector::Vec3;
pub struct Matrix<const R: usize, const C: usize>(pub [[f32; C]; R]);

pub type Mat4 = Matrix<4, 4>;
pub type Mat3 = Matrix<3, 3>;
pub type Mat2 = Matrix<2, 2>;

impl Mat4 {
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

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let z = (eye - target).norm();
        let x = up.cross(&z).norm();
        let y = z.cross(&x);

        Matrix([
            [x.0, x.1, x.2, -x.dot(&eye)],
            [y.0, y.1, y.2, -y.dot(&eye)],
            [z.0, z.1, z.2, -z.dot(&eye)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(vector: Vec3) -> Self {
        Matrix([
            [1.0, 0.0, 0.0, vector.0],
            [0.0, 1.0, 0.0, vector.1],
            [0.0, 0.0, 1.0, vector.2],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(vector: Vec3) -> Self {
        Matrix([
            [vector.0, 0.0, 0.0, 0.0],
            [0.0, vector.1, 0.0, 0.0],
            [0.0, 0.0, vector.2, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transform(scale: Vec3, rotation: Rotation, translation: Vec3) -> Self {
        Self::scale(scale) * rotation.into() * Self::translation(translation)
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