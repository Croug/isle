use std::{fmt::Display, marker::PhantomData};

use isle_engine::asset::{self, ProtocolHandler, StageInfo};

use crate::{geometry::{self, GeometryState}, renderer::Renderer};
use super::components::Mesh;

#[derive(Debug)]
pub struct InvalidGeometryError(usize);

impl std::error::Error for InvalidGeometryError {}

impl Display for InvalidGeometryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid geometry index: {}", self.0)
    }
}

#[derive(Default)]
pub struct GeometryHandler<'a>(PhantomData<&'a ()>);

impl<'a> ProtocolHandler for GeometryHandler<'a> {
    type Asset = Mesh;

    type Resource = Renderer<'a>;

    const MIME_TYPES: &'static [&'static str] = &["application/obj"];

    const STAGES: &'static [isle_engine::asset::StageInfo] = &[
        StageInfo {
            name: "Memory",
            load_impact: 0.0,
        },
        StageInfo {
            name: "GPU",
            load_impact: 0.0,
        },
    ];

    fn init_asset(resource: &mut Self::Resource, source: &std::path::Path) -> asset::Result<()> {
        todo!()
    }

    fn load_asset(resource: &mut Self::Resource, asset: &Self::Asset, stage: usize) -> asset::Result<()> {
        let &Mesh {
            geometry: geometry_id,
            ..
        } = asset;

        let geometry = resource.geometry_mut(geometry_id);

        if matches!(geometry.state, GeometryState::Disk) {
            let _ = geometry.load_to_mem();
        }

        if matches!(geometry.state, GeometryState::Memory(_)) {
            resource.upload_geometry(geometry_id);
        }

        Ok(())
    }
}