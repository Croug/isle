use isle_ecs::world::World;

use crate::{executor::Executor, flow::FlowBuilder, schedule::Scheduler};

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

type Flow = FlowBuilder<isle_ecs::schedule::Scheduler, isle_ecs::executor::Executor>;

pub fn default_plugins(flow: Flow) -> Flow {
    flow.with_executor(isle_ecs::executor::Executor)
        .with_scheduler(isle_ecs::schedule::Scheduler)
}
