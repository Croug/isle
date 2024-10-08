use std::sync::atomic::AtomicUsize;

use crate::ecs::ECS;

pub struct Scheduler;

pub struct Schedule {
    pub(crate) systems: Vec<usize>,
    pub(crate) next: AtomicUsize,
}

impl Schedule {
    pub fn from_ecs(ecs: &ECS) -> Self {
        Self {
            systems: ecs.get_system_ids(),
            next: AtomicUsize::new(0),
        }
    }
}