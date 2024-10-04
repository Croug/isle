use std::cell::UnsafeCell;

impl isle_engine::world::World for crate::world::World {
    fn new() -> Self {
        Self::new()
    }
}

type System = Box<dyn crate::ecs::System + 'static>;

impl isle_engine::schedule::Scheduler<crate::world::World, System> for crate::schedule::Scheduler {
    fn get_schedule(&mut self, _world: &UnsafeCell<crate::world::World>) -> impl isle_engine::schedule::Schedule<Item = System> {
        crate::schedule::Schedule {}
    }
}

impl isle_engine::schedule::Schedule for crate::schedule::Schedule {
    type Item = System;

    fn get_next(&self) -> Option<Self::Item> {
        None
    }
    fn report_done(&self, _item: Self::Item) {
        
    }
}

impl isle_engine::executor::Executor<System, crate::world::World, crate::schedule::Scheduler> for crate::executor::Executor {
    fn run(&mut self, _world: &UnsafeCell<crate::world::World>, _scheduler: &mut crate::schedule::Scheduler) {

    }
}

type FlowBuilder = isle_engine::flow::FlowBuilder<System, crate::world::World, crate::schedule::Scheduler, crate::executor::Executor>;

pub fn ecs_plugin(flow_builder: FlowBuilder) -> FlowBuilder {
    flow_builder
        .with_world(crate::world::World::new())
        .with_scheduler(crate::schedule::Scheduler {})
        .with_executor(crate::executor::Executor {})
}