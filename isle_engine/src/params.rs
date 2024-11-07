use std::{
    fmt::Debug,
    ops::Deref, time::Instant,
};

use isle_ecs::{
    ecs::{RefType, SystemParam},
    world,
};

use crate::{
    event::{EventReader, EventWriter},
    input::{AxisMapping, InputMap, Mapping},
};

pub struct Event<'a, T: Clone + Debug + 'static> {
    reader: &'a mut EventReader<T>,
}

impl<T: Clone + Debug + 'static> Event<'_, T> {
    pub fn read(&mut self) -> Option<T> {
        self.reader.read()
    }

    pub fn iter(&mut self) -> impl Iterator<Item = T> + '_ {
        self.reader.iter()
    }
}

impl<'a, T: Clone + Debug + 'static> SystemParam for Event<'a, T> {
    type State = EventReader<T>;
    type Item<'new> = Event<'new, T>;
    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
    fn from_world<'w>(
        _: &'w std::cell::UnsafeCell<isle_ecs::world::World>,
        state: &'w mut Self::State,
    ) -> Self::Item<'w> {
        Event { reader: state }
    }
    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        let writer = world_ref
            .get_resource::<EventWriter<T>>()
            .unwrap_or_else(|| {
                let world = unsafe { &mut *world.get() };
                let writer = EventWriter::<T>::new();
                world.store_resource(writer);
                world.get_resource().unwrap()
            });

        EventReader::from_writer(writer)
    }
}

pub struct EventTrigger<'a, T: Clone + Debug + 'static> {
    writer: &'a mut EventWriter<T>,
}

impl<T: Clone + Debug + 'static> EventTrigger<'_, T> {
    pub fn send(&mut self, event: T) {
        self.writer.send(event);
    }
}

impl<'a, T: Clone + Debug + 'static> SystemParam for EventTrigger<'a, T> {
    type State = EventWriter<T>;
    type Item<'new> = EventTrigger<'new, T>;

    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        let writer = world_ref
            .get_resource::<EventWriter<T>>()
            .unwrap_or_else(|| {
                let world = unsafe { &mut *world.get() };
                let writer = EventWriter::<T>::new();
                world.store_resource(writer);
                world.get_resource().unwrap()
            });

        writer.clone()
    }

    fn from_world<'w>(
        _: &'w std::cell::UnsafeCell<isle_ecs::world::World>,
        state: &'w mut Self::State,
    ) -> Self::Item<'w> {
        EventTrigger { writer: state }
    }

    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
}

#[derive(Clone, Copy)]
pub struct Input<T: Mapping> {
    state: bool,
    just_changed: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Mapping> Input<T> {
    pub fn just_changed(&self) -> bool {
        self.just_changed
    }

    pub fn state(&self) -> bool {
        self.state
    }
}

impl<T: Mapping> Deref for Input<T> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

pub struct InputState<T: Mapping> {
    last_state: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Mapping + 'static> SystemParam for Input<T> {
    type State = InputState<T>;
    type Item<'a> = Input<T>;

    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        let input_map = world_ref.get_resource::<InputMap>().unwrap_or_else(|| {
            let world = unsafe { &mut *world.get() };
            let input_map = InputMap::new();
            world.store_resource(input_map);
            world.get_resource().unwrap()
        });
        InputState {
            last_state: T::get(input_map),
            _phantom: std::marker::PhantomData,
        }
    }

    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }

    fn from_world<'w>(
        world: &'w std::cell::UnsafeCell<world::World>,
        state: &'w mut Self::State,
    ) -> Self::Item<'w> {
        let input_map = unsafe { &*world.get() }.get_resource::<InputMap>().unwrap();
        let input_state = T::get(input_map);
        let just_changed = input_state != state.last_state;
        state.last_state = input_state;

        Input {
            state: input_state,
            just_changed,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct InputAxis<T: AxisMapping> {
    value: f32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AxisMapping> InputAxis<T> {
    pub fn value(&self) -> f32 {
        self.value
    }
}

impl<T: AxisMapping + 'static> SystemParam for InputAxis<T> {
    type State = ();
    type Item<'new> = InputAxis<T>;

    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        world_ref.get_resource::<InputMap>().unwrap_or_else(|| {
            let world = unsafe { &mut *world.get() };
            let input_map = InputMap::new();
            world.store_resource(input_map);
            world.get_resource().unwrap()
        });
    }

    fn from_world<'w>(
        world: &'w std::cell::UnsafeCell<world::World>,
        _: &'w mut Self::State,
    ) -> Self::Item<'w> {
        let input_map = unsafe { &*world.get() }.get_resource::<InputMap>().unwrap();
        let value = T::get(input_map);

        InputAxis {
            value,
            _phantom: std::marker::PhantomData,
        }
    }

    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
}

struct Tick {
    delta: f32,
}

impl SystemParam for Tick {
    type State = Instant;
    type Item<'a> = Tick;

    fn init_state(_: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        Instant::now()
    }
    fn from_world<'w>(world: &'w std::cell::UnsafeCell<world::World>, state: &'w mut Self::State) -> Self::Item<'w> {
        let delta = state.elapsed().as_secs_f32();
        *state = Instant::now();

        Tick { delta }
    }
    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<Instant>(RefType::Immutable);
    }
}
