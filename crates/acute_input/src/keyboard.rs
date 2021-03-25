use crate::events::KeyboardEvent;
use crate::{Key, State};
use acute_app::Events;
use acute_ecs::system;
use std::collections::HashSet;

pub struct Keyboard {
    pub just_pressed: HashSet<Key>,
    pub pressed: HashSet<Key>,
    pub just_released: HashSet<Key>,
}

impl Keyboard {
    pub fn just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_released(&self, key: Key) -> bool {
        self.just_released.contains(&key)
    }

    pub(crate) fn press(&mut self, key: Key) {
        if !self.pressed(key) {
            self.just_pressed.insert(key);
        }
        self.pressed.insert(key);
    }

    pub(crate) fn release(&mut self, key: Key) {
        self.pressed.remove(&key);
        self.just_released.insert(key);
    }

    pub(crate) fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn new() -> Self {
        Self {
            just_pressed: Default::default(),
            pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

#[system]
pub fn keyboard_update(
    #[resource] keyboard: &mut Keyboard,
    #[resource] events: &Events<KeyboardEvent>,
) {
    keyboard.clear();
    let mut event_reader = events.get_reader();
    for event in event_reader.iter(&events) {
        if let KeyboardEvent {
            key: Some(key),
            state,
        } = event
        {
            match state {
                State::Pressed => keyboard.press(*key),
                State::Released => keyboard.release(*key),
            }
        }
    }
}
