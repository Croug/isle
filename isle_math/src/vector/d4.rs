use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl Vec4 {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0, 0.0);
    pub const IDENTITY: Self = Self(1.0, 1.0, 1.0, 1.0);
    pub const UP: Self = Self(0.0, 1.0, 0.0, 0.0);
    pub const FORWARD: Self = Self(0.0, 0.0, 1.0, 0.0);
    pub const RIGHT: Self = Self(1.0, 0.0, 0.0, 0.0);

    pub fn dot(&self, other: &Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn mag(&self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn norm(&self) -> Self {
        self / self.mag()
    }
}

crate::macros::impl_swizzle!(Vec4, 4);

impl Div for Vec4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2, self.3 / rhs.3)
    }
}

impl Div for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Self) -> Self::Output {
        (*self).div(*rhs)
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

impl Div<f32> for &Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2, self.3 * rhs.3)
    }
}

impl Mul for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Self) -> Self::Output {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Mul<f32> for &Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2, self.3 + rhs.3)
    }
}

impl Add for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Self) -> Self::Output {
        (*self).add(*rhs)
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs, self.2 + rhs, self.3 + rhs)
    }
}

impl Add<f32> for &Vec4 {
    type Output = Vec4;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2, self.3 - rhs.3)
    }
}

impl Sub for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        (*self).sub(*rhs)
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs, self.2 - rhs, self.3 - rhs)
    }
}

impl Sub<f32> for &Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl Neg for Vec4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Neg for &Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        (*self).neg()
    }
}
