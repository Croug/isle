use std::{cell::UnsafeCell, marker::PhantomData};

use crate::{executor::Executor, plugin::EnginePlugin, schedule::Scheduler, world::World};

pub struct Flow<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    world: UnsafeCell<W>,
    scheduler: S,
    executor: E,
    plugins: Vec<Box<dyn EnginePlugin<T, W, S, E>>>,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> Flow<T, W, S, E> {
    pub fn new() -> FlowBuilder<T, W, S, E> {
        FlowBuilder {
            world: None,
            scheduler: None,
            executor: None,
            plugins: Vec::new(),
            _phantom: PhantomData
        }
    }
}

pub struct FlowBuilder<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    world: Option<W>,
    scheduler: Option<S>,
    executor: Option<E>,
    plugins: Vec<Box<dyn EnginePlugin<T, W, S, E>>>,
    _phantom: PhantomData<T>
}

impl<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> FlowBuilder<T, W, S, E> {
    pub fn with_world(&mut self, world: W) -> &mut Self {
        self.world = Some(world);
        self
    }
    pub fn with_scheduler(&mut self, scheduler: S) -> &mut Self {
        self.scheduler = Some(scheduler);
        self
    }
    pub fn with_executor(&mut self, executor: E) -> &mut Self {
        self.executor = Some(executor);
        self
    }
    pub fn with_engine_plugin<P: EnginePlugin<T,W,S,E> + 'static>(&mut self, mut plugin: P) -> &mut Self {
        plugin.setup(self);
        self.plugins.push(Box::new(plugin));
        self
    }
    pub fn with_plugin<P: FnMut(&mut FlowBuilder<T, W, S, E>)>(&mut self, mut plugin: P) -> &mut Self {
        plugin(self);
        self
    }
    pub fn build(self) -> Flow<T, W, S, E> {
        if let (Some(world), Some(scheduler), Some(executor)) = (self.world, self.scheduler, self.executor) {
            Flow {
                world: UnsafeCell::new(world),
                scheduler,
                executor,
                plugins: self.plugins,
                _phantom: PhantomData
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}