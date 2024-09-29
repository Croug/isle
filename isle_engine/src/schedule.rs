use std::cell::UnsafeCell;

use crate::world::World;

pub trait Scheduler<W: World, T: 'static> {
    fn get_schedule(&mut self, world: &UnsafeCell<W>) -> impl Schedule<Item = T>;
}

pub trait Schedule {
    type Item;

    fn get_next(&self) -> Option<Self::Item>;
    fn report_done(&self, item: Self::Item);
}