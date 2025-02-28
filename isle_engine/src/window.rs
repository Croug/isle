use std::sync::OnceLock;

use isle_math::vector::d2::Vec2;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::PhysicalKey,
    window::{Window, WindowAttributes},
};

use crate::{executor::Executor, flow::Flow, input::Key, schedule::Scheduler};

pub static WINDOW: OnceLock<Window> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub struct ReconfigureSurface(pub Vec2);

#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
    pub state: bool,
    pub key: Key,
}

impl<S: Scheduler, E: Executor> ApplicationHandler for Flow<S, E> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        WINDOW
            .set(
                event_loop
                    .create_window(WindowAttributes::default())
                    .unwrap(),
            )
            .unwrap();
        let size = WINDOW.get().unwrap().inner_size();
        self.send_event(ReconfigureSurface(Vec2(
            size.width as f32,
            size.height as f32,
        )));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = WINDOW.get().unwrap();

        if window_id != window.id() {
            return;
        }

        match event {
            WindowEvent::RedrawRequested => {
                self.spin();
                WINDOW.get().unwrap().request_redraw();
            }
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                self.send_event(ReconfigureSurface(Vec2(
                    size.width as f32,
                    size.height as f32,
                )));
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        physical_key: PhysicalKey::Code(key_code),
                        ..
                    },
                ..
            } => self.send_event(KeyboardEvent {
                state: state == ElementState::Pressed,
                key: key_code.into(),
            }),

            _ => (),
        }
    }
}
