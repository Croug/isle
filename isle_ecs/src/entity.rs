use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Entity(pub u32, pub u32);

pub trait EntityFactory {
    fn make_entity(&self) -> Entity;
    fn advance_generation(&self) -> u32;
}

pub struct DefaultEntityFactory {
    generation: AtomicU32,
    next_entity: AtomicU32,
}

impl DefaultEntityFactory {
    pub fn new() -> Self {
        Self {
            generation: AtomicU32::new(0),
            next_entity: AtomicU32::new(0),
        }
    }
}

impl EntityFactory for DefaultEntityFactory {
    fn make_entity(&self) -> Entity {
        Entity(
            self.generation.load(Ordering::SeqCst),
            self.next_entity
                .fetch_add(1, Ordering::SeqCst),
        )
    }

    fn advance_generation(&self) -> u32 {
        self.generation.fetch_add(1, Ordering::SeqCst)
    }
}