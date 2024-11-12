use isle_engine::{executor::Executor, flow::FlowBuilder, plugin::EngineHook, schedule::Scheduler};
use wgpu::SurfaceError;

use crate::renderer::Renderer;

pub mod components;
pub mod systems;

struct RenderPlugin;

impl<S: Scheduler, E: Executor> EngineHook<S,E> for RenderPlugin {
    fn render(&mut self, world: &mut isle_ecs::world::World, _scheduler: &mut S, _executor: &mut E) {
        let renderer = unsafe { world.get_resource_mut::<Renderer>() }.unwrap();
        if let Err(err) = renderer.render() {
            match err {
                SurfaceError::Lost => {
                    let size = renderer.size();
                    renderer.resize(size);
                }
                e => {
                    eprintln!("Error rendering: {:?}", e);
                }
            }
        }
    }
}

pub fn geode_plugin<S: Scheduler, E: Executor>(mut flow: FlowBuilder<S, E>) -> FlowBuilder<S, E> {
    flow = flow.with_run_once(systems::setup);

    flow = flow.with_postfix_system(systems::update_cameras);
    flow = flow.with_postfix_system(systems::update_lights);
    flow = flow.with_postfix_system(systems::update_instances);
    flow = flow.with_postfix_system(systems::create_geometries);

    flow = flow.with_hook(RenderPlugin);

    flow
}