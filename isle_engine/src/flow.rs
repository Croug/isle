use std::{cell::UnsafeCell, marker::PhantomData, sync::atomic::{AtomicU32, Ordering}};

use crate::{entity::Entity, executor::Executor, plugin::EngineHook, schedule::{Schedule, Scheduler}, world::World};

pub struct Flow<T: Copy + 'static, W: World + 'static, S: Scheduler<T, W, E>, E: Executor<T, W>> {
    world: UnsafeCell<W>,
    scheduler: S,
    executor: E,
    hooks: Vec<Box<dyn EngineHook<T, W, S, E>>>,
    generation: AtomicU32,
    next_entity: AtomicU32,
    _phantom: PhantomData<T>
}

impl<T: Copy + 'static, W: World, S: Scheduler<T, W, E>, E: Executor<T, W>> Flow<T, W, S, E> {
    pub fn new() -> FlowBuilder<T, W, S, E> {
        FlowBuilder {
            world: None,
            scheduler: None,
            executor: None,
            hooks: Vec::new(),
            _phantom: PhantomData
        }
    }

    fn run_schedule(&mut self) {
        let schedule = self.scheduler.get_schedule(&self.world, &self.executor);

        while let Some(item) = schedule.get_next() {
            self.executor.run(&self.world, item);
            schedule.report_done(item);
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.hooks.iter_mut().for_each(|hook| hook.pre_run(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.run_schedule();
            self.hooks.iter_mut().for_each(|hook| hook.post_run(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.pre_render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
            self.hooks.iter_mut().for_each(|hook| hook.post_render(unsafe { &mut *self.world.get() }, &mut self.scheduler, &mut self.executor));
        }
    }

    pub fn get_scheduler(&mut self) -> &mut S {
        &mut self.scheduler
    }

    pub fn get_executor(&mut self) -> &mut E {
        &mut self.executor
    }

    pub fn get_world(&self) -> &UnsafeCell<W> {
        &self.world
    }

    pub fn make_entity(&self) -> Entity {
        Entity(
            self.generation.load(Ordering::SeqCst),
            self.next_entity.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        )
    }
}

pub struct FlowBuilder<T: 'static, W: World + 'static, S: Scheduler<T, W, E>, E: Executor<T, W>> {
    world: Option<UnsafeCell<W>>,
    scheduler: Option<S>,
    executor: Option<E>,
    hooks: Vec<Box<dyn EngineHook<T, W, S, E>>>,
    _phantom: PhantomData<T>
}

impl<T: Copy + 'static, W: World, S: Scheduler<T, W, E>, E: Executor<T, W>> FlowBuilder<T, W, S, E> {
    pub fn with_world(mut self, world: W) -> Self {
        self.world = Some(UnsafeCell::new(world));
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
                world,
                scheduler,
                executor,
                generation: AtomicU32::new(0),
                next_entity: AtomicU32::new(0),
                hooks: self.hooks,
                _phantom: PhantomData
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}