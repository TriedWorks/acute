pub mod assets;
pub mod types;

use acute_app::{AppBuilder, Plugin};
pub use assets::Assets;
pub use types::*;

pub struct AssetPlugin;

impl Default for AssetPlugin {
    fn default() -> Self {
        Self {}
    }
}

impl Plugin for AssetPlugin {
    fn add(&self, app: &mut AppBuilder) {
        app.add_resource(Assets::default());
    }
}
