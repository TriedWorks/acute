use crate::keyboard::Keyboard;
use crate::mouse::Mouse;
use crate::{State, Key, MouseButton};

pub struct Input {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn update_keyboard(&mut self, key: Key, state: State) {
        self.keyboard.clear();
        match state {
            State::Pressed => self.keyboard.press(key),
            State::Released => self.keyboard.release(key),
        }
    }

    pub fn update_mouse(&mut self, button: MouseButton) {
        self.mouse.clear();
        self.mouse.toggle(button);
    }

    pub fn update_mouse_scroll(&mut self, delta: (f32, f32)) {
        self.mouse.update_scroll(delta);
    }

    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        }
    }
}
