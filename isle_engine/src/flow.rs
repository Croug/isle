use std::{cell::UnsafeCell, marker::PhantomData};

use crate::{executor::Executor, plugin::EngineHook, schedule::Scheduler, world::World};

pub struct Flow<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    world: UnsafeCell<W>,
    scheduler: S,
    executor: E,
    hooks: Vec<Box<dyn EngineHook<T, W, S, E>>>,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> Flow<T, W, S, E> {
    pub fn new() -> FlowBuilder<T, W, S, E> {
        FlowBuilder {
            world: None,
            scheduler: None,
            executor: None,
            hooks: Vec::new(),
            _phantom: PhantomData
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.hooks.iter_mut().for_each(|hook| hook.pre_run(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.run(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.post_run(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.pre_render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.post_render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
        }
    }
}

pub struct FlowBuilder<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    world: Option<W>,
    scheduler: Option<S>,
    executor: Option<E>,
    hooks: Vec<Box<dyn EngineHook<T, W, S, E>>>,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> FlowBuilder<T, W, S, E> {
    pub fn with_world(mut self, world: W) -> Self {
        self.world = Some(world);
        self
    }
    pub fn with_scheduler(mut self, scheduler: S) -> Self {
        self.scheduler = Some(scheduler);
        self
    }
    pub fn with_executor(mut self, executor: E) -> Self {
        self.executor = Some(executor);
        self
    }
    pub fn with_hook<P: EngineHook<T,W,S,E> + 'static>(mut self, mut plugin: P) -> Self {
        self = plugin.setup(self);
        self.hooks.push(Box::new(plugin));
        self
    }
    pub fn with_plugin<P: FnMut(Self) -> Self>(self, mut plugin: P) -> Self {
        plugin(self)
    }
    pub fn build(self) -> Flow<T, W, S, E> {
        if let (Some(world), Some(scheduler), Some(executor)) = (self.world, self.scheduler, self.executor) {
            Flow {
                world: UnsafeCell::new(world),
                scheduler,
                executor,
                hooks: self.hooks,
                _phantom: PhantomData
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}