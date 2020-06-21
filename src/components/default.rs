use ultraviolet::{
    Vec3,
    Rotor3,
};
use legion::prelude::*;

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
