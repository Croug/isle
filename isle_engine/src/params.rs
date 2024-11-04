use isle_ecs::ecs::{RefType, SystemParam};

use crate::event::{EventArgs, EventReader, EventWriter};

pub struct Event<'a, T: EventArgs> {
    reader: &'a mut EventReader<T>,
}

impl<T: EventArgs> Event<'_, T> {
    pub fn read(&mut self) -> Option<T> {
        self.reader.read()
    }
    
    pub fn iter(&mut self) -> impl Iterator<Item = T> + '_ {
        self.reader.iter()
    }
}

impl<'a, T: EventArgs + 'static> SystemParam for Event<'a, T> {
    type State = EventReader<T>;
    type Item<'new> = Event<'new, T>;
    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
    fn from_world<'w>(_: &'w std::cell::UnsafeCell<isle_ecs::world::World>, state: &'w mut Self::State) -> Self::Item<'w> {
        Event {
            reader: state,
        }
    }
    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        let writer = world_ref.get_resource::<EventWriter<T>>().unwrap_or_else(|| {
            let world = unsafe { &mut *world.get() };
            let writer = EventWriter::<T>::new();
            world.store_resource(writer);
            world.get_resource().unwrap()
        });

        EventReader::from_writer(writer)
    }
}

pub struct EventTrigger<'a, T: EventArgs> {
    writer: &'a mut EventWriter<T>,
}

impl<T: EventArgs> EventTrigger<'_, T> {
    pub fn send(&mut self, event: T) {
        self.writer.send(event);
    }
}

impl<'a, T: EventArgs + 'static> SystemParam for EventTrigger<'a, T> {
    type State = EventWriter<T>;
    type Item<'new> = EventTrigger<'new, T>;

    fn init_state(world: &std::cell::UnsafeCell<isle_ecs::world::World>) -> Self::State {
        let world_ref = unsafe { &*world.get() };
        let writer = world_ref.get_resource::<EventWriter<T>>().unwrap_or_else(|| {
            let world = unsafe { &mut *world.get() };
            let writer = EventWriter::<T>::new();
            world.store_resource(writer);
            world.get_resource().unwrap()
        });

        writer.clone()
    }

    fn from_world<'w>(world: &'w std::cell::UnsafeCell<isle_ecs::world::World>, state: &'w mut Self::State) -> Self::Item<'w> {
        EventTrigger {
            writer: state,
        }
    }

    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
}