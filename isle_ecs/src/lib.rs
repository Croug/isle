pub mod world;
pub mod component;
pub mod ecs;
pub mod query;

pub mod prelude {
    pub use super::ecs::{
        System,
        IntoSystem,
        SystemParam,
    };
}
