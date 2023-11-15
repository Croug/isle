pub mod entity {
    #[derive(Hash, PartialEq, Eq)]
    pub struct Entity(pub u32, pub u32);
}

pub trait Scheduler {
    fn spin(&mut self) -> ();
}
