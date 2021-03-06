use crate::events::{MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use crate::MouseButton;
use acute_app::Events;
use acute_ecs::system;
use std::collections::HashSet;

pub struct Mouse {
    pub position: (f64, f64),
    pub position_delta: (f64, f64),
    pub scroll: (f32, f32),
    pub scroll_delta: (f32, f32),
    pub just_pressed: HashSet<MouseButton>,
    pub pressed: HashSet<MouseButton>,
    pub just_released: HashSet<MouseButton>,
}

impl Mouse {
    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed.contains(&button)
    }

    pub fn pressed(&self, button: MouseButton) -> bool {
        self.pressed.contains(&button)
    }

    pub fn just_released(&self, button: MouseButton) -> bool {
        self.just_released.contains(&button)
    }

    pub fn reset_scroll(&mut self) {
        self.scroll = (0.0, 0.0);
    }

    pub(crate) fn update_position(&mut self, new_position: (f64, f64)) {
        self.position_delta = (
            self.position.0 - new_position.0,
            self.position.1 - new_position.1,
        );
        self.position = new_position;
    }

    pub(crate) fn update_scroll(&mut self, scroll_delta: (f32, f32)) {
        self.scroll_delta = scroll_delta;
        self.scroll.0 += scroll_delta.0;
        self.scroll.1 += scroll_delta.1;
    }

    pub(crate) fn toggle(&mut self, button: MouseButton) {
        if !self.pressed(button) {
            self.just_pressed.insert(button);
            self.pressed.insert(button);
        } else {
            self.pressed.remove(&button);
            self.just_released.insert(button);
        }
    }

    pub(crate) fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn new() -> Self {
        Self {
            position: (0.0, 0.0),
            position_delta: (0.0, 0.0),
            scroll: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
            just_pressed: Default::default(),
            pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

#[system]
pub fn mouse_button_update(
    #[resource] mouse: &mut Mouse,
    #[resource] events: &Events<MouseButtonEvent>,
) {
    mouse.clear();
    let mut event_reader = events.get_reader();
    for event in event_reader.iter(&events) {
        if let MouseButtonEvent {
            button: Some(button),
            ..
        } = event
        {
            mouse.toggle(*button)
        }
    }
}
#[system]
pub fn mouse_scroll_update(
    #[resource] mouse: &mut Mouse,
    #[resource] events: &Events<MouseScrollEvent>,
) {
    let mut event_reader = events.get_reader();
    for event in event_reader.iter(&events) {
        let MouseScrollEvent { scroll } = event;
        mouse.update_scroll(*scroll)
    }
}

#[system]
pub fn mouse_move_update(
    #[resource] mouse: &mut Mouse,
    #[resource] events: &Events<MouseMoveEvent>,
) {
    let mut event_reader = events.get_reader();
    for event in event_reader.iter(&events) {
        let MouseMoveEvent { position } = event;
        mouse.update_position(*position)
    }
}
