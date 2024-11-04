use isle_ecs::{ecs::{RefType, SystemParam}, world};

use crate::event::{EventArgs, EventReader, EventWriter};

pub struct Event<'a, T: EventArgs> {
    reader: &'a EventReader<T>,
}

impl<'a, T: EventArgs + 'static> SystemParam for Event<'a, T> {
    type State = EventReader<T>;
    type Item<'new> = Event<'new, T>;
    fn collect_types(types: &mut impl isle_ecs::prelude::TypeSet) {
        types.insert_type::<T>(RefType::Immutable);
    }
    fn from_world<'w>(world: &'w std::cell::UnsafeCell<isle_ecs::world::World>, state: &'w mut Self::State) -> Self::Item<'w> {
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