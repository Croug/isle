use std::{
    collections::HashMap,
    any::{Any, TypeId},
};

use isle_engine::entity::Entity;

use super::component::Component;

pub struct World {
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Component + 'static>>>,
}

impl World {
    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T>{
        self.components
            .get(&TypeId::of::<T>())?
            .get(entity)?
            .as_ref()
            .downcast_ref::<T>()
    }
}
