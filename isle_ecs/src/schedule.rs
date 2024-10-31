use std::sync::atomic::AtomicUsize;

use crate::ecs::SystemSet;

pub struct Scheduler;

pub struct Schedule {
    pub systems: Vec<usize>,
    pub next: AtomicUsize,
}

impl Schedule {
    pub fn from_system_set(systems: &SystemSet) -> Self {
        Self {
            systems: systems.get_system_ids(),
            next: AtomicUsize::new(0),
        }
    }
}
