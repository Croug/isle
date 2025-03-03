use std::{any::{Any, TypeId}, error::Error, path::Path};

use isle_ecs::{prelude::Component, world::World};
use rustc_hash::FxHashMap;

use crate::{executor::Executor, plugin::EngineHook, schedule::Scheduler};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct StageInfo {
    name: &'static str,
    load_impact: f32,
}

pub trait ProtocolHandler {
    type Asset;
    type Resource;
    const MIME_TYPES: &'static [&'static str];
    const STAGES: &'static [StageInfo];

    fn init_asset(resource: &Self::Resource, source: &Path) -> Result<()>;
    fn load_asset(resource: &Self::Resource, asset: &mut Self::Asset, stage: usize) -> Result<()>;
}

pub trait ProtocolHandlerAny {
    fn resource_type(&self) -> TypeId;
    fn asset_type(&self) -> TypeId;
    fn mime_types(&self) -> &'static [&'static str];
    fn stages(&self) -> &'static [StageInfo];
    fn init_asset(&self, resource: &dyn Any, source: &Path) -> Result<()>;
    fn load_asset(&self, resource: &dyn Any, asset: &mut dyn Any, stage: usize) -> Result<()>;
}

impl<T: ProtocolHandler<Asset = A, Resource=R>, A: 'static, R: 'static> ProtocolHandlerAny for T
{
    fn resource_type(&self) -> TypeId {
        TypeId::of::<R>()
    }
    fn asset_type(&self) -> TypeId {
        TypeId::of::<A>()
    }
    fn mime_types(&self) -> &'static [&'static str] {
        T::MIME_TYPES
    }
    fn stages(&self) -> &'static [StageInfo] {
        T::STAGES
    }
    fn init_asset(&self, resource: &dyn Any, source: &Path) -> Result<()> {
        let resource = resource.downcast_ref().unwrap();
        T::init_asset(&resource, source)
    }
    fn load_asset(&self, resource: &dyn Any, asset: &mut dyn Any, stage: usize) -> Result<()> {
        let resource = resource.downcast_ref().unwrap();
        let asset = asset.downcast_mut::<A>().unwrap();
        T::load_asset(&resource, asset, stage)
    }
}

#[derive(Default)]
pub struct AssetManager {
    handlers: FxHashMap<TypeId, Box<dyn ProtocolHandlerAny>>,
}

impl<S: Scheduler, E: Executor> EngineHook<S,E> for AssetManager {
    fn pre_run(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {
        self.handlers.iter().for_each(|(type_id, handler)| {
            let resource_type = handler.resource_type();
            let resource = world.get_resource_by_id(&resource_type).unwrap();
            let max_stage = handler.stages().len();
            world.get_components_by_id_mut(type_id).iter_mut().for_each(|asset| {
                handler.load_asset(resource, asset, max_stage).unwrap();
            });
        });
    }
}
