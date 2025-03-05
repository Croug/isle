use std::{any::{Any, TypeId}, error::Error, path::Path, sync::{LazyLock, Mutex, OnceLock, RwLock}};

use isle_ecs::{prelude::Component, world::{AssetManagerExt, World}};
use rustc_hash::FxHashMap;

use crate::{components, executor::Executor, plugin::EngineHook, schedule::Scheduler};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct StageInfo {
    pub name: &'static str,
    pub load_impact: f32,
}

pub trait ProtocolHandler {
    type Asset;
    type Resource;
    const MIME_TYPES: &'static [&'static str];
    const STAGES: &'static [StageInfo];

    fn init_asset(resource: &mut Self::Resource, source: &Path) -> Result<()>;
    fn load_asset(resource: &mut Self::Resource, asset: &Self::Asset, stage: usize) -> Result<()>;
}

pub trait ProtocolHandlerAny {
    fn resource_type(&self) -> TypeId;
    fn asset_type(&self) -> TypeId;
    fn mime_types(&self) -> &'static [&'static str];
    fn stages(&self) -> &'static [StageInfo];
    fn init_asset(&self, resource: &mut dyn Any, source: &Path) -> Result<()>;
    fn load_asset(&self, resource: &mut dyn Any, asset: &dyn Any, stage: usize) -> Result<()>;
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
    fn init_asset(&self, resource: &mut dyn Any, source: &Path) -> Result<()> {
        let resource = resource.downcast_mut().unwrap();
        T::init_asset(resource, source)
    }
    fn load_asset(&self, resource: &mut dyn Any, asset: &dyn Any, stage: usize) -> Result<()> {
        let resource = resource.downcast_mut().unwrap();
        let asset = asset.downcast_ref::<A>().unwrap();
        T::load_asset(resource, asset, stage)
    }
}

static HANDLERS: LazyLock<Mutex<FxHashMap<TypeId, Box<dyn ProtocolHandlerAny + Send + Sync>>>> = LazyLock::new(|| Mutex::default());

#[derive(Default)]
pub struct AssetManager {
    handlers: FxHashMap<TypeId, Box<dyn ProtocolHandlerAny>>,
}

impl AssetManager {
    pub fn register_handler<T: ProtocolHandler + Send + Sync + 'static>(handler: T) {
        let mut handlers = HANDLERS.lock().unwrap();
        handlers.insert(TypeId::of::<T::Resource>(), Box::new(handler));
    }
}

impl<S: Scheduler, E: Executor> EngineHook<S,E> for AssetManager {
    fn pre_run(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {
        self.handlers.iter().for_each(|(type_id, handler)| {
            let resource_type = handler.resource_type();
            let max_stage = handler.stages().len();

            let Some((resource, mut components)) = world.get_res_and_components(&resource_type, &type_id) else {
                return;
            };

            components.iter().for_each(|asset| {
                handler.load_asset(resource, *asset, max_stage).unwrap();
            });
        });
    }
}
