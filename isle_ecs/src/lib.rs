pub mod component;
pub mod ecs;
pub mod query;
pub mod world;

pub mod prelude {
    pub use crate::{component::*, ecs::*, query::*};
    pub use isle_ecs_macros::Component;
}
