use std::cell::UnsafeCell;

use isle_ecs::{ecs::ECS, world::World};

use crate::schedule::Schedule;

pub trait Executor {
    fn run<T: Schedule + Sized>(
        &mut self,
        ecs: &UnsafeCell<ECS>,
        world: &UnsafeCell<World>,
        schedule: &T,
    );
}

impl Executor for isle_ecs::executor::Executor {
    fn run<T: Schedule + Sized>(
        &mut self,
        ecs: &UnsafeCell<ECS>,
        world: &UnsafeCell<World>,
        schedule: &T,
    ) {
        for system_id in schedule.iter() {
            let ecs = unsafe { &mut *ecs.get() };
            ecs.run_system_by_id(system_id, world);
        }
    }
}
