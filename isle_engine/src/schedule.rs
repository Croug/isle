use std::cell::UnsafeCell;

use crate::{executor::Executor, world::World};

pub trait Scheduler<T: 'static, W: World, E: Executor<T, W>> {
    fn get_schedule(
        &mut self,
        world: &UnsafeCell<W>,
        executor: &E,
    ) -> impl Schedule<Item = T> + 'static;
}

pub struct ScheduleIter<'a, T: Schedule> (&'a T);

impl<T: Schedule> Iterator for ScheduleIter<'_, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.get_next()
    }
}

pub trait Schedule {
    type Item;

    fn get_next(&self) -> Option<Self::Item>;
    fn report_done(&self, item: Self::Item);

    fn iter<'a>(&'a self) -> ScheduleIter<'a, Self> where Self: Sized {
        ScheduleIter(self)
    }
}
