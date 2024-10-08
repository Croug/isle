use std::cell::UnsafeCell;

use crate::world::World;

pub trait Executor<T, W: World> {
    fn run(&mut self, world: &UnsafeCell<W>, identifier: T);
}