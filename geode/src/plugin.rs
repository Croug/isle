use isle_engine::{executor::Executor, flow::Flow, schedule::Scheduler};

pub mod components;
pub mod systems;

pub fn geode_plugin<S: Scheduler, E: Executor>(mut flow: Flow<S, E>) -> Flow<S, E> {
    flow.run_once(systems::setup);

    flow.add_postfix_system(systems::update_cameras);
    flow.add_postfix_system(systems::update_lights);
    flow.add_postfix_system(systems::update_instances);
    flow.add_postfix_system(systems::create_geometries);

    flow
}