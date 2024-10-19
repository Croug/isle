use std::ops::{Div, Mul, Add, Sub, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub f32, pub f32);

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Self) -> Self::Output {
        (*self).div(*rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

impl Div<f32> for &Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Self) -> Self::Output {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<f32> for &Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        (*self).add(*rhs)
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs, self.1 + rhs)
    }
}

impl Add<f32> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        (*self).sub(*rhs)
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs, self.1 - rhs)
    }
}

impl Sub<f32> for &Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Neg for &Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        (*self).neg()
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self(x, y)
    }
}

impl Into<(f32, f32)> for Vec2 {
    fn into(self) -> (f32, f32) {
        (self.0, self.1)
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from((x, y): (u32, u32)) -> Self {
        Self(x as f32, y as f32)
    }
}

impl Into<(u32, u32)> for Vec2 {
    fn into(self) -> (u32, u32) {
        (self.0 as u32, self.1 as u32)
    }
}