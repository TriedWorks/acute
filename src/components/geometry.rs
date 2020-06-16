use ultraviolet::{
    Vec3
};

use crate::graphics::types::{Renderable, Vertex};
use crate::components::simple::{Transform, Color};

/// m is the transform point,
/// A, B, C are vectors from the transform point and represent Vertices!
///     C
///     m
///   A   B
#[derive(Debug, Copy, Clone)]
pub struct Triangle2D {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Renderable for Triangle2D {
    fn vertices_of(triangle: &Self, transform: &Transform, color: Option<&Color>) -> Vec<Vertex> {
        let color = match color { Some(color) => color.clone(), None => Color { data: [1.0, 0.0, 1.0, 1.0] }};
        let position: Vec3 = transform.pos;
        let a = position + triangle.a;
        let b = position + triangle.b;
        let c = position + triangle.c;
        let vertices = vec![
            Vertex {
                position: a.into(),
                color: color.data,
            },
            Vertex {
                position: b.into(),
                color: color.data
            },
            Vertex {
                position: c.into(),
                color: color.data,
            }
        ];
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
