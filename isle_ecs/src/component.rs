use std::any::Any;

pub trait Component: Any + 'static {}

impl<T> Component for T where T: Any + 'static {}
