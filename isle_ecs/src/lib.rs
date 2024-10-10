pub mod component;
pub mod ecs;
pub mod query;
pub mod schedule;
pub mod world;
pub mod entity;
pub mod executor;

pub mod prelude {
    pub use crate::{component::*, ecs::*, query::*, executor::*, entity::*};
    pub use isle_ecs_macros::Component;
}
