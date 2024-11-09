use isle_engine::{executor::Executor, flow::FlowBuilder, schedule::Scheduler};

pub mod components;
pub mod systems;

pub fn geode_plugin<S: Scheduler, E: Executor>(mut flow: FlowBuilder<S, E>) -> FlowBuilder<S, E> {
    flow = flow.with_run_once(systems::setup);

    flow = flow.with_postfix_system(systems::update_cameras);
    flow = flow.with_postfix_system(systems::update_lights);
    flow = flow.with_postfix_system(systems::update_instances);
    flow = flow.with_postfix_system(systems::create_geometries);

    flow
}