use std::hash::Hash;

use rustc_hash::{FxHashMap, FxHashSet};

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

pub trait AxisMapping: Sized {
    fn axes<'a>() -> &'a [Axis];
    fn positive_keys<'a>() -> &'a [Key];
    fn positive_buttons<'a>() -> &'a [Button];
    fn negative_keys<'a>() -> &'a [Key];
    fn negative_buttons<'a>() -> &'a [Button];

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
        let positive_keys = FxHashSet::from_iter(M::positive_keys().iter().copied());
        let positive_buttons = FxHashSet::from_iter(M::positive_buttons().iter().copied());
        let negative_keys = FxHashSet::from_iter(M::negative_keys().iter().copied());
        let negative_buttons = FxHashSet::from_iter(M::negative_buttons().iter().copied());

        let positive = !self.keys.is_disjoint(&positive_keys) || !self.buttons.is_disjoint(&positive_buttons);
        let negative = !self.keys.is_disjoint(&negative_keys) || !self.buttons.is_disjoint(&negative_buttons);
        let positive: f32 = positive.into();
        let negative: f32 = negative.into();

        let axis = M::axes().iter().map(|axis| self.get_axis(*axis)).fold(0.0_f32, |max, value| {
            if value.abs() > max.abs() {
                value
            } else {
                max
            }
        });

        axis.max(positive - negative)
    }
}
