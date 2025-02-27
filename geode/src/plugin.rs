use isle_engine::{executor::Executor, flow::{FlowBuilder, stages}, plugin::EngineHook, schedule::Scheduler, window::ReconfigureSurface};
use isle_event::{EventReader, EventWriter};
use wgpu::SurfaceError;

use crate::renderer::Renderer;

pub mod components;
pub mod systems;

#[derive(Default)]
struct RenderPlugin {
    reconfigure_surface: Option<EventReader<ReconfigureSurface>>,
}

impl RenderPlugin {
    fn get_reconfigure_surface_listener(&mut self, world: &mut isle_ecs::world::World) -> &mut EventReader<ReconfigureSurface> {
        self.reconfigure_surface.as_mut().unwrap()
    }
}

impl<S: Scheduler, E: Executor> EngineHook<S,E> for RenderPlugin {
    fn setup(&mut self, mut flow_builder: FlowBuilder<S, E>) -> FlowBuilder<S, E> {
        let writer = EventWriter::<ReconfigureSurface>::new();
        self.reconfigure_surface = Some(EventReader::from_writer(&writer));
        flow_builder.with_resource(writer)
    }
    fn pre_render(&mut self, world: &mut isle_ecs::world::World, _scheduler: &mut S, _executor: &mut E) {
        let settings = self.get_reconfigure_surface_listener(world).iter().last();
        if let Some(ReconfigureSurface(size)) = settings {
            let renderer = unsafe { world.get_resource_mut::<Renderer>() }.unwrap();
            renderer.resize(size);
        }
    }
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

    flow = flow.with_staged_system(stages::POST_RUN, systems::update_cameras);
    flow = flow.with_staged_system(stages::POST_RUN, systems::update_lights);
    flow = flow.with_staged_system(stages::POST_RUN, systems::update_instances);
    flow = flow.with_staged_system(stages::POST_RUN, systems::create_geometries);

    flow = flow.with_hook(RenderPlugin::default());

    flow
}