use std::ops::Mul;

use quaternion::Quaternion;

use crate::{
    matrix::{Mat3, Mat4, Matrix},
    vector::{d3::Vec3, d4::Vec4},
};

pub mod quaternion {
    use std::{f32::consts::PI, ops::Mul};

    use crate::{
        matrix::{Mat3, Mat4, Matrix},
        vector::{d3::Vec3, d4::Vec4},
    };

    use super::Rotation;

    #[derive(Clone, Copy, Debug)]
    pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

    impl Quaternion {
        pub const IDENTITY: Self = Self(1.0, 0.0, 0.0, 0.0);
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

        pub fn inverse(&self) -> Self {
            Quaternion(self.0, -self.1, -self.2, -self.3)
        }

        pub fn rotate_point(&self, point: Vec3) -> Vec3 {
            let s = self.norm();
            let p = Quaternion(0.0, point.0, point.1, point.2);
            let p = s.inverse() * p * s;
            let v: Vec4 = p.into();

            v.yzw()
        }

        pub fn look_at(source: &Vec3, dest: &Vec3) -> Self {
            let forward = (dest - source).norm();
            let dot = Vec3::FORWARD.dot(&forward);

            const EPSILON: f32 = 0.000001;
            if (dot - -1.).abs() < EPSILON {
                Self(PI, 0.0, 1.0, 0.0)
            } else if (dot - 1.).abs() < EPSILON {
                Self::IDENTITY
            } else {
                let angle = dot.acos();
                let axis = Vec3::FORWARD.cross(&forward).norm();
                
                Self::from_axis_angle(axis, angle)
            }
        }

        pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
            let half_angle = angle / 2.0;
            let half_angle_sin = half_angle.sin();
            Self(
                half_angle.cos(),
                axis.0 * half_angle_sin,
                axis.1 * half_angle_sin,
                axis.2 * half_angle_sin,
            )
        }

        pub fn to_mat3(&self) -> Mat3 {
            let xx = self.1 * self.1;
            let yy = self.2 * self.2;
            let zz = self.3 * self.3;
            let xy = self.1 * self.2;
            let xz = self.1 * self.3;
            let yz = self.2 * self.3;
            let wx = self.0 * self.1;
            let wy = self.0 * self.2;
            let wz = self.0 * self.3;

            Matrix([
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy)],
                [2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx)],
                [2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy)],
            ])
        }

        pub fn to_mat4(&self) -> Mat4 {
            let [[a, b, c], [d, e, f], [g, h, i]] = self.to_mat3().0;

            Matrix([
                [a, b, c, 0.0], // Column 0
                [d, e, f, 0.0], // Column 1
                [g, h, i, 0.0], // Column 2
                [0.0, 0.0, 0.0, 1.0], // Column 3
            ])
        }
    }

    impl Mul for Quaternion {
        type Output = Self;
        
        fn mul(self, rhs: Self) -> Self::Output {
            Quaternion(
                self.0 * rhs.0 - self.1 * rhs.1 - self.2 * rhs.2 - self.3 * rhs.3,
                self.0 * rhs.1 + self.1 * rhs.0 - self.2 * rhs.3 + self.3 * rhs.2,
                self.0 * rhs.2 + self.1 * rhs.3 + self.2 * rhs.0 - self.3 * rhs.1,
                self.0 * rhs.3 - self.1 * rhs.2 + self.2 * rhs.1 + self.3 * rhs.0,
            )
        }
    }

    impl Into<Mat3> for Quaternion {
        fn into(self) -> Mat3 {
            self.to_mat3()
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
    pub fn to_mat3(&self) -> Mat3 {
        match self {
            Rotation::Quaternion(quat) => quat.to_mat3(),
            Rotation::Euler(euler) => {
                // ZYX Yaw-Pitch-Roll
                Matrix([
                    [euler.2.cos(), -euler.2.sin(), 0.0], // Column 0
                    [euler.2.sin(), euler.2.cos(), 0.0],  // Column 1
                    [0.0, 0.0, 1.0],                      // Column 2
                ]) * Matrix([
                    [euler.1.cos(), 0.0, euler.1.sin()],  // Column 0
                    [0.0, 1.0, 0.0],                      // Column 1
                    [-euler.1.sin(), 0.0, euler.1.cos()], // Column 2
                ]) * Matrix([
                    [1.0, 0.0, 0.0],                      // Column 0
                    [0.0, euler.0.cos(), -euler.0.sin()], // Column 1
                    [0.0, euler.0.sin(), euler.0.cos()],  // Column 2
                ])
            }
        }
    }
    pub fn to_mat4(&self) -> Mat4 {
        match self {
            Rotation::Quaternion(quaternion) => quaternion.to_mat4(),
            Rotation::Euler(euler) => {
                // ZYX Yaw-Pitch-Roll
                Matrix([
                    [euler.2.cos(), -euler.2.sin(), 0.0, 0.0], // Column 0
                    [euler.2.sin(), euler.2.cos(), 0.0, 0.0],  // Column 1
                    [0.0, 0.0, 1.0, 0.0],                      // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ]) * Matrix([
                    [euler.1.cos(), 0.0, euler.1.sin(), 0.0],  // Column 0
                    [0.0, 1.0, 0.0, 0.0],                      // Column 1
                    [-euler.1.sin(), 0.0, euler.1.cos(), 0.0], // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ]) * Matrix([
                    [1.0, 0.0, 0.0, 0.0],                      // Column 0
                    [0.0, euler.0.cos(), -euler.0.sin(), 0.0], // Column 1
                    [0.0, euler.0.sin(), euler.0.cos(), 0.0],  // Column 2
                    [0.0, 0.0, 0.0, 1.0],                      // Column 3
                ])
            }
        }
    }

    pub fn to_quat(&self) -> Quaternion {
        match self {
            Self::Quaternion(quat) => *quat,
            Self::Euler(euler) => {
                let Vec3(x, y, z) = euler / 2.;
                let (sx, cx) = x.sin_cos();
                let (sy, cy) = y.sin_cos();
                let (sz, cz) = z.sin_cos();

                Quaternion(
                    cx * cy * cz + sx * sy * sx,
                    sx * cy * cz - cx * sy * sz,
                    cx * sy * cz + sx * cy * sz,
                    cx * cy * sz - sx * sy * cz,
                )
            }
        }
    }

    pub fn to_euler(&self) -> Vec3 {
        match self {
            Self::Euler(euler) => *euler,
            Self::Quaternion(Quaternion(w, i, j, k)) => {
                Vec3(
                    (2. * (w * i + j * k)).atan2(w*w - i*i - j*j + k*k),
                    (2. * (w * j - i * k)).asin(),
                    (2. * (w * k + i * j)).atan2(w*w + i*i - j*j - k*k),
                )
            }
        }
    }
}

impl Into<Quaternion> for &Rotation {
    fn into(self) -> Quaternion {
        self.to_quat()
    }
}

impl Into<Quaternion> for Rotation {
    fn into(self) -> Quaternion {
        self.to_quat()
    }
}

impl Into<Vec3> for &Rotation {
    fn into(self) -> Vec3 {
        self.to_euler()
    }
}

impl Into<Vec3> for Rotation {
    fn into(self) -> Vec3 {
        self.to_euler()
    }
}

impl Into<Mat3> for Rotation {
    fn into(self) -> Mat3 {
        self.to_mat3()
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
