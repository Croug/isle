pub mod matrix {
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

        // pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {

        // }
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
}
pub mod vector {
    use std::ops::{Add, Div, Mul, Sub};

    pub struct Vec3(pub f32, pub f32, pub f32);
    pub struct Vec2(pub f32, pub f32);

    impl Vec3 {
        pub fn cross(&self, other: &Self) -> Self {
            Self(
                self.1 * other.2 - self.2 * other.1,
                self.2 * other.0 - self.0 * other.2,
                self.0 * other.1 - self.1 * other.0,
            )
        }

        pub fn dot(&self, other: &Self) -> f32 {
            self.0 * other.0 + self.1 * other.1 + self.2 * other.2
        }
    }

    impl Div<f32> for Vec3 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self::Output {
            Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
        }
    }

    impl Mul<f32> for Vec3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self::Output {
            Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
        }
    }

    impl Add<f32> for Vec3 {
        type Output = Self;

        fn add(self, rhs: f32) -> Self::Output {
            Self(self.0 + rhs, self.1 + rhs, self.2 + rhs)
        }
    }

    impl Sub<f32> for Vec3 {
        type Output = Self;

        fn sub(self, rhs: f32) -> Self::Output {
            Self(self.0 - rhs, self.1 - rhs, self.2 - rhs)
        }
    }
}