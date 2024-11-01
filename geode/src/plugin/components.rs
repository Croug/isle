use isle_ecs::prelude::Component;
use isle_math::{rotation::Angle, vector::d3::Vec3};

use crate::{camera::CameraProjection, material::IntoBindGroup};

#[derive(Component)]
pub struct Camera {
    pub projection: CameraProjection,
    pub znear: f32,
    pub zfar: f32,
}

#[derive(Component)]
pub struct Mesh {
    geometry: usize,
    instance: usize,
}

#[derive(Component)]
pub struct Material {
    material: usize,
    instance: usize,
    settings: Box<dyn IntoBindGroup>,
}

#[derive(Component)]
pub struct PointLight {
    color: Vec3,
    intensity: f32,
}

#[derive(Component)]
pub struct SpotLight {
    color: Vec3,
    intensity: f32,
    outer: Angle,
    inner: Angle,
}