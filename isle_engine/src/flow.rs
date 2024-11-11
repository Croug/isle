use std::{
    cell::UnsafeCell,
    fmt::Debug,
    sync::atomic::{AtomicU32, Ordering},
};

use isle_ecs::{
    ecs::{IntoSystem, System, SystemSet},
    entity::Entity,
    prelude::Component,
    world::World,
};
use isle_event::{EventReader, EventWriter};
use winit::{error::EventLoopError, event_loop::{self, EventLoop}};

use crate::{executor::Executor, input::InputMap, plugin::EngineHook, schedule::Scheduler};

pub struct Flow<S: Scheduler, E: Executor> {
    world: UnsafeCell<World>,
    system_sets: Vec<SystemSet>,
    run_once_systems: Option<SystemSet>,
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
            world: UnsafeCell::new(World::new()),
            system_sets: (0..3).map(|_| SystemSet::new()).collect(),
            run_once_systems: None,
        }
    }

    fn run_schedules(&mut self) {
        self.run_once_systems.take().map(|mut system_set| {
            let schedule = self.scheduler.get_schedule(&self.world, &system_set);
            self.executor.run(&mut system_set, &self.world, &schedule);
            unsafe { &mut *self.world.get() }.apply_commands();
        });

        for system_set in self.system_sets.iter_mut() {
            let schedule = self.scheduler.get_schedule(&self.world, system_set);
            self.executor.run(system_set, &self.world, &schedule);
            unsafe { &mut *self.world.get() }.apply_commands();
        }
    }

    pub fn spin(&mut self) {
        self.hooks.iter_mut().for_each(|hook| {
            hook.pre_run(
                unsafe { &mut *self.world.get() },
                &mut self.scheduler,
                &mut self.executor,
            )
        });
        self.run_schedules();
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

    pub fn run(&mut self) -> Result<(), EventLoopError> {
        self.add_resource(InputMap::new());
        self.add_prefix_system(crate::input::update_input);
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        event_loop.run_app(self)
    }

    fn get_event_writer<T: Clone + Debug + 'static>(&self) -> &EventWriter<T> {
        let world = unsafe { &*self.world.get() };
        let writer = world
            .get_resource::<EventWriter<T>>()
            .unwrap_or_else(|| {
                let world = unsafe { &mut *self.world.get() };
                let writer = EventWriter::<T>::new();
                world.store_resource(writer);
                world.get_resource().unwrap()
            });
        writer
    }

    pub fn send_event<T: Clone + Debug + 'static>(&mut self, event: T) {
        self.get_event_writer().clone().send(event);
    }

    pub fn get_event_listener<T: Clone + Debug + 'static>(&self) -> EventReader<T> {
        EventReader::from_writer(self.get_event_writer())
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

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let world = unsafe { &*self.world.get() };
        world.get_resource::<T>()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let world = unsafe { &mut *self.world.get() };
        unsafe { world.get_resource_mut::<T>() }
    }

    pub fn make_entity(&self) -> Entity {
        Entity(
            self.generation.load(Ordering::SeqCst),
            self.next_entity
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        )
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        let world = unsafe { &mut *self.world.get() };
        world.store_resource(resource);
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        let world = unsafe { &mut *self.world.get() };
        world.store_component(entity, component);
    }

    pub fn run_once<I, T: System + 'static>(&mut self, system: impl IntoSystem<I, System = T>) {
        self.run_once_systems.get_or_insert_with(SystemSet::new).add_system(system, &self.world);
    }

    pub fn add_prefix_system<I, T: System + 'static>(
        &mut self,
        system: impl IntoSystem<I, System = T>,
    ) {
        self.system_sets[0].add_system(system, &self.world);
    }

    pub fn add_system<I, T: System + 'static>(&mut self, system: impl IntoSystem<I, System = T>) {
        let current_set = self.current_set();
        self.system_sets[current_set].add_system(system, &self.world);
    }

    pub fn add_postfix_system<I, T: System + 'static>(
        &mut self,
        system: impl IntoSystem<I, System = T>,
    ) {
        self.system_sets
            .last_mut()
            .unwrap()
            .add_system(system, &self.world);
    }

    pub fn barrier(&mut self) {
        let index = self.system_sets.len() - 1;
        self.system_sets.insert(index, SystemSet::new());
    }

    fn current_set(&self) -> usize {
        self.system_sets.len() - 2
    }
}

pub struct FlowBuilder<S: Scheduler, E: Executor> {
    scheduler: Option<S>,
    executor: Option<E>,
    hooks: Vec<Box<dyn EngineHook<S, E>>>,
    world: UnsafeCell<World>,
    system_sets: Vec<SystemSet>,
    run_once_systems: Option<SystemSet>,
}

impl<S: Scheduler, E: Executor> FlowBuilder<S, E> {
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
    pub fn with_plugin<P: FnOnce(Self) -> Self>(self, plugin: P) -> Self {
        plugin(self)
    }
    pub fn with_run_once<I, T: System + 'static>(mut self, system: impl IntoSystem<I, System = T>) -> Self {
        self.run_once_systems.get_or_insert_with(SystemSet::new).add_system(system, &self.world);
        self
    }
    pub fn with_prefix_system<I, T: System + 'static>(mut self, system: impl IntoSystem<I, System = T>) -> Self {
        self.system_sets[0].add_system(system, &self.world);
        self
    }
    pub fn with_system<I, T: System + 'static>(mut self, system: impl IntoSystem<I, System = T>) -> Self {
        let current_set = self.current_set();
        self.system_sets[current_set].add_system(system, &self.world);

        self
    }
    pub fn with_postfix_system<I, T: System + 'static>(mut self, system: impl IntoSystem<I, System = T>) -> Self {
        self.system_sets.last_mut().unwrap().add_system(system, &self.world);
        self
    }
    fn current_set(&self) -> usize {
        self.system_sets.len() - 2
    }
    pub fn build(self) -> Flow<S, E> {
        if let (Some(scheduler), Some(executor)) = (self.scheduler, self.executor) {
            Flow {
                world: self.world,
                system_sets: self.system_sets,
                scheduler,
                executor,
                generation: AtomicU32::new(0),
                next_entity: AtomicU32::new(0),
                hooks: self.hooks,
                run_once_systems: self.run_once_systems,
            }
        } else {
            panic!("FlowBuilder missing required fields");
        }
    }
}
