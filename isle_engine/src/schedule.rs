use std::cell::UnsafeCell;

use isle_ecs::{ecs::ECS, world::World};

pub trait Scheduler {
    fn get_schedule(
        &mut self,
        world: &UnsafeCell<World>,
        ecs: &UnsafeCell<ECS>,
    ) -> impl Schedule + 'static;
}

pub struct ScheduleIter<'a, T: Schedule>(&'a T);

impl<T: Schedule> Iterator for ScheduleIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.0.get_next()
    }
}

pub trait Schedule {
    fn get_next(&self) -> Option<usize>;
    fn report_done(&self, item: usize);

    fn iter<'a>(&'a self) -> ScheduleIter<'a, Self>
    where
        Self: Sized,
    {
        ScheduleIter(self)
    }
}
