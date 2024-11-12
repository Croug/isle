use isle_ecs::prelude::Component;
use isle_math::{rotation::Angle, vector::d3::Vec3};

use crate::camera::{CameraCreationSettings, CameraProjection};

#[derive(Component)]
pub struct Camera {
    pub(crate) id: usize,
    pub(crate) projection: CameraProjection,
    pub(crate) znear: f32,
    pub(crate) zfar: f32,
    pub(crate) dirty: bool,
}

impl Camera {
    pub fn new(settings: &CameraCreationSettings) -> Self {
        Camera {
            id: 0,
            projection: settings.projection,
            znear: settings.znear,
            zfar: settings.zfar,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Mesh {
    pub(crate) geometry: usize,
    pub(crate) instance: Option<usize>,
    pub(crate) dirty: bool,
}

impl Mesh {
    pub fn new(geometry: usize) -> Self {
        Mesh {
            geometry,
            instance: None,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Material {
    pub(crate) material: usize,
    pub(crate) instance: usize,
}

impl Material {
    pub fn new(material: usize, instance: usize) -> Self {
        Material {
            material,
            instance,
        }
    }
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