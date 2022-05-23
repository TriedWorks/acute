use crate::events::{MouseButtonEvent, MouseMoveEvent, MouseScrollEvent};
use crate::MouseButton;
use acute_ecs::event::EventReader;
use acute_ecs::system::ResMut;
use std::collections::HashSet;

#[derive(Debug, Default)]
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

    pub fn new() -> Self {
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

pub fn mouse_button_update_system(
    mut mouse: ResMut<Mouse>,
    mut events: EventReader<MouseButtonEvent>,
) {
    mouse.clear();
    for event in events.iter() {
        if let MouseButtonEvent {
            button: Some(button),
            ..
        } = event
        {
            mouse.toggle(*button)
        }
    }
}

pub fn mouse_scroll_update_system(
    mut mouse: ResMut<Mouse>,
    mut events: EventReader<MouseScrollEvent>,
) {
    for event in events.iter() {
        let MouseScrollEvent { scroll } = event;
        mouse.update_scroll(*scroll)
    }
}

pub fn mouse_move_update_system(mut mouse: ResMut<Mouse>, mut events: EventReader<MouseMoveEvent>) {
    for event in events.iter() {
        let MouseMoveEvent { position } = event;
        mouse.update_position(*position)
    }
}
