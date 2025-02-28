use std::ops::Mul;

use crate::{
    rotation::{Angle, Rotation},
    vector::{d2::Vec2, d3::Vec3, d4::Vec4},
};

#[derive(Clone, Copy, Debug)]
pub struct Matrix<const C: usize, const R: usize>(pub [[f32; R]; C]);

pub type Mat4 = Matrix<4, 4>;
pub type Mat3 = Matrix<3, 3>;
pub type Mat2 = Matrix<2, 2>;

impl<const C: usize, const R: usize> Matrix<C, R> {
    pub fn transpose(&self) -> Matrix<R, C> {
        let mut result = Matrix::<R, C>([[0.0; C]; R]);

        for i in 0..C {
            for j in 0..R {
                result.set(i, j, self.get(j, i));
            }
        }

        result
    }

    pub fn get(&self, col: usize, row: usize) -> f32 {
        self.0[col][row]
    }

    pub fn set(&mut self, col: usize, row: usize, value: f32) {
        self.0[col][row] = value;
    }
}

impl Mat4 {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0], // Column 0
            [0.0, 1.0, 0.0, 0.0], // Column 1
            [0.0, 0.0, 1.0, 0.0], // Column 2
            [0.0, 0.0, 0.0, 1.0], // Column 3
        ])
    }

    pub fn perspective_projection(aspect: f32, fovy: Angle, znear: f32, zfar: f32) -> Self {
        let f = (fovy.to_radians() / 2.0).tan().recip();

        Matrix([
            [f / aspect, 0.0, 0.0, 0.0],                     // Column 0
            [0.0, f, 0.0, 0.0],                              // Column 1
            [0.0, 0.0, zfar / (zfar - znear), 1.0],          // Column 2
            [0.0, 0.0, -znear * zfar / (zfar - znear), 0.0], // Column 3
        ])
    }

    pub fn orthographic_projection(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let a = 2.0 / (right - left);
        let b = 2.0 / (top - bottom);
        let c = 1.0 / (zfar - znear);

        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -znear / (zfar - znear);

        Matrix([
            [a, 0.0, 0.0, 0.0], // Column 0
            [0.0, b, 0.0, 0.0], // Column 1
            [0.0, 0.0, c, 0.0], // Column 2
            [tx, ty, tz, 1.0],  // Column 3
        ])
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let z = (target - eye).norm();
        let x = up.cross(&z).norm();
        let y = z.cross(&x);

        Matrix([
            [x.0, y.0, z.0, 0.0],                            // Column 0
            [x.1, y.1, z.1, 0.0],                            // Column 1
            [x.2, y.2, z.2, 0.0],                            // Column 2
            [-x.dot(&eye), -y.dot(&eye), -z.dot(&eye), 1.0], // Column 3
        ])
    }

    pub fn translation(vector: Vec3) -> Self {
        Matrix([
            [1.0, 0.0, 0.0, 0.0],                // Column 0
            [0.0, 1.0, 0.0, 0.0],                // Column 1
            [0.0, 0.0, 1.0, 0.0],                // Column 2
            [vector.0, vector.1, vector.2, 1.0], // Column 3
        ])
    }

    pub fn scale(vector: Vec3) -> Self {
        Matrix([
            [vector.0, 0.0, 0.0, 0.0], // Column 0
            [0.0, vector.1, 0.0, 0.0], // Column 1
            [0.0, 0.0, vector.2, 0.0], // Column 2
            [0.0, 0.0, 0.0, 1.0],      // Column 3
        ])
    }

    pub fn transform(scale: Vec3, rotation: &Rotation, translation: Vec3) -> Self {
        Self::translation(translation) * rotation.to_mat4() * Self::scale(scale)
    }

    pub fn inverse_transform(scale: Vec3, rotation: &Rotation, translation: Vec3) -> Self {
        let inv_scale = Mat4::scale(Vec3(1. / scale.0, 1. / scale.1, 1. / scale.2));
        let inv_rot = rotation.to_mat4().transpose();
        let inv_translation = Mat4::translation(-translation);

        inv_rot * inv_scale * inv_translation
    }

    pub fn determinant(&self) -> f32 {
        let [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self.0;

        let coef00 = f * (k * p - o * l) - g * (j * p - n * l) + h * (j * o - n * k);
        let coef01 = -(e * (k * p - o * l) - g * (i * p - m * l) + h * (i * o - m * k));
        let coef02 = e * (j * p - n * l) - f * (i * p - m * l) + h * (i * n - m * j);
        let coef03 = -(e * (j * o - n * k) - f * (i * o - m * k) + g * (i * n - m * j));

        a * coef00 + b * coef01 + c * coef02 + d * coef03
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None; // Singular matrix, no inverse
        }

        let [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]] = self.0;

        // Cofactor matrix
        let cofactors = Matrix([
            [
                f * (k * p - o * l) - g * (j * p - n * l) + h * (j * o - n * k),
                -(e * (k * p - o * l) - g * (i * p - m * l) + h * (i * o - m * k)),
                e * (j * p - n * l) - f * (i * p - m * l) + h * (i * n - m * j),
                -(e * (j * o - n * k) - f * (i * o - m * k) + g * (i * n - m * j)),
            ],
            [
                -(b * (k * p - o * l) - c * (j * p - n * l) + d * (j * o - n * k)),
                a * (k * p - o * l) - c * (i * p - m * l) + d * (i * o - m * k),
                -(a * (j * p - n * l) - b * (i * p - m * l) + d * (i * n - m * j)),
                a * (j * o - n * k) - b * (i * o - m * k) + c * (i * n - m * j),
            ],
            [
                b * (g * p - o * h) - c * (f * p - n * h) + d * (f * o - n * g),
                -(a * (g * p - o * h) - c * (e * p - m * h) + d * (e * o - m * g)),
                a * (f * p - n * h) - b * (e * p - m * h) + d * (e * n - m * f),
                -(a * (f * o - n * g) - b * (e * o - m * g) + c * (e * n - m * f)),
            ],
            [
                -(b * (g * l - k * h) - c * (f * l - j * h) + d * (f * k - j * g)),
                a * (g * l - k * h) - c * (e * l - i * h) + d * (e * k - i * g),
                -(a * (f * l - j * h) - b * (e * l - i * h) + d * (e * j - i * f)),
                a * (f * k - j * g) - b * (e * k - i * g) + c * (e * j - i * f),
            ],
        ]);

        let adjugate = cofactors.transpose();
        Some(adjugate * (1.0 / det))
    }

    pub fn normal(&self) -> Option<Self> {
        Some(self.inverse()?.transpose())
    }
}

impl Mat3 {
    pub fn scale(vector: &Vec3) -> Self {
        Matrix([
            [vector.0, 0.0, 0.0],
            [0.0, vector.1, 0.0],
            [0.0, 0.0, vector.2],
        ])
    }
    pub fn to_align_4(&self) -> [[f32; 4]; 3] {
        let [[a, b, c], [d, e, f], [g, h, i]] = self.0;

        [[a, b, c, 0.0], [d, e, f, 0.0], [g, h, i, 0.0]]
    }
    pub fn normal(transform: &Mat4) -> Option<Self> {
        Some(transform.normal()?.into())
    }
}

impl<const C: usize, const R: usize, const U: usize> Mul<Matrix<C, U>> for Matrix<U, R> {
    type Output = Matrix<C, R>;

    fn mul(self, rhs: Matrix<C, U>) -> Self::Output {
        let mut result = Matrix::<C, R>([[0.0; R]; C]);

        for i in 0..C {
            for j in 0..R {
                for k in 0..U {
                    result.0[i][j] += self.get(k, j) * rhs.get(i, k);
                }
            }
        }

        result
    }
}

impl<const C: usize, const R: usize> Mul<f32> for Matrix<C, R> {
    type Output = Matrix<C, R>;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Matrix([[0.0; R]; C]);

        for i in 0..C {
            for j in 0..R {
                result.0[i][j] = self.get(i, j) * rhs;
            }
        }

        result
    }
}

impl Into<Mat2> for Mat3 {
    fn into(self) -> Mat2 {
        let [[a, b, _], [c, d, _], _] = self.0;

        Matrix([[a, b], [c, d]])
    }
}

impl Into<Mat3> for Mat4 {
    fn into(self) -> Mat3 {
        let [[a, b, c, _], [d, e, f, _], [g, h, i, _], _] = self.0;

        Matrix([[a, b, c], [d, e, f], [g, h, i]])
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Matrix<1, 4>;

    fn mul(self, rhs: Vec4) -> Self::Output {
        let mat: Matrix<1, 4> = rhs.into();
        self * mat
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Matrix<1, 3>;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mat: Matrix<1, 3> = rhs.into();
        self * mat
    }
}

impl Mul<Vec2> for Mat2 {
    type Output = Matrix<1, 2>;

    fn mul(self, rhs: Vec2) -> Self::Output {
        let mat: Matrix<1, 2> = rhs.into();
        self * mat
    }
}
