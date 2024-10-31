use std::cell::UnsafeCell;

use isle_ecs::{ecs::SystemSet, world::World};

use crate::schedule::Schedule;

pub trait Executor {
    fn run<T: Schedule + Sized>(
        &mut self,
        ecs: &mut SystemSet,
        world: &UnsafeCell<World>,
        schedule: &T,
    );
}

impl Executor for isle_ecs::executor::Executor {
    fn run<T: Schedule + Sized>(
        &mut self,
        system_set: &mut SystemSet,
        world: &UnsafeCell<World>,
        schedule: &T,
    ) {
        for system_id in schedule.iter() {
            system_set.run_system_by_id(system_id, world);
        }
    }
}
