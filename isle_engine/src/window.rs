use std::sync::OnceLock;

use winit::{application::ApplicationHandler, window::{Window, WindowAttributes}};

use crate::{executor::Executor, flow::Flow, schedule::Scheduler};

static WINDOW: OnceLock<Window> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
struct ReconfigureSurface;

impl<S: Scheduler, E: Executor> ApplicationHandler for Flow<S,E> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if WINDOW.get().is_none() {
            let window = event_loop.create_window(WindowAttributes::default()).unwrap();
            WINDOW.set(window).unwrap();
        }
        self.send_event(ReconfigureSurface);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        WINDOW.get().unwrap();
    }
}