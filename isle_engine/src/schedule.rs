use std::cell::UnsafeCell;

use crate::{executor::Executor, world::World};

pub trait Scheduler<T: 'static, W: World, E: Executor<T, W>> {
    fn get_schedule(&mut self, world: &UnsafeCell<W>, executor: &E) -> impl Schedule<Item = T> + 'static;
}

pub trait Schedule {
    type Item;

    fn get_next(&self) -> Option<Self::Item>;
    fn report_done(&self, item: Self::Item);
}