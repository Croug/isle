use std::cell::UnsafeCell;

use crate::{world::World, schedule::Scheduler};

pub trait Executor<T: 'static, W: World, S: Scheduler<W, T>> {
    fn run(&mut self, world: &UnsafeCell<W>, scheduler: &mut S);
}