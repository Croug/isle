use std::cell::UnsafeCell;

use isle_ecs::{ecs::ECS, world::World};

use crate::schedule::Schedule;

pub trait Executor {
    fn run<T: Schedule + Sized>(&mut self, ecs: &UnsafeCell<ECS>, world: &UnsafeCell<World>, schedule: &T);
}
