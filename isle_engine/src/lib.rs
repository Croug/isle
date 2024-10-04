pub mod entity {
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub struct Entity(pub u32, pub u32);
}

pub trait Scheduler {
    fn spin(&mut self);
}

pub mod flow;
pub mod world;
pub mod executor;
pub mod schedule;
pub mod plugin;

pub mod prelude {
    pub use crate::flow::Flow;
}