use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU32, Ordering},
};

use isle_ecs::{ecs::{IntoSystem, System, ECS}, entity::Entity, prelude::Component, world::World};

use crate::{
    executor::Executor,
    plugin::EngineHook,
    schedule::Scheduler,
};

pub struct Flow<S: Scheduler, E: Executor> {
    world: UnsafeCell<World>,
    ecs: UnsafeCell<ECS>,
    scheduler: S,
    executor: E,
    hooks: Vec<Box<dyn EngineHook<S, E>>>,
    generation: AtomicU32,
    next_entity: AtomicU32,
}

impl<S: Scheduler, E: Executor> Flow<S, E> {
    pub fn new() -> FlowBuilder<S, E> {
        FlowBuilder {
            scheduler: None,
            executor: None,
            hooks: Vec::new(),
        }
    }

    fn run_schedule(&mut self) {
        let schedule = self.scheduler.get_schedule(&self.world, &self.ecs);
        self.executor.run(&self.ecs, &self.world, &schedule);
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.hooks.iter_mut().for_each(|hook| {
                hook.pre_run(
                    unsafe { &mut *self.world.get() },
                    &mut self.scheduler,
                    &mut self.executor,
                )
            });
            self.run_schedule();
            self.hooks.iter_mut().for_each(|hook| {
                hook.post_run(
                    unsafe { &mut *self.world.get() },
                    &mut self.scheduler,
                    &mut self.executor,
                )
            });
            self.hooks.iter_mut().for_each(|hook| {
                hook.pre_render(
                    unsafe { &mut *self.world.get() },
                    &mut self.scheduler,
                    &mut self.executor,
                )
            });
            self.hooks.iter_mut().for_each(|hook| {
                hook.render(
                    unsafe { &mut *self.world.get() },
                    &mut self.scheduler,
                    &mut self.executor,
                )
            });
            self.hooks.iter_mut().for_each(|hook| {
                hook.post_render(
                    unsafe { &mut *self.world.get() },
                    &mut self.scheduler,
                    &mut self.executor,
                )
            });
        }
    }

    pub fn get_scheduler(&mut self) -> &mut S {
        &mut self.scheduler
    }

    pub fn get_executor(&mut self) -> &mut E {
        &mut self.executor
    }

    pub fn get_world(&self) -> &UnsafeCell<World> {
        &self.world
    }

    pub fn make_entity(&self) -> Entity {
        Entity(
            self.generation.load(Ordering::SeqCst),
            self.next_entity
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        )
    }

    pub fn add_resource<T: 'static>(&self, resource: T) {
        let ecs = unsafe { &mut *self.ecs.get() };
        let world = unsafe { &mut *self.world.get() };
        ecs.add_resource(resource, world);
    }

    pub fn add_component<T: Component + 'static>(&self, entity: Entity, component: T) {
        let ecs = unsafe { &mut *self.ecs.get() };
        let world = unsafe { &mut *self.world.get() };
        ecs.add_component(entity, component, world);
    }

    pub fn add_system<I, T: System + 'static>(&self, system: impl IntoSystem<I, System = T>) {
        let ecs = unsafe { &mut *self.ecs.get() };
        ecs.add_system(system);
    }
}

pub struct FlowBuilder<S: Scheduler, E: Executor> {
    scheduler: Option<S>,
    executor: Option<E>,
    hooks: Vec<Box<dyn EngineHook<S, E>>>,
}

impl<S: Scheduler, E: Executor>
    FlowBuilder<S, E>
{
    pub fn with_scheduler(mut self, scheduler: S) -> Self {
        self.scheduler = Some(scheduler);
        self
    }
    pub fn with_executor(mut self, executor: E) -> Self {
        self.executor = Some(executor);
        self
    }
    pub fn with_hook<P: EngineHook<S, E> + 'static>(mut self, mut plugin: P) -> Self {
        self = plugin.setup(self);
        self.hooks.push(Box::new(plugin));
        self
    }
    pub fn with_plugin<P: FnMut(Self) -> Self>(self, mut plugin: P) -> Self {
        plugin(self)
    }
    pub fn build(self) -> Flow<S, E> {
        if let (Some(scheduler), Some(executor)) =
            (self.scheduler, self.executor)
        {
            Flow {
                world: UnsafeCell::new(World::new()),
                ecs: UnsafeCell::new(ECS::new()),
                scheduler,
                executor,
                generation: AtomicU32::new(0),
                next_entity: AtomicU32::new(0),
                hooks: self.hooks,
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}
