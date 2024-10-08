pub mod entity {
    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    pub struct Entity(pub u32, pub u32);
}

pub trait Scheduler {
    fn spin(&mut self);
}

pub mod executor;
pub mod flow;
pub mod plugin;
pub mod schedule;
pub mod world;

pub mod prelude {
    pub use crate::entity::Entity;
    pub use crate::flow::Flow;
}
