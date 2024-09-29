use std::{cell::UnsafeCell, marker::PhantomData};

use crate::{event::EventLoop, schedule::Scheduler, world::World};

pub struct Flow<T: 'static, W: World, S: Scheduler<W, T>, E: EventLoop<T, W, S>> {
    world: UnsafeCell<W>,
    scheduler: S,
    event_loop: E,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: EventLoop<T, W, S>> Flow<T, W, S, E> {
    pub fn new() -> FlowBuilder<T, W, S, E> {
        FlowBuilder {
            world: None,
            scheduler: None,
            event_loop: None,
            _phantom: PhantomData
        }
    }
}

pub struct FlowBuilder<T: 'static, W: World, S: Scheduler<W, T>, E: EventLoop<T, W, S>> {
    world: Option<W>,
    scheduler: Option<S>,
    event_loop: Option<E>,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: EventLoop<T, W, S>> FlowBuilder<T, W, S, E> {
    pub fn with_world(&mut self, world: W) -> &mut Self {
        self.world = Some(world);
        self
    }
    pub fn with_scheduler(&mut self, scheduler: S) -> &mut Self {
        self.scheduler = Some(scheduler);
        self
    }
    pub fn with_event_loop(&mut self, event_loop: E) -> &mut Self {
        self.event_loop = Some(event_loop);
        self
    }
    pub fn build(self) -> Flow<T, W, S, E> {
        if let (Some(world), Some(scheduler), Some(event_loop)) = (self.world, self.scheduler, self.event_loop) {
            Flow {
                world: UnsafeCell::new(world),
                scheduler,
                event_loop,
                _phantom: PhantomData
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}