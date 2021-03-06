use legion::prelude::*;
use ultraviolet::{Rotor3, Vec3};

// #[derive(Debug, Copy, Clone)]
// pub struct Camera {
//     pub
// }

#[derive(Debug, Copy, Clone)]
pub struct ID(pub i32);

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Rotor3,
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub data: [f32; 4],
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group<'a>(pub &'a str);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Static;

pub struct Light {
    pub position: Vec3,
    offset: u32,
    color: Vec3,
}
