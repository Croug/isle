use std::{
    any::{Any, TypeId},
    collections::HashSet,
    sync::mpsc::{Receiver, Sender},
};

type EntityEvents = EventWriter<EntityEvent>;

pub mod event;

use event::EntityEvent;
use hashbrown::HashMap;
use isle_event::EventWriter;

use crate::{component::Component, entity::Entity};

pub type Command = Box<dyn FnOnce(&mut World)>;

pub struct World {
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
    entities: HashMap<Entity, HashSet<TypeId>>,
    command_sender: Sender<Command>,
    command_receiver: Receiver<Command>,
}

impl World {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = std::sync::mpsc::channel();
        let mut world = Self {
            components: HashMap::new(),
            resources: HashMap::new(),
            entities: HashMap::new(),
            command_sender,
            command_receiver,
        };

        world.store_resource(EntityEvents::new());

        world
    }

    pub fn command_sender(&self) -> &Sender<Command> {
        &self.command_sender
    }

    pub fn store_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub fn get_resource_by_id(&self, type_id: &TypeId) -> Option<&dyn Any> {
        self.resources.get(type_id).map(|r| r.as_ref())
    }

    pub fn apply_commands(&mut self) {
        while let Ok(command) = self.command_receiver.try_recv() {
            command(self);
        }
    }

    pub unsafe fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_many_unchecked_mut([&TypeId::of::<T>()])?[0]
            .downcast_mut::<T>()
    }

    pub fn get_resource_by_id_mut(&mut self, type_id: &TypeId) -> Option<&mut dyn Any> {
        self.resources.get_mut(type_id).map(|r| r.as_mut())
    }

    pub fn store_component<T: Component>(&mut self, entity: Entity, component: T) {
        let mut events = self.get_resource::<EntityEvents>().cloned().unwrap();
        self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new)
            .insert(entity, Box::new(component));

        self.entities
            .entry(entity)
            .or_insert_with(|| {
                events.send(EntityEvent::Created(entity));
                HashSet::new()
            })
            .insert(TypeId::of::<T>());

        events.send(EntityEvent::ComponentAdded(entity, TypeId::of::<T>()));
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .get(entity)?
            .downcast_ref::<T>()
    }

    pub fn get_entities_with_component(&mut self, type_id: &TypeId) -> Vec<Entity> {
        self.components
            .entry(*type_id)
            .or_insert_with(HashMap::new)
            .keys()
            .copied()
            .collect()
    }

    pub fn get_entity_components(&self, entity: &Entity) -> HashSet<TypeId> {
        self.entities.get(entity).unwrap().clone()
    }

    pub fn get_components_by_id(&self, type_id: &TypeId) -> Option<Vec<&dyn Any>> {
        Some(self.components.get(type_id)?.values().map(Box::as_ref).collect())
    }

    /// # Safety
    /// Caller ensures that there are no other mutable references to the component
    pub unsafe fn get_component_mut<T: Component + 'static>(
        &mut self,
        entity: &Entity,
    ) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())?
            .get_many_unchecked_mut([entity])?[0]
            .downcast_mut::<T>()
    }

    pub fn get_components_by_id_mut(&mut self, type_id: &TypeId) -> Option<Vec<&mut dyn Any>> {
        Some(
            self.components
                .get_mut(type_id)?
                .values_mut()
                .map(Box::as_mut)
                .collect(),
        )
    }
}

// fn hash_key<K: Hash, V>(key: &K, map: &HashMap<K, V>) -> u64 {
//     let mut hasher = map.hasher().build_hasher();
//     key.hash(&mut hasher);
//     hasher.finish()
// }

#[cfg(test)]
mod tests {
    use super::*;

    impl Component for u32 {}
    impl Component for u8 {}

    #[test]
    fn resource_storage_retrieval() {
        let mut world = World::new();
        let val = 47u32;

        world.store_resource(val);

        let val = world.get_resource::<u32>().unwrap();

        assert_eq!(47u32, *val);
    }

    #[test]
    fn resource_storage_retrieval_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_resource(val1);
        world.store_resource(val2);

        let val1 = world.get_resource::<u32>().unwrap();
        let val2 = world.get_resource::<u8>().unwrap();

        assert_eq!(47u32, *val1);
        assert_eq!(64u8, *val2);
    }

    #[test]
    fn resource_mutate() {
        let mut world = World::new();
        let val = 47u32;

        world.store_resource(val);

        let val = unsafe { world.get_resource_mut::<u32>() }.unwrap();
        *val = 42u32;

        let val = world.get_resource::<u32>().unwrap();

        assert_eq!(42u32, *val);
    }

    #[test]
    fn resource_mutate_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_resource(val1);
        world.store_resource(val2);

        let val1 = unsafe { world.get_resource_mut::<u32>() }.unwrap();
        *val1 = 42u32;

        let val2 = unsafe { world.get_resource_mut::<u8>() }.unwrap();
        *val2 = 54u8;

        assert_eq!(42u32, *world.get_resource::<u32>().unwrap());
        assert_eq!(54u8, *world.get_resource::<u8>().unwrap());
    }

    #[test]
    fn component_storage_retrieval() {
        let mut world = World::new();
        let val = 47u32;

        world.store_component(Entity(0, 0), val);

        let val = world.get_component(&Entity(0, 0)).unwrap();

        assert_eq!(47u32, *val);
    }

    #[test]
    fn component_storage_retrieval_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_component(Entity(0, 0), val1);
        world.store_component(Entity(0, 1), val2);

        let val1 = world.get_component(&Entity(0, 0)).unwrap();
        let val2 = world.get_component(&Entity(0, 1)).unwrap();

        assert_eq!(47u32, *val1);
        assert_eq!(64u8, *val2);
    }

    #[test]
    fn component_mutate() {
        let mut world = World::new();
        let val = 47u32;

        world.store_component(Entity(0, 0), val);

        let val = unsafe { world.get_component_mut(&Entity(0, 0)) }.unwrap();
        *val = 42u32;

        let val = world.get_component(&Entity(0, 0)).unwrap();

        assert_eq!(42u32, *val);
    }

    #[test]
    fn component_mutate_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_component(Entity(0, 0), val1);
        world.store_component(Entity(0, 1), val2);

        let val1 = unsafe { world.get_component_mut(&Entity(0, 0)) }.unwrap();
        *val1 = 42u32;

        let val2 = unsafe { world.get_component_mut(&Entity(0, 1)) }.unwrap();
        *val2 = 54u8;

        let val1 = world.get_component(&Entity(0, 0)).unwrap();
        let val2 = world.get_component(&Entity(0, 1)).unwrap();

        assert_eq!(42u32, *val1);
        assert_eq!(54u8, *val2);
    }
}

/// For isle engine internal use
pub trait AssetManagerExt {
    fn get_res_and_components(&mut self, resource_id: &TypeId, component_id: &TypeId) -> Option<(&mut dyn Any, Vec<&mut dyn Any>)>;
}

impl AssetManagerExt for World {
    fn get_res_and_components(&mut self, resource_id: &TypeId, component_id: &TypeId) -> Option<(&mut dyn Any, Vec<&mut dyn Any>)> {
        let World{
            ref mut resources,
            ref mut components,
            ..
        } = *self;

        Some(
            (
                resources.get_mut(resource_id)?.as_mut(),
                components.get_mut(component_id)?.values_mut().map(|v| v.as_mut()).collect()
            )
        )
    }
}
