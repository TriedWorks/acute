pub mod assets;
pub mod types;

use acute_app::{AppBuilder, Plugin};
pub use assets::Assets;
pub use types::*;

#[derive(Default)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Assets::default());
    }
}
