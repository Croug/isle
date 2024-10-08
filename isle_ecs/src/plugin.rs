use std::{cell::UnsafeCell, sync::atomic::Ordering};

use isle_engine::entity::Entity;

use crate::{ecs::{IntoSystem, System, ECS}, prelude::Component, schedule::Schedule};

impl isle_engine::world::World for crate::world::World {
}

impl isle_engine::schedule::Schedule for crate::schedule::Schedule {
    type Item = usize;

    fn get_next(&self) -> Option<Self::Item> {
        let next = self.next.fetch_add(1, Ordering::SeqCst);
        self.systems.get(next).copied()
    }
    fn report_done(&self, _item: Self::Item) {}
}

impl isle_engine::executor::Executor<usize, crate::world::World> for crate::ecs::ECS {
    fn run(&mut self, world: &UnsafeCell<crate::world::World>, id: usize) {
        self.run_system_by_id(id, world);
    }
}

impl isle_engine::schedule::Scheduler<usize, crate::world::World, ECS> for crate::schedule::Scheduler {
    fn get_schedule(&mut self, _world: &UnsafeCell<crate::world::World>, ecs: &ECS) -> impl isle_engine::schedule::Schedule<Item = usize> + 'static {
        Schedule::from_ecs(ecs)
    }
}

type FlowBuilder = isle_engine::flow::FlowBuilder<usize, crate::world::World, crate::schedule::Scheduler, crate::ecs::ECS>;
type Flow = isle_engine::flow::Flow<usize, crate::world::World, crate::schedule::Scheduler, crate::ecs::ECS>;

pub trait WithECS {
    fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T);
    fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>);
    fn add_resource<T: 'static>(&mut self, resource: T);
}

impl WithECS for Flow {
    fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        let world = unsafe { &mut *self.get_world().get() };
        self.get_executor().add_component(entity, component, world);
    }

    fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.get_executor().add_system(system);
    }

    fn add_resource<T: 'static>(&mut self, resource: T) {
        let world = unsafe { &mut *self.get_world().get() };
        self.get_executor().add_resource(resource, world);
    }
}

pub fn ecs_plugin(flow_builder: FlowBuilder) -> FlowBuilder {
    flow_builder
        .with_world(crate::world::World::new())
        .with_scheduler(crate::schedule::Scheduler)
        .with_executor(crate::ecs::ECS::new())
}