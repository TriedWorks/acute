use std::collections::HashSet;

pub struct Keyboard {
    pub just_pressed: HashSet<i32>,
    pub pressed: HashSet<i32>,
    pub just_released: HashSet<i32>,
}

impl Keyboard {
    pub fn just_pressed(&self, key: i32) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn pressed(&self, key: i32) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_released(&self, key: i32) -> bool {
        self.just_released.contains(&key)
    }

    pub(crate) fn press(&mut self, key: i32) {
        if !self.pressed(key) {
            self.just_pressed.insert(key);
        }
        self.pressed.insert(key);
    }

    pub(crate) fn release(&mut self, key: i32) {
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
