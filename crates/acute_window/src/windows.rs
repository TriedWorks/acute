use crate::{Window, WindowId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Windows {
    pub windows: HashMap<WindowId, Window>,
}

impl Windows {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn get(&self, id: WindowId) -> Option<&Window> {
        self.windows.get(&id)
    }

    pub fn get_mut(&mut self, id: WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    pub fn add(&mut self, window: Window) {
        self.windows.insert(window.id.clone(), window);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Window> {
        self.windows.values()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Window> {
        self.windows.values_mut()
    }

    pub fn remove(&mut self, id: WindowId) -> Option<Window> {
        self.windows.remove(&id)
    }
}
