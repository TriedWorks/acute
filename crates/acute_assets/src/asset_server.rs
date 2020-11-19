
use std::collections::HashMap;
use uuid::Uuid;
use crate::types::Asset;
use crate::AssetKind;
use crate::types::Image;
use std::path::PathBuf;

pub struct Assets {
    asset_folder: PathBuf,
    assets: HashMap<Uuid, Asset>,
}

impl Assets {
    pub fn new(asset_path: PathBuf) -> Self {
        Self {
            asset_folder: asset_path,
            assets: Default::default()
        }
    }

    pub fn load(&mut self, path: &str, kind: AssetKind) {
        let mut location = self.asset_folder.clone();
        location.push(path);
        println!("complete path: {:?}", location);
        match kind {
            AssetKind::Image => {
                self.assets.insert(Uuid::new_v4(), Asset::Image(Image::load(location.to_str().unwrap())));
                println!("Added image: {}", path)
            }
            AssetKind::Texture => {}
            AssetKind::Object => {}
            AssetKind::Shader(_) => {}
            AssetKind::Spirv => {}
        }
    }
}
unsafe impl Send for Assets { }
unsafe impl Sync for Assets { }

impl Default for Assets {
    fn default() -> Self {
        let mut asset_path = std::env::current_dir().unwrap();
        asset_path.push("assets");
        Self {
            asset_folder: asset_path,
            assets: Default::default(),
        }
    }
}