use std::hash::Hash;

use isle_ecs::ecs::ResMut;
use rustc_hash::{FxHashMap, FxHashSet};

pub use isle_engine_macros::{define_axis_binding, define_binding};
use winit::keyboard::KeyCode;

use crate::{params::Event, window::KeyboardEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Symbols
    Grave,
    Minus,
    Equal,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,

    // Num row
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Num pad
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadAdd,
    NumPadSubtract,
    NumPadMultiply,
    NumPadDivide,
    NumPadDecimal,
    NumPadEnter,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Special keys
    Space,
    Enter,
    Escape,
    Backspace,
    Tab,
    CapsLock,
    LeftShift,
    LeftControl,
    LeftAlt,
    RightShift,
    RightControl,
    RightAlt,
    Super,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    PrintScreen,
    ScrollLock,
    Pause,
    Menu,

    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    LeftStickX,
    LeftStickY,

    RightStickX,
    RightStickY,

    LeftTrigger,
    RightTrigger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    North,
    East,
    South,
    West,

    PadUp,
    PadRight,
    PadDown,
    PadLeft,

    LeftBumper,
    RightBumper,

    LeftStick,
    RightStick,

    Start,
    Select,
    Menu,
}

pub trait Mapping: Sized {
    fn keys<'a>() -> &'a [Key];
    fn buttons<'a>() -> &'a [Button];

    fn get(input_map: &InputMap) -> bool {
        input_map.check_mapping::<Self>()
    }
}

impl Mapping for () {
    fn keys<'a>() -> &'a [Key] {
        &[]
    }

    fn buttons<'a>() -> &'a [Button] {
        &[]
    }

    fn get(_: &InputMap) -> bool {
        false
    }
}

pub trait AxisMapping: Sized {
    type PositiveMapping: Mapping;
    type NegativeMapping: Mapping;

    fn axes<'a>() -> &'a [Axis];

    fn get(input_map: &InputMap) -> f32 {
        input_map.check_axis_mapping::<Self>()
    }
}

#[derive(Default)]
pub struct InputMap {
    keys: FxHashSet<Key>,
    buttons: FxHashSet<Button>,
    axes: FxHashMap<Axis, f32>,
}

impl InputMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_key(&mut self, key: Key, state: bool) {
        if state {
            self.keys.insert(key);
        } else {
            self.keys.remove(&key);
        }
    }

    pub fn set_button(&mut self, button: Button, state: bool) {
        if state {
            self.buttons.insert(button);
        } else {
            self.buttons.remove(&button);
        }
    }

    pub fn set_axis(&mut self, axis: Axis, value: f32) {
        self.axes.insert(axis, value);
    }

    pub fn get_key(&self, key: Key) -> bool {
        self.keys.contains(&key)
    }

    pub fn get_button(&self, button: Button) -> bool {
        self.buttons.contains(&button)
    }

    pub fn get_axis(&self, axis: Axis) -> f32 {
        *self.axes.get(&axis).unwrap_or(&0.0)
    }

    pub fn check_mapping<M: Mapping>(&self) -> bool {
        let keys = FxHashSet::from_iter(M::keys().iter().copied());
        let buttons = FxHashSet::from_iter(M::buttons().iter().copied());

        !self.keys.is_disjoint(&keys) || !self.buttons.is_disjoint(&buttons)
    }

    pub fn check_axis_mapping<M: AxisMapping>(&self) -> f32 {
        let positive: f32 = M::PositiveMapping::get(self).into();
        let negative: f32 = M::NegativeMapping::get(self).into();

        let axis = M::axes()
            .iter()
            .map(|axis| self.get_axis(*axis))
            .fold(
                0.0_f32,
                |max, value| {
                    if value.abs() > max.abs() {
                        value
                    } else {
                        max
                    }
                },
            );

        let fallback = positive - negative;
        if axis.abs() > fallback.abs() {
            axis
        } else {
            fallback
        }
    }
}

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::KeyA => Key::A,
            KeyCode::KeyB => Key::B,
            KeyCode::KeyC => Key::C,
            KeyCode::KeyD => Key::D,
            KeyCode::KeyE => Key::E,
            KeyCode::KeyF => Key::F,
            KeyCode::KeyG => Key::G,
            KeyCode::KeyH => Key::H,
            KeyCode::KeyI => Key::I,
            KeyCode::KeyJ => Key::J,
            KeyCode::KeyK => Key::K,
            KeyCode::KeyL => Key::L,
            KeyCode::KeyM => Key::M,
            KeyCode::KeyN => Key::N,
            KeyCode::KeyO => Key::O,
            KeyCode::KeyP => Key::P,
            KeyCode::KeyQ => Key::Q,
            KeyCode::KeyR => Key::R,
            KeyCode::KeyS => Key::S,
            KeyCode::KeyT => Key::T,
            KeyCode::KeyU => Key::U,
            KeyCode::KeyV => Key::V,
            KeyCode::KeyW => Key::W,
            KeyCode::KeyX => Key::X,
            KeyCode::KeyY => Key::Y,
            KeyCode::KeyZ => Key::Z,

            KeyCode::Backquote => Key::Grave,
            KeyCode::Minus => Key::Minus,
            KeyCode::Equal => Key::Equal,
            KeyCode::BracketLeft => Key::LeftBracket,
            KeyCode::BracketRight => Key::RightBracket,
            KeyCode::Backslash => Key::Backslash,
            KeyCode::Semicolon => Key::Semicolon,
            KeyCode::Quote => Key::Apostrophe,
            KeyCode::Comma => Key::Comma,
            KeyCode::Period => Key::Period,
            KeyCode::Slash => Key::Slash,

            KeyCode::Digit0 => Key::Num0,
            KeyCode::Digit1 => Key::Num1,
            KeyCode::Digit2 => Key::Num2,
            KeyCode::Digit3 => Key::Num3,
            KeyCode::Digit4 => Key::Num4,
            KeyCode::Digit5 => Key::Num5,
            KeyCode::Digit6 => Key::Num6,
            KeyCode::Digit7 => Key::Num7,
            KeyCode::Digit8 => Key::Num8,
            KeyCode::Digit9 => Key::Num9,

            KeyCode::Numpad0 => Key::NumPad0,
            KeyCode::Numpad1 => Key::NumPad1,
            KeyCode::Numpad2 => Key::NumPad2,
            KeyCode::Numpad3 => Key::NumPad3,
            KeyCode::Numpad4 => Key::NumPad4,
            KeyCode::Numpad5 => Key::NumPad5,
            KeyCode::Numpad6 => Key::NumPad6,
            KeyCode::Numpad7 => Key::NumPad7,
            KeyCode::Numpad8 => Key::NumPad8,
            KeyCode::Numpad9 => Key::NumPad9,
            KeyCode::NumpadAdd => Key::NumPadAdd,
            KeyCode::NumpadSubtract => Key::NumPadSubtract,
            KeyCode::NumpadMultiply => Key::NumPadMultiply,
            KeyCode::NumpadDivide => Key::NumPadDivide,
            KeyCode::NumpadDecimal => Key::NumPadDecimal,
            KeyCode::NumpadEnter => Key::NumPadEnter,

            KeyCode::F1 => Key::F1,
            KeyCode::F2 => Key::F2,
            KeyCode::F3 => Key::F3,
            KeyCode::F4 => Key::F4,
            KeyCode::F5 => Key::F5,
            KeyCode::F6 => Key::F6,
            KeyCode::F7 => Key::F7,
            KeyCode::F8 => Key::F8,
            KeyCode::F9 => Key::F9,
            KeyCode::F10 => Key::F10,
            KeyCode::F11 => Key::F11,
            KeyCode::F12 => Key::F12,

            KeyCode::ArrowUp => Key::Up,
            KeyCode::ArrowDown => Key::Down,
            KeyCode::ArrowLeft => Key::Left,
            KeyCode::ArrowRight => Key::Right,

            KeyCode::Space => Key::Space,
            KeyCode::Enter => Key::Enter,
            KeyCode::Escape => Key::Escape,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Tab => Key::Tab,
            KeyCode::CapsLock => Key::CapsLock,
            KeyCode::ShiftLeft => Key::LeftShift,
            KeyCode::ControlLeft => Key::LeftControl,
            KeyCode::AltLeft => Key::LeftAlt,
            KeyCode::ShiftRight => Key::RightShift,
            KeyCode::ControlRight => Key::RightControl,
            KeyCode::AltRight => Key::RightAlt,
            KeyCode::SuperLeft | KeyCode::SuperRight => Key::Super,
            KeyCode::Insert => Key::Insert,
            KeyCode::Delete => Key::Delete,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::PrintScreen => Key::PrintScreen,
            KeyCode::ScrollLock => Key::ScrollLock,
            KeyCode::Pause => Key::Pause,
            KeyCode::ContextMenu => Key::Menu,

            _ => Key::Unknown,
        }
    }
}

pub fn update_input(mut event: Event<KeyboardEvent>, mut input_map: ResMut<InputMap>) {
    event.iter().for_each(|event| {
        input_map.set_key(event.key, event.state);
    });
}
