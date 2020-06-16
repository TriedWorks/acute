use ultraviolet::{
    Vec3,
    Rotor3,
};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub pos: Vec3,
    pub scale: Vec3,
    pub rotation: Rotor3,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub data: [f32; 4],
}

