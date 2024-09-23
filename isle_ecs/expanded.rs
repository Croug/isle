#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod world {
    use std::{
        any::{Any, TypeId},
        hash::{BuildHasher, Hash, Hasher},
        cell::RefCell,
    };
    use hashbrown::HashMap;
    use isle_engine::entity::Entity;
    use super::component::Component;
    pub struct World {
        components: HashMap<TypeId, HashMap<Entity, Box<dyn Any>>>,
        resources: HashMap<TypeId, Box<dyn Any>>,
    }
    impl World {
        pub fn new() -> Self {
            Self {
                components: HashMap::new(),
                resources: HashMap::new(),
            }
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
        }
        pub fn get_component<T: Component>(&self, entity: &Entity) -> Option<&T> {
            self.components.get(&TypeId::of::<T>())?.get(entity)?.downcast_ref::<T>()
        }
        /// # Safety
        /// This function must only be called in a single threaded environment or
        /// from within a scheduled context.
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
    fn hash_key<K: Hash, V>(key: &K, map: &HashMap<K, V>) -> u64 {
        let mut hasher = map.hasher().build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }
}
pub mod component {
    use std::any::Any;
    pub trait Component: Any + 'static {}
    impl<T> Component for T
    where
        T: Any + 'static,
    {}
}
pub mod ecs {
    use std::marker::PhantomData;
    use super::world::World;
    use isle_engine::Scheduler;
    pub struct ECS {
        systems: Vec<Box<dyn System>>,
        world: World,
    }
    impl ECS {
        pub fn new() -> Self {
            Self {
                systems: Vec::new(),
                world: World::new(),
            }
        }
        pub fn add_system<I, S: System + 'static>(
            &mut self,
            system: impl IntoSystem<I, System = S>,
        ) {
            self.systems.push(Box::new(system.into_system()));
        }
    }
    impl Scheduler for ECS {
        fn spin(&mut self) -> () {
            for system in self.systems.iter_mut() {
                system.run(&mut self.world)
            }
        }
    }
    pub trait System {
        fn run(&mut self, world: &mut World);
    }
    pub trait IntoSystem<Input> {
        type System: System;
        fn into_system(self) -> Self::System;
    }
    pub trait SystemParam {
        type Item<'new>;
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w>;
    }
    pub struct StoredSystem<Input, F> {
        f: F,
        marker: PhantomData<fn() -> Input>,
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2> SystemParam for (T1, T2)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
    {
        type Item<'new> = (T1::Item<'new>, T2::Item<'new>);
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            (T1, T2)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3> SystemParam for (T1, T2, T3)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
    {
        type Item<'new> = (T1::Item<'new>, T2::Item<'new>, T3::Item<'new>);
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            (T1, T2, T3)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3, T4> SystemParam for (T1, T2, T3, T4)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
        for<'a> T4: SystemParam<Item<'a> = T4>,
    {
        type Item<'new> = (
            T1::Item<'new>,
            T2::Item<'new>,
            T3::Item<'new>,
            T4::Item<'new>,
        );
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            (T1, T2, T3, T4)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3, T4, T5> SystemParam for (T1, T2, T3, T4, T5)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
        for<'a> T4: SystemParam<Item<'a> = T4>,
        for<'a> T5: SystemParam<Item<'a> = T5>,
    {
        type Item<'new> = (
            T1::Item<'new>,
            T2::Item<'new>,
            T3::Item<'new>,
            T4::Item<'new>,
            T5::Item<'new>,
        );
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            (T1, T2, T3, T4, T5)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3, T4, T5, T6> SystemParam for (T1, T2, T3, T4, T5, T6)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
        for<'a> T4: SystemParam<Item<'a> = T4>,
        for<'a> T5: SystemParam<Item<'a> = T5>,
        for<'a> T6: SystemParam<Item<'a> = T6>,
    {
        type Item<'new> = (
            T1::Item<'new>,
            T2::Item<'new>,
            T3::Item<'new>,
            T4::Item<'new>,
            T5::Item<'new>,
            T6::Item<'new>,
        );
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            (T1, T2, T3, T4, T5, T6)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3, T4, T5, T6, T7> SystemParam for (T1, T2, T3, T4, T5, T6, T7)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
        for<'a> T4: SystemParam<Item<'a> = T4>,
        for<'a> T5: SystemParam<Item<'a> = T5>,
        for<'a> T6: SystemParam<Item<'a> = T6>,
        for<'a> T7: SystemParam<Item<'a> = T7>,
    {
        type Item<'new> = (
            T1::Item<'new>,
            T2::Item<'new>,
            T3::Item<'new>,
            T4::Item<'new>,
            T5::Item<'new>,
            T6::Item<'new>,
            T7::Item<'new>,
        );
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            let T7 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T7::from_world(world)
            };
            (T1, T2, T3, T4, T5, T6, T7)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1, T2, T3, T4, T5, T6, T7, T8> SystemParam for (T1, T2, T3, T4, T5, T6, T7, T8)
    where
        for<'a> T1: SystemParam<Item<'a> = T1>,
        for<'a> T2: SystemParam<Item<'a> = T2>,
        for<'a> T3: SystemParam<Item<'a> = T3>,
        for<'a> T4: SystemParam<Item<'a> = T4>,
        for<'a> T5: SystemParam<Item<'a> = T5>,
        for<'a> T6: SystemParam<Item<'a> = T6>,
        for<'a> T7: SystemParam<Item<'a> = T7>,
        for<'a> T8: SystemParam<Item<'a> = T8>,
    {
        type Item<'new> = (
            T1::Item<'new>,
            T2::Item<'new>,
            T3::Item<'new>,
            T4::Item<'new>,
            T5::Item<'new>,
            T6::Item<'new>,
            T7::Item<'new>,
            T8::Item<'new>,
        );
        fn from_world<'w>(world: &'w mut World) -> Self::Item<'w> {
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            let T7 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T7::from_world(world)
            };
            let T8 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T8::from_world(world)
            };
            (T1, T2, T3, T4, T5, T6, T7, T8)
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F> System for StoredSystem<(), F>
    where
        for<'a, 'b> &'a mut F: FnMut() + FnMut(),
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner(mut f: impl FnMut()) {
                f()
            }
            call_inner(&mut self.f);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1> System for StoredSystem<(T1), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
        T1: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1>(mut f: impl FnMut(T1), T1: T1) {
                f(T1)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            call_inner(&mut self.f, T1);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2> System for StoredSystem<(T1, T2), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2)
            + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
        T1: SystemParam,
        T2: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2>(mut f: impl FnMut(T1, T2), T1: T1, T2: T2) {
                f(T1, T2)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            call_inner(&mut self.f, T1, T2);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3> System for StoredSystem<(T1, T2, T3), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3>(
                mut f: impl FnMut(T1, T2, T3),
                T1: T1,
                T2: T2,
                T3: T3,
            ) {
                f(T1, T2, T3)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3, T4> System for StoredSystem<(T1, T2, T3, T4), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3, T4>(
                mut f: impl FnMut(T1, T2, T3, T4),
                T1: T1,
                T2: T2,
                T3: T3,
                T4: T4,
            ) {
                f(T1, T2, T3, T4)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3, T4);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3, T4, T5> System for StoredSystem<(T1, T2, T3, T4, T5), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3, T4, T5>(
                mut f: impl FnMut(T1, T2, T3, T4, T5),
                T1: T1,
                T2: T2,
                T3: T3,
                T4: T4,
                T5: T5,
            ) {
                f(T1, T2, T3, T4, T5)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3, T4, T5);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3, T4, T5, T6> System for StoredSystem<(T1, T2, T3, T4, T5, T6), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3, T4, T5, T6>(
                mut f: impl FnMut(T1, T2, T3, T4, T5, T6),
                T1: T1,
                T2: T2,
                T3: T3,
                T4: T4,
                T5: T5,
                T6: T6,
            ) {
                f(T1, T2, T3, T4, T5, T6)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3, T4, T5, T6);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3, T4, T5, T6, T7> System
    for StoredSystem<(T1, T2, T3, T4, T5, T6, T7), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6, T7)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
                <T7 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
        T7: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3, T4, T5, T6, T7>(
                mut f: impl FnMut(T1, T2, T3, T4, T5, T6, T7),
                T1: T1,
                T2: T2,
                T3: T3,
                T4: T4,
                T5: T5,
                T6: T6,
                T7: T7,
            ) {
                f(T1, T2, T3, T4, T5, T6, T7)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            let T7 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T7::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3, T4, T5, T6, T7);
        }
    }
    #[allow(non_snake_case, unused)]
    impl<F, T1, T2, T3, T4, T5, T6, T7, T8> System
    for StoredSystem<(T1, T2, T3, T4, T5, T6, T7, T8), F>
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6, T7, T8)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
                <T7 as SystemParam>::Item<'b>,
                <T8 as SystemParam>::Item<'b>,
            ),
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
        T7: SystemParam,
        T8: SystemParam,
    {
        fn run(&mut self, world: &mut World) {
            fn call_inner<T1, T2, T3, T4, T5, T6, T7, T8>(
                mut f: impl FnMut(T1, T2, T3, T4, T5, T6, T7, T8),
                T1: T1,
                T2: T2,
                T3: T3,
                T4: T4,
                T5: T5,
                T6: T6,
                T7: T7,
                T8: T8,
            ) {
                f(T1, T2, T3, T4, T5, T6, T7, T8)
            }
            let T1 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T1::from_world(world)
            };
            let T2 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T2::from_world(world)
            };
            let T3 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T3::from_world(world)
            };
            let T4 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T4::from_world(world)
            };
            let T5 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T5::from_world(world)
            };
            let T6 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T6::from_world(world)
            };
            let T7 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T7::from_world(world)
            };
            let T8 = {
                let world: &mut World = unsafe { &mut *(world as *mut World) };
                T8::from_world(world)
            };
            call_inner(&mut self.f, T1, T2, T3, T4, T5, T6, T7, T8);
        }
    }
    impl<F> IntoSystem<()> for F
    where
        for<'a, 'b> &'a mut F: FnMut() + FnMut(),
    {
        type System = StoredSystem<(), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<F, T1: SystemParam> IntoSystem<(T1,)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
    {
        type System = StoredSystem<(T1,), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<F, T1: SystemParam, T2: SystemParam> IntoSystem<(T1, T2)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2)
            + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
    {
        type System = StoredSystem<(T1, T2), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<F, T1: SystemParam, T2: SystemParam, T3: SystemParam> IntoSystem<(T1, T2, T3)>
    for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<
        F,
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
    > IntoSystem<(T1, T2, T3, T4)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3, T4), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<
        F,
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
    > IntoSystem<(T1, T2, T3, T4, T5)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3, T4, T5), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<
        F,
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
    > IntoSystem<(T1, T2, T3, T4, T5, T6)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3, T4, T5, T6), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<
        F,
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
        T7: SystemParam,
    > IntoSystem<(T1, T2, T3, T4, T5, T6, T7)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6, T7)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
                <T7 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3, T4, T5, T6, T7), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
    impl<
        F,
        T1: SystemParam,
        T2: SystemParam,
        T3: SystemParam,
        T4: SystemParam,
        T5: SystemParam,
        T6: SystemParam,
        T7: SystemParam,
        T8: SystemParam,
    > IntoSystem<(T1, T2, T3, T4, T5, T6, T7, T8)> for F
    where
        for<'a, 'b> &'a mut F: FnMut(T1, T2, T3, T4, T5, T6, T7, T8)
            + FnMut(
                <T1 as SystemParam>::Item<'b>,
                <T2 as SystemParam>::Item<'b>,
                <T3 as SystemParam>::Item<'b>,
                <T4 as SystemParam>::Item<'b>,
                <T5 as SystemParam>::Item<'b>,
                <T6 as SystemParam>::Item<'b>,
                <T7 as SystemParam>::Item<'b>,
                <T8 as SystemParam>::Item<'b>,
            ),
    {
        type System = StoredSystem<(T1, T2, T3, T4, T5, T6, T7, T8), Self>;
        fn into_system(self) -> Self::System {
            StoredSystem {
                f: self,
                marker: Default::default(),
            }
        }
    }
}
pub mod query {
    use std::{marker::PhantomData, any::TypeId};
    use isle_engine::entity::Entity;
    use crate::{ecs::SystemParam, world::World, component::Component};
    pub enum RefType {
        Immutable,
        Mutable,
        OptionalImmutable,
        OptionalMutable,
    }
    impl RefType {
        pub fn make_optional(self) -> Self {
            match self {
                Self::Immutable => Self::OptionalImmutable,
                Self::Mutable => Self::OptionalMutable,
                _ => self,
            }
        }
    }
    pub struct Query<T, V = ()>
    where
        T: QueryParam,
        V: ReadOnlyQueryParam,
    {
        marker: PhantomData<(T, V)>,
    }
    pub struct With<T>(PhantomData<T>);
    pub struct Without<T>(PhantomData<T>);
    pub trait QueryParam {
        type Item<'new>;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w>;
        fn get_components() -> Vec<(TypeId, RefType)>;
    }
    pub trait ReadOnlyQueryParam {
        fn get_components() -> Vec<(TypeId, RefType)>;
    }
    impl<T: Component + 'static> ReadOnlyQueryParam for With<T> {
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(TypeId::of::<T>(), RefType::Immutable)]),
            )
        }
    }
    impl<T: Component + 'static> ReadOnlyQueryParam for Without<T> {
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(TypeId::of::<T>(), RefType::Immutable)]),
            )
        }
    }
    impl<T: Component> QueryParam for &T {
        type Item<'new> = &'new T;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            unsafe { world.get_component_mut::<T>(entity).unwrap() }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(TypeId::of::<T>(), RefType::Immutable)]),
            )
        }
    }
    impl<T: Component> QueryParam for &mut T {
        type Item<'new> = &'new mut T;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let comp = unsafe { world.get_component_mut::<T>(entity).unwrap() };
            &mut *comp
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(TypeId::of::<T>(), RefType::Mutable)]),
            )
        }
    }
    impl<T: Component> QueryParam for Option<&T> {
        type Item<'new> = Option<&'new T>;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            world.get_component::<T>(entity)
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    (TypeId::of::<T>(), RefType::OptionalImmutable),
                ]),
            )
        }
    }
    impl<T: Component> QueryParam for Option<&mut T> {
        type Item<'new> = Option<&'new mut T>;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            unsafe { world.get_component_mut::<T>(entity) }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(TypeId::of::<T>(), RefType::OptionalMutable)]),
            )
        }
    }
    impl<T, V> SystemParam for Query<T, V>
    where
        T: QueryParam,
        V: ReadOnlyQueryParam,
    {
        type Item<'new> = Query<T, V>;
        fn from_world(_world: &mut World) -> Self {
            Self { marker: PhantomData }
        }
    }
    #[allow(non_snake_case, unused)]
    impl QueryParam for () {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe { () }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1: QueryParam, T2: QueryParam> QueryParam for (T1, T2)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1: QueryParam, T2: QueryParam, T3: QueryParam> QueryParam for (T1, T2, T3)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1: QueryParam, T2: QueryParam, T3: QueryParam, T4: QueryParam> QueryParam
    for (T1, T2, T3, T4)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
        for<'a> T4: QueryParam<Item<'a> = T4>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                    T4::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: QueryParam,
        T2: QueryParam,
        T3: QueryParam,
        T4: QueryParam,
        T5: QueryParam,
    > QueryParam for (T1, T2, T3, T4, T5)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
        for<'a> T4: QueryParam<Item<'a> = T4>,
        for<'a> T5: QueryParam<Item<'a> = T5>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                    T4::from_world(entity, &mut *world),
                    T5::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: QueryParam,
        T2: QueryParam,
        T3: QueryParam,
        T4: QueryParam,
        T5: QueryParam,
        T6: QueryParam,
    > QueryParam for (T1, T2, T3, T4, T5, T6)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
        for<'a> T4: QueryParam<Item<'a> = T4>,
        for<'a> T5: QueryParam<Item<'a> = T5>,
        for<'a> T6: QueryParam<Item<'a> = T6>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                    T4::from_world(entity, &mut *world),
                    T5::from_world(entity, &mut *world),
                    T6::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: QueryParam,
        T2: QueryParam,
        T3: QueryParam,
        T4: QueryParam,
        T5: QueryParam,
        T6: QueryParam,
        T7: QueryParam,
    > QueryParam for (T1, T2, T3, T4, T5, T6, T7)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
        for<'a> T4: QueryParam<Item<'a> = T4>,
        for<'a> T5: QueryParam<Item<'a> = T5>,
        for<'a> T6: QueryParam<Item<'a> = T6>,
        for<'a> T7: QueryParam<Item<'a> = T7>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                    T4::from_world(entity, &mut *world),
                    T5::from_world(entity, &mut *world),
                    T6::from_world(entity, &mut *world),
                    T7::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components.extend(T7::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: QueryParam,
        T2: QueryParam,
        T3: QueryParam,
        T4: QueryParam,
        T5: QueryParam,
        T6: QueryParam,
        T7: QueryParam,
        T8: QueryParam,
    > QueryParam for (T1, T2, T3, T4, T5, T6, T7, T8)
    where
        for<'a> T1: QueryParam<Item<'a> = T1>,
        for<'a> T2: QueryParam<Item<'a> = T2>,
        for<'a> T3: QueryParam<Item<'a> = T3>,
        for<'a> T4: QueryParam<Item<'a> = T4>,
        for<'a> T5: QueryParam<Item<'a> = T5>,
        for<'a> T6: QueryParam<Item<'a> = T6>,
        for<'a> T7: QueryParam<Item<'a> = T7>,
        for<'a> T8: QueryParam<Item<'a> = T8>,
    {
        type Item<'new> = Self;
        fn from_world<'w>(entity: &Entity, world: &'w mut World) -> Self::Item<'w> {
            let world: *mut World = world;
            unsafe {
                (
                    T1::from_world(entity, &mut *world),
                    T2::from_world(entity, &mut *world),
                    T3::from_world(entity, &mut *world),
                    T4::from_world(entity, &mut *world),
                    T5::from_world(entity, &mut *world),
                    T6::from_world(entity, &mut *world),
                    T7::from_world(entity, &mut *world),
                    T8::from_world(entity, &mut *world),
                )
            }
        }
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components.extend(T7::get_components());
            components.extend(T8::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl ReadOnlyQueryParam for () {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<T1: ReadOnlyQueryParam, T2: ReadOnlyQueryParam> ReadOnlyQueryParam
    for (T1, T2) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
        T4: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3, T4) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
        T4: ReadOnlyQueryParam,
        T5: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3, T4, T5) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
        T4: ReadOnlyQueryParam,
        T5: ReadOnlyQueryParam,
        T6: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3, T4, T5, T6) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
        T4: ReadOnlyQueryParam,
        T5: ReadOnlyQueryParam,
        T6: ReadOnlyQueryParam,
        T7: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3, T4, T5, T6, T7) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components.extend(T7::get_components());
            components
        }
    }
    #[allow(non_snake_case, unused)]
    impl<
        T1: ReadOnlyQueryParam,
        T2: ReadOnlyQueryParam,
        T3: ReadOnlyQueryParam,
        T4: ReadOnlyQueryParam,
        T5: ReadOnlyQueryParam,
        T6: ReadOnlyQueryParam,
        T7: ReadOnlyQueryParam,
        T8: ReadOnlyQueryParam,
    > ReadOnlyQueryParam for (T1, T2, T3, T4, T5, T6, T7, T8) {
        fn get_components() -> Vec<(TypeId, RefType)> {
            let mut components = Vec::new();
            components.extend(T1::get_components());
            components.extend(T2::get_components());
            components.extend(T3::get_components());
            components.extend(T4::get_components());
            components.extend(T5::get_components());
            components.extend(T6::get_components());
            components.extend(T7::get_components());
            components.extend(T8::get_components());
            components
        }
    }
}
pub mod prelude {
    pub use super::ecs::{System, IntoSystem, SystemParam};
}
