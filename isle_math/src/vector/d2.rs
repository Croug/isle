#[derive(Clone, Copy, Debug)]
pub struct Vec2(pub f32, pub f32);

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