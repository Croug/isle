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

        for i in 0..R {
            for j in 0..C {
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
