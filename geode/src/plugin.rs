use isle_engine::{executor::Executor, flow::Flow, schedule::Scheduler};

pub mod components;
pub mod systems;

pub fn geode_plugin<S: Scheduler, E: Executor>(flow: Flow<S, E>) -> Flow<S, E> {
    flow
}