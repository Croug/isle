#[cfg(feature = "ecs")]
pub extern crate isle_ecs;
pub extern crate isle_engine;

pub mod prelude {
    pub use defaults::DefaultPlugins;
    #[cfg(feature = "ecs")]
    pub use isle_ecs;
    #[cfg(feature = "ecs")]
    pub use isle_ecs::prelude::*;
    pub use isle_engine;
    pub use isle_engine::prelude::*;
}

pub mod defaults {
    type Scheduler = isle_ecs::schedule::Scheduler;
    type Executor = isle_ecs::executor::Executor;

    pub trait DefaultPlugins {
        fn with_default_plugins(self) -> Self;
    }

    impl DefaultPlugins for isle_engine::flow::FlowBuilder<Scheduler, Executor> {
        fn with_default_plugins(self) -> Self {
            self.with_plugin(isle_engine::plugin::default_plugins)
        }
    }
}
