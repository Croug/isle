use std::{
    any::{Any, TypeId},
    cell::UnsafeCell,
    collections::HashSet,
};

use hashbrown::HashMap;

use isle_engine::entity::Entity;

use super::component::Component;

pub struct World {
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
    resources: HashMap<TypeId, Box<dyn Any>>,
    entities: HashMap<Entity, HashSet<TypeId>>,
}

impl World {
    pub fn new() -> UnsafeCell<Self> {
        UnsafeCell::new(Self {
            components: HashMap::new(),
            resources: HashMap::new(),
            entities: HashMap::new(),
        })
    }

    pub fn store_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<T>())?.downcast_ref::<T>()
    }

    pub unsafe fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.resources
            .get_many_unchecked_mut([&TypeId::of::<T>()])?[0]
            .downcast_mut::<T>()
    }

    pub fn store_component<T: Component>(&mut self, entity: Entity, component: T) {
        self.components
            .entry(TypeId::of::<T>())
            .or_insert(HashMap::new())
            .insert(entity, Box::new(component));

        self.entities
            .entry(entity)
            .or_insert(HashSet::new())
            .insert(TypeId::of::<T>());
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .get(entity)?
            .downcast_ref::<T>()
    }

    pub fn get_entities_with_component(&self, type_id: &TypeId) -> Vec<Entity> {
        self.components
            .get(type_id)
            .unwrap()
            .keys()
            .copied()
            .collect()
    }

    pub fn get_entity_components(&self, entity: &Entity) -> HashSet<TypeId> {
        self.entities.get(entity).unwrap().clone()
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
}

// fn hash_key<K: Hash, V>(key: &K, map: &HashMap<K, V>) -> u64 {
//     let mut hasher = map.hasher().build_hasher();
//     key.hash(&mut hasher);
//     hasher.finish()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_storage_retrieval() {
        let world = World::new();
        let world = unsafe { &mut *world.get() };
        let val = 47u32;

        world.store_resource(val);

        let val = world.get_resource::<u32>().unwrap();

        assert_eq!(47u32, *val);
    }

    #[test]
    fn resource_storage_retrieval_varied() {
        let world = World::new();
        let world = unsafe { &mut *world.get() };
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
        let world = World::new();
        let world = unsafe { &mut *world.get() };
        let val = 47u32;

        world.store_resource(val);

        let val = unsafe { world.get_resource_mut::<u32>() }.unwrap();
        *val = 42u32;

        let val = world.get_resource::<u32>().unwrap();

        assert_eq!(42u32, *val);
    }

    #[test]
    fn resource_mutate_varied() {
        let world = World::new();
        let world = unsafe { &mut *world.get() };
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
        let world = World::new();
        let world = unsafe { &mut *world.get() };
        let val = 47u32;

        world.store_component(Entity(0, 0), val);

        let val = world.get_component(&Entity(0, 0)).unwrap();

        assert_eq!(47u32, *val);
    }

    #[test]
    fn component_storage_retrieval_varied() {
        let world = World::new();
        let world = unsafe { &mut *world.get() };
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
        let world = World::new();
        let world = unsafe { &mut *world.get() };
        let val = 47u32;

        world.store_component(Entity(0, 0), val);

        let val = unsafe { world.get_component_mut(&Entity(0, 0)) }.unwrap();
        *val = 42u32;

        let val = world.get_component(&Entity(0, 0)).unwrap();

        assert_eq!(42u32, *val);
    }

    #[test]
    fn component_mutate_varied() {
        let world = World::new();
        let world = unsafe { &mut *world.get() };
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
