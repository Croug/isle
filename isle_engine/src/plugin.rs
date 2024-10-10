use std::{cell::UnsafeCell, sync::atomic::Ordering};
use isle_ecs::{ecs::ECS, world::World};

use crate::{executor::Executor, flow::FlowBuilder, schedule::{Schedule, Scheduler}};

#[allow(unused_variables)]
pub trait EngineHook<S: Scheduler, E: Executor> {
    fn setup(&mut self, flow_builder: FlowBuilder<S, E>) -> FlowBuilder<S, E> {
        flow_builder
    }
    fn pre_run(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {}
    fn post_run(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {}
    fn pre_render(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {}
    fn render(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {}
    fn post_render(&mut self, world: &mut World, scheduler: &mut S, executor: &mut E) {}
}

#[allow(unused_variables)]
#[cfg(feature = "async")]
pub trait AsyncEngineHook<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    fn setup(
        &mut self,
        flow_builder: FlowBuilder<T, W, S, E>,
    ) -> impl std::future::Future<Output = FlowBuilder<T, W, S, E>> + Send {
        async { flow_builder }
    }
    fn pre_run(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn run(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn post_run(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn pre_render(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn render(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn post_render(
        &mut self,
        world: &mut W,
        scheduler: &mut S,
        executor: &mut E,
    ) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}

impl crate::schedule::Schedule for isle_ecs::schedule::Schedule {
    fn get_next(&self) -> Option<usize> {
        let next = self.next.fetch_add(1, Ordering::SeqCst);
        self.systems.get(next).copied()
    }
    fn report_done(&self, _item: usize) {}
}

impl crate::executor::Executor for isle_ecs::executor::Executor {
    fn run<T: Schedule + Sized>(&mut self, ecs: &UnsafeCell<ECS>, world: &UnsafeCell<World>, schedule: &T) {
        for system_id in schedule.iter() {
            let ecs = unsafe { &mut *ecs.get() };
            ecs.run_system_by_id(system_id, world);
        }
    }
}

impl crate::schedule::Scheduler
    for isle_ecs::schedule::Scheduler
{
    fn get_schedule(
        &mut self,
        _world: &UnsafeCell<World>,
        ecs: &UnsafeCell<ECS>,
    ) -> impl crate::schedule::Schedule + 'static {
        let ecs = unsafe { &*ecs.get() };
        isle_ecs::schedule::Schedule::from_ecs(ecs)
    }
}

type Flow = FlowBuilder<isle_ecs::schedule::Scheduler, isle_ecs::executor::Executor>;

pub fn default_plugins(flow: Flow) -> Flow {
    flow
    .with_executor(isle_ecs::executor::Executor)
    .with_scheduler(isle_ecs::schedule::Scheduler)
}
