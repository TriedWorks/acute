use crate::State;
use crate::{Key, MouseButton};

#[derive(Debug, Copy, Clone)]
pub struct KeyboardEvent {
    pub key: Option<Key>,
    pub state: State,
}
#[derive(Debug, Copy, Clone)]
pub struct MouseButtonEvent {
    pub button: Option<MouseButton>,
}
#[derive(Debug, Copy, Clone)]
pub struct MouseScrollEvent {
    pub scroll: (f32, f32),
}
#[derive(Debug, Copy, Clone)]
pub struct MouseMoveEvent {
    pub position: (f64, f64),
}
