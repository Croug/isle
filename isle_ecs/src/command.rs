use std::sync::mpsc::Sender;

use crate::{ecs::{RefType, SystemParam, TypeSet}, world::{Command, World}};

pub struct WorldCommand<'a> {
    sender: &'a mut Sender<Command>,
}

impl WorldCommand<'_> {
    pub fn add_resource<T: 'static>(&mut self, resource: T) {
        self.send(Box::new(move |world| {
            world.store_resource(resource);
        }));
    }
    pub fn send(&mut self, command: Command) {
        self.sender.send(command).unwrap();
    }
}

impl<'a> SystemParam for WorldCommand<'a> {
    type Item<'new> = WorldCommand<'new>;
    type State = Sender<Command>;

    fn collect_types(types: &mut impl TypeSet) {
        types.insert_type::<WorldCommand>(RefType::Immutable);
    }

    fn init_state(world: &std::cell::UnsafeCell<World>) -> Self::State {
        let world = unsafe { &*world.get() };
        world.command_sender().clone()
    }

    fn from_world<'w>(_: &'w std::cell::UnsafeCell<World>, state: &'w mut Self::State) -> Self::Item<'w> {
        WorldCommand { sender: state }
    }
}