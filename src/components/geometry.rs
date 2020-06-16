use ultraviolet::{
    Vec3
};

use crate::graphics::types::{Renderable, Vertex};
/// m is the transform point,
/// A, B, C are vectors from the transform point and represent Vertices!
///     C
///     m
///   A   B
pub struct Triangle2D {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

impl Renderable for Triangle2D {
    fn to_vertices(&self) -> Vec<Vertex> {
        let mut vertices: Vec<Vertex> = Vec::new();
        // vertices.push(Vertex {})
        vertices
    }
}

impl Default for Triangle2D {
    fn default() -> Self {
        Self {
            a: Vec3::new(-1.0, -1.0, 0.0),
            b: Vec3::new(1.0, -1.0, 0.0),
            c: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}

pub struct Pyramid {

}

pub struct Rectangle2D {

}
