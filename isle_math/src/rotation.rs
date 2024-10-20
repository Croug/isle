use crate::{
    matrix::{Mat4, Matrix},
    vector::d3::Vec3,
};

pub mod quaternion {
    use crate::matrix::{Mat4, Matrix};

    pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

    impl Quaternion {
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
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
                [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
                [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        }
    }

    impl Into<Mat4> for Quaternion {
        fn into(self) -> Mat4 {
            self.to_mat4()
        }
    }
}

pub enum Rotation {
    Euler(Vec3),
    Quaternion(quaternion::Quaternion),
}

impl Rotation {
    pub fn to_mat4(&self) -> Mat4 {
        match self {
            Rotation::Quaternion(quaternion) => quaternion.to_mat4(),
            Rotation::Euler(euler) => {
                Matrix([
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, euler.0.cos(), -euler.0.sin(), 0.0],
                    [0.0, euler.0.sin(), euler.0.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]) * Matrix([
                    [euler.1.cos(), 0.0, euler.1.sin(), 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [-euler.1.sin(), 0.0, euler.1.cos(), 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ]) * Matrix([
                    [euler.2.cos(), -euler.2.sin(), 0.0, 0.0],
                    [euler.2.sin(), euler.2.cos(), 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
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
