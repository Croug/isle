pub mod component;
pub mod ecs;
pub mod plugin;
pub mod query;
pub mod schedule;
pub mod world;

pub mod prelude {
    pub use crate::{component::*, ecs::*, plugin::WithECS, query::*};
    pub use isle_ecs_macros::Component;
}
