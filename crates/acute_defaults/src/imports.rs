pub mod app {
    pub use acute_app::*;
}

pub mod window {
    pub use acute_window::*;
}

pub mod winit {
    pub use acute_winit::*;
}

// pub mod wgpu {
//     pub use acute_wgpu::*;
// }

pub mod input {
    pub use acute_input::*;
}

pub mod assets {
    pub use acute_assets::*;
}

pub mod ecs {
    pub use acute_ecs::*;
}

pub mod defaults {
    pub use crate::*;
}

pub mod prelude {
    pub use crate::DefaultBundle;
    pub use acute_app::prelude::*;
    pub use acute_ecs::prelude::*;
    pub use acute_input::prelude::*;
}
