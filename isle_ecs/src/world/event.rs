use std::any::TypeId;

use crate::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub enum EntityEvent {
    Created(Entity),
    ComponentAdded(Entity, TypeId),
    Destroyed(Entity),
}
