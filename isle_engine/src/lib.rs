pub trait Scheduler {
    fn spin(&mut self);
}

pub mod executor;
pub mod flow;
pub mod plugin;
pub mod schedule;
pub mod components;
pub mod event;
pub mod params;

pub mod prelude {
    pub use crate::flow::Flow;
    pub use crate::components::*;
}
