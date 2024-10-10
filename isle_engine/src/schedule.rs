use std::{cell::UnsafeCell, sync::atomic::Ordering};

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

impl Scheduler for isle_ecs::schedule::Scheduler {
    fn get_schedule(
        &mut self,
        _world: &UnsafeCell<World>,
        ecs: &UnsafeCell<ECS>,
    ) -> impl crate::schedule::Schedule + 'static {
        let ecs = unsafe { &*ecs.get() };
        isle_ecs::schedule::Schedule::from_ecs(ecs)
    }
}

impl Schedule for isle_ecs::schedule::Schedule {
    fn get_next(&self) -> Option<usize> {
        let next = self.next.fetch_add(1, Ordering::SeqCst);
        self.systems.get(next).copied()
    }
    fn report_done(&self, _item: usize) {}
}
