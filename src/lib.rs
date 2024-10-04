extern crate isle_ecs;
extern crate isle_engine;

pub mod prelude {
    #[cfg(feature = "ecs")]
    pub use isle_ecs::prelude::*;
    pub use isle_engine::prelude::*;
    pub use defaults::DefaultPlugins;
}

pub mod defaults {
    type System = Box<dyn isle_ecs::ecs::System>;
    type World = isle_ecs::world::World;
    type Scheduler = isle_ecs::schedule::Scheduler;
    type Executor = isle_ecs::executor::Executor;

    pub trait DefaultPlugins {
        fn with_default_plugins(self) -> Self;
    }

    impl DefaultPlugins for isle_engine::flow::FlowBuilder<System, World, Scheduler, Executor> {
        fn with_default_plugins(self) -> Self {
            self
                .with_plugin(isle_ecs::plugin::ecs_plugin)
        }
    }
}