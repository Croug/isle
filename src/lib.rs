pub extern crate isle_ecs;
pub extern crate isle_engine;

pub mod prelude {
    pub use defaults::DefaultPlugins;
    pub use isle_ecs;
    pub use isle_ecs::prelude::*;
    pub use isle_engine;
    pub use isle_engine::prelude::*;
}

pub mod defaults {
    use isle_engine::flow::FlowBuilder;

    type Scheduler = isle_ecs::schedule::Scheduler;
    type Executor = isle_ecs::executor::Executor;

    #[cfg(feature = "geode")]
    fn renderer(mut flow: FlowBuilder<Scheduler, Executor>) -> FlowBuilder<Scheduler, Executor> {
        flow = flow.with_plugin(geode::plugin::geode_plugin);
        flow
    }

    #[cfg(not(feature = "geode"))]
    fn renderer<S: Scheduler, E: Executor> (flow: FlowBuilder<S,E>) -> FlowBuilder<S,E> {
        flow
    }


    pub trait DefaultPlugins {
        fn with_default_plugins(self) -> Self;
    }

    impl DefaultPlugins for isle_engine::flow::FlowBuilder<Scheduler, Executor> {
        fn with_default_plugins(mut self) -> Self {
            self = self.with_plugin(isle_engine::plugin::default_plugins);

            self = renderer(self);

            self
        }
    }
}
