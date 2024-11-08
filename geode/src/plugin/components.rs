use isle_ecs::prelude::Component;
use isle_math::{rotation::Angle, vector::d3::Vec3};

use crate::{camera::CameraProjection, material::IntoBindGroup};

#[derive(Component)]
pub struct Camera {
    pub(crate) id: usize,
    pub(crate) projection: CameraProjection,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
    pub(crate) dirty: bool,
}

#[derive(Component)]
pub struct Mesh {
    pub(crate) geometry: usize,
    pub(crate) instance: usize,
}

#[derive(Component)]
pub struct Material {
    pub(crate) material: usize,
    pub(crate) instance: usize,
    pub(crate) settings: Box<dyn IntoBindGroup>,
}

#[derive(Component)]
pub struct PointLight {
    pub(crate) id: usize,
    pub(crate) color: Vec3,
    pub(crate) intensity: f32,
    pub(crate) dirty: bool,
}

#[derive(Component)]
pub struct SpotLight {
    pub(crate) id: usize,
    pub(crate) color: Vec3,
    pub(crate) intensity: f32,
    pub(crate) outer: Angle,
    pub(crate) inner: Angle,
    pub(crate) dirty: bool,
}