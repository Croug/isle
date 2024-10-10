pub trait Scheduler {
    fn spin(&mut self);
}

pub mod executor;
pub mod flow;
pub mod plugin;
pub mod schedule;

pub mod prelude {
    pub use crate::flow::Flow;
}
