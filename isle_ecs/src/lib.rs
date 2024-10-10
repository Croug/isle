pub mod component;
pub mod ecs;
pub mod entity;
pub mod executor;
pub mod query;
pub mod schedule;
pub mod world;

pub mod prelude {
    pub use crate::{component::*, ecs::*, entity::*, executor::*, query::*};
    pub use isle_ecs_macros::Component;
}
