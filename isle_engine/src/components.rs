use isle_ecs::prelude::Component;
use isle_math::{rotation::Rotation, vector::d3::Vec3};

#[derive(Component)]
pub struct Transform {
    position: Vec3,
    orientation: Rotation,
    scale: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, orientation: Rotation, scale: Vec3) -> Self {
        Self {
            position,
            orientation,
            scale,
        }
    }

    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            orientation: Rotation::quaternion_identity(),
            scale: Vec3::IDENTITY,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn orientation(&self) -> Rotation {
        self.orientation
    }

    pub fn scale_by(&self) -> Vec3 {
        self.scale
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        self.orientation = rotation * self.orientation;
    }

    pub fn scale(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    pub fn set_translation(&mut self, translation: Vec3) {
        self.position = translation;
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.orientation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }
}
