use uuid::Uuid;
use std::io::Error;
use crate::types::Image;

mod asset_server;
mod types;

pub use asset_server::Assets;
pub use types::*;