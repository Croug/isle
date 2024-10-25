use std::ops::Mul;

use quaternion::Quaternion;

use crate::{
    matrix::{Mat4, Matrix},
    vector::{d3::Vec3, d4::Vec4},
};

pub mod quaternion {
    use crate::{
        matrix::{Mat4, Matrix},
        vector::d4::Vec4,
    };

    use super::Rotation;

    #[derive(Clone, Copy, Debug)]
    pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

    impl Quaternion {
        pub const IDENTITY: Self = Self(0.0, 0.0, 0.0, 1.0);
        pub const ZERO: Self = Self(0.0, 0.0, 0.0, 0.0);

        pub fn dot(&self, other: &Quaternion) -> f32 {
            <Self as Into<Vec4>>::into(*self).dot(&other.into())
        }

        pub fn mag(&self) -> f32 {
            <Self as Into<Vec4>>::into(*self).mag()
        }

        pub fn norm(&self) -> Self {
            <Self as Into<Vec4>>::into(*self).norm().into()
        }

        pub fn to_mat4(&self) -> Mat4 {
            let xx = self.0 * self.0;
            let yy = self.1 * self.1;
            let zz = self.2 * self.2;
            let xy = self.0 * self.1;
            let xz = self.0 * self.2;
            let yz = self.1 * self.2;
            let wx = self.3 * self.0;
            let wy = self.3 * self.1;
            let wz = self.3 * self.2;

            Matrix([
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0],
                [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0],
                [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        }
    }

    impl Into<Mat4> for Quaternion {
        fn into(self) -> Mat4 {
            self.to_mat4()
        }
    }

    impl Into<Rotation> for Quaternion {
        fn into(self) -> Rotation {
            Rotation::Quaternion(self)
        }
    }

    impl Into<Vec4> for Quaternion {
        fn into(self) -> Vec4 {
            Vec4(self.0, self.1, self.2, self.3)
        }
    }

    impl Into<Vec4> for &Quaternion {
        fn into(self) -> Vec4 {
            Vec4(self.0, self.1, self.2, self.3)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rotation {
    Euler(Vec3),
    Quaternion(quaternion::Quaternion),
}

impl Rotation {
    pub fn quaternion_identity() -> Self {
        Quaternion::IDENTITY.into()
    }
    pub fn euler_identity() -> Self {
        Rotation::Euler(Vec3::ZERO)
    }
    pub fn to_mat4(&self) -> Mat4 {
        match self {
            Rotation::Quaternion(quaternion) => quaternion.to_mat4(),
            Rotation::Euler(euler) => {
                Matrix([
                    [1.0, 0.0, 0.0, 0.0],                      // Column 0
                    [0.0, euler.0.cos(), -euler.0.sin(), 0.0], // Column 1
                    [0.0, euler.0.sin(), euler.0.cos(), 0.0],  // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ]) * Matrix([
                    [euler.1.cos(), 0.0, euler.1.sin(), 0.0],  // Column 0
                    [0.0, 1.0, 0.0, 0.0],                      // Column 1
                    [-euler.1.sin(), 0.0, euler.1.cos(), 0.0], // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ]) * Matrix([
                    [euler.2.cos(), -euler.2.sin(), 0.0, 0.0], // Column 0
                    [euler.2.sin(), euler.2.cos(), 0.0, 0.0],  // Column 1
                    [0.0, 0.0, 1.0, 0.0],                      // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ])
            }
        }
    }
}

impl Into<Mat4> for Rotation {
    fn into(self) -> Mat4 {
        self.to_mat4()
    }
}

impl From<Vec3> for Rotation {
    fn from(euler: Vec3) -> Self {
        Rotation::Euler(euler)
    }
}

impl Mul<Vec3> for Rotation {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let rotation_mat = self.to_mat4();
        let mut vec4: Vec4 = rhs.into();
        let mat = rotation_mat * vec4;
        vec4 = mat.into();
        vec4.xyz()
    }
}

pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

impl Angle {
    pub fn to_radians(&self) -> f32 {
        match self {
            Angle::Radians(radians) => *radians,
            Angle::Degrees(degrees) => degrees.to_radians(),
        }
    }

    pub fn to_degrees(&self) -> f32 {
        match self {
            Angle::Radians(radians) => radians.to_degrees(),
            Angle::Degrees(degrees) => *degrees,
        }
    }
}
