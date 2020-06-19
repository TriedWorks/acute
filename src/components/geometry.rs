#![allow(dead_code, unused_imports)]

use ultraviolet::{
    Vec3,
    Rotor3,
};

use crate::graphics::types::{Renderable, VertexC};
use crate::components::simple::{Transform, Color};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>
}

impl Mesh {
    pub fn update_rotation(&mut self, rotor: Rotor3) {
        for i in 0..self.vertices.len() {
            let mut vertex_vector: Vec3 = self.vertices[i].position.into();
            rotor.rotate_vec(&mut vertex_vector);
            self.vertices[i] = Vertex {position: [vertex_vector.x, vertex_vector.y, vertex_vector.z] };
        }
    }
}

impl Renderable for Mesh {
    fn vertices_of(&self, transform: &Transform, color: Option<&Color>) -> Vec<VertexC> {
        let mut colored_vertices: Vec<VertexC> = Vec::new();
        let color = match color {
            Some(color) => { color.clone() },
            None => Color { data: [0.0, 0.0, 0.0, 1.0] }
        };

        for vertex in &self.vertices {
            let new_pos = [vertex.position[0] + transform.position.x, vertex.position[1] + transform.position.y, vertex.position[2] + transform.position.z];
            let colored_vertex: VertexC = VertexC { position: new_pos, color: color.data };
            colored_vertices.push(colored_vertex)
        }

        colored_vertices
    }
}

pub type Triangle = Mesh;

impl Triangle {
    pub fn new(vertices: &[Vertex; 3]) -> Triangle {
        let vertices: Vec<Vertex> = vertices.to_vec();
        Triangle {
            vertices,
        }
    }

    pub fn new_mesh(triangles: &Vec<Triangle>) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        for triangle in triangles {
            vertices.extend(&triangle.vertices);
        }
        Mesh {
            vertices,
        }
    }
}