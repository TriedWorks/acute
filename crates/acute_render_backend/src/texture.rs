use uuid::Uuid;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct TextureId(Uuid);

impl TextureId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}