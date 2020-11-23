pub struct Events {
    system: SystemEvent,
    render: RenderEvent
}

impl Events {
    pub fn change_system(&mut self, event: SystemEvent) {
        self.system = event;
    }

    pub fn change_render(&mut self, event: RenderEvent) {
        self.render = event;
    }
}

impl Default for Events {
    fn default() -> Self {
        Self {
            system: SystemEvent::Run,
            render: RenderEvent::Nothing
        }
    }
}


pub enum SystemEvent {
    Run,
    Exit
}

pub enum RenderEvent {
    Nothing,
    Resize((u32, u32)),
    RedrawRequest,
}