pub mod component;
pub mod ecs;
pub mod query;
pub mod world;
pub mod schedule;
pub mod plugin;

pub mod prelude {
    pub use crate::{component::*, ecs::*, query::*, plugin::WithECS};
    pub use isle_ecs_macros::Component;
}
