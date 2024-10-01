use crate::{executor::Executor, flow::{Flow, FlowBuilder}, schedule::Scheduler, world::World};

#[allow(unused_variables)]
pub trait EnginePlugin<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    fn setup(&mut self, flow_builder: &mut FlowBuilder<T,W,S,E>) {}
    fn pre_run(&mut self, flow: &mut Flow<T,W,S,E>) {}
    fn run(&mut self, flow: &mut Flow<T,W,S,E>) {}
    fn post_run(&mut self, flow: &mut Flow<T,W,S,E>) {}
    fn pre_render(&mut self, flow: &mut Flow<T,W,S,E>) {}
    fn render(&mut self, flow: &mut Flow<T,W,S,E>) {}
    fn post_render(&mut self, flow: &mut Flow<T,W,S,E>) {}
}

#[allow(unused_variables)]
#[cfg(feature = "async")]
pub trait AsyncEnginePlugin<T: 'static, W: World, S: Scheduler<W, T>, E: Executor<T, W, S>> {
    fn setup(flow_builder: &mut FlowBuilder<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn pre_run(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn run(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn post_run(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn pre_render(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn render(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
    fn post_render(&mut self, flow: &mut Flow<T,W,S,E>) -> impl std::future::Future<Output = ()> + Send {
        async {}
    }
}