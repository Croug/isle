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