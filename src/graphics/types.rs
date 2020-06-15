use bytemuck::{Zeroable, Pod};
use ultraviolet::{
    Vec3, Vec4,
};


pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
}

unsafe impl Zeroable for Vertex { }
unsafe impl Pod for Vertex {}