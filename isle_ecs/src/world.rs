use std::{
    collections::HashMap,
    any::{Any, TypeId},
};

use isle_engine::entity::Entity;

use super::component::Component;

pub struct World {
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }
    pub fn store_component<T: Component>(&mut self, entity: Entity, component: T){
        self.components
            .entry(TypeId::of::<T>())
            .or_insert(HashMap::new())
            .insert(entity, Box::new(component));
    }

    pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T>{
        self.components
            .get(&TypeId::of::<T>())?
            .get(entity)?
            .downcast_ref::<T>()
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: &Entity) -> Option<&mut T>{
        self.components
            .get_mut(&TypeId::of::<T>())?
            .get_mut(entity)?
            .downcast_mut::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_storage_retrieval() {
        let mut world = World::new();
        let val = 47u32;
        
        world.store_component(Entity(0,0), val);

        let val = world.get_component(&Entity(0,0)).unwrap();

        assert_eq!(47u32, *val);
    }

    #[test]
    fn component_storage_retrieval_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_component(Entity(0,0), val1);
        world.store_component(Entity(0,1), val2);

        let val1 = world.get_component(&Entity(0,0)).unwrap();
        let val2 = world.get_component(&Entity(0,1)).unwrap();

        assert_eq!(47u32, *val1);
        assert_eq!(64u8, *val2);
    }

    #[test]
    fn component_mutate() {
        let mut world = World::new();
        let val = 47u32;

        world.store_component(Entity(0,0), val);

        let val = world.get_component_mut(&Entity(0,0)).unwrap();
        *val = 42u32;

        let val = world.get_component(&Entity(0,0)).unwrap();

        assert_eq!(42u32, *val);
    }

    #[test]
    fn component_mutate_varied() {
        let mut world = World::new();
        let val1 = 47u32;
        let val2 = 64u8;

        world.store_component(Entity(0,0), val1);
        world.store_component(Entity(0,1), val2);

        let val1 = world.get_component_mut(&Entity(0,0)).unwrap();
        *val1 = 42u32;

        let val2 = world.get_component_mut(&Entity(0,1)).unwrap();
        *val2 = 54u8;


        let val1 = world.get_component(&Entity(0,0)).unwrap();
        let val2 = world.get_component(&Entity(0,1)).unwrap();

        assert_eq!(42u32, *val1);
        assert_eq!(54u8, *val2);
    }
}
