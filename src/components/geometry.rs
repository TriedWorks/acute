#![allow(dead_code, unused_imports)]

use ultraviolet::{
    Vec3,
    Rotor3,
};

use crate::graphics::types::{Renderable, VertexC};
use crate::components::default::{Transform, Color};

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

    // can take a vec of any meshes or primitive types of meshes for example "triangle" or quad
    pub fn new_mesh(sub_meshes: &Vec<Mesh>) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        for sub_mesh in sub_meshes {
            vertices.extend(&sub_mesh.vertices);
        }
        Mesh {
            vertices,
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

// these "primitive" mesh types are here for easier understanding
pub type Triangle = Mesh;
pub type Quad = Mesh;
pub type Cuboid = Mesh;
pub type Tetrahedron = Mesh;
pub type Pyramid = Mesh;

impl Triangle {
    pub fn new_triangle(vertices: &[Vertex; 3]) -> Triangle {
        let vertices: Vec<Vertex> = vertices.to_vec();
        Triangle {
            vertices,
        }
    }
}

impl Quad {
    pub fn new_quad(vertices: &[Vertex; 4]) -> [Triangle; 2] {
        let triangle_0 = Mesh::new_triangle(&[vertices[0], vertices[1], vertices[3]]);
        let triangle_1 = Mesh::new_triangle(&[vertices[1], vertices[2], vertices[3]]);
        [triangle_0, triangle_1]
    }

    pub fn default_quad() -> [Triangle; 2] {
        Quad::new_quad(&[
            Vertex { position: [-0.5, 0.5, 0.0]},
            Vertex { position: [-0.5, -0.5, 0.0]},
            Vertex { position: [0.5, -0.5, 0.0]},
            Vertex { position: [0.5, 0.5, 0.0]},
        ])
    }

    pub fn default_horizontal_quad() -> [Triangle; 2] {
        Quad::new_quad(&[
            Vertex { position: [-0.5, 0.0, 0.5]},
            Vertex { position: [-0.5, 0.0, -0.5]},
            Vertex { position: [0.5, 0.0, -0.5]},
            Vertex { position: [0.5, 0.0, 0.5]},
        ])
    }
}

impl Cuboid {
    pub fn new_cuboid(vertices: &[Vertex; 8]) -> [Triangle; 12] {
        //bottom top
        let triangle_0 = Mesh::new_triangle(&[vertices[3], vertices[2], vertices[0]]);
        let triangle_1 = Mesh::new_triangle(&[vertices[2], vertices[1], vertices[0]]);

        let triangle_2 = Mesh::new_triangle(&[vertices[4], vertices[5], vertices[7]]);
        let triangle_3 = Mesh::new_triangle(&[vertices[5], vertices[6], vertices[7]]);

        let triangle_4 = Mesh::new_triangle(&[vertices[5], vertices[1], vertices[6]]);
        let triangle_5 = Mesh::new_triangle(&[vertices[1], vertices[2], vertices[6]]);

        let triangle_6 = Mesh::new_triangle(&[vertices[7], vertices[3], vertices[4]]);
        let triangle_7 = Mesh::new_triangle(&[vertices[3], vertices[0], vertices[4]]);

        let triangle_8 = Mesh::new_triangle(&[vertices[4], vertices[0], vertices[5]]);
        let triangle_9 = Mesh::new_triangle(&[vertices[0], vertices[1], vertices[5]]);

        let triangle_10 = Mesh::new_triangle(&[vertices[6], vertices[2], vertices[7]]);
        let triangle_11 = Mesh::new_triangle(&[vertices[2], vertices[3], vertices[7]]);

        [triangle_0, triangle_1, triangle_2, triangle_3,triangle_4, triangle_5,
             triangle_6, triangle_7,triangle_8, triangle_9, triangle_10, triangle_11]
    }

    pub fn default_cuboid() -> [Triangle; 12] {
        Cuboid::new_cuboid(&[
            Vertex {position: [-0.5, -0.5, -0.5]},
            Vertex {position: [-0.5, -0.5, 0.5]},
            Vertex {position: [0.5, -0.5, 0.5]},
            Vertex {position: [0.5, -0.5, -0.5]},

            Vertex {position: [-0.5, 0.5, -0.5]},
            Vertex {position: [-0.5, 0.5, 0.5]},
            Vertex {position: [0.5, 0.5, 0.5]},
            Vertex {position: [0.5, 0.5, -0.5]},
        ])
    }
}

impl Tetrahedron {
    pub fn new_tetrahedron(vertices: &[Vertex; 4]) -> [Triangle; 4]{
        let triangle_0 = Mesh::new_triangle(&[vertices[1], vertices[0], vertices[2]]);
        let triangle_1 = Mesh::new_triangle(&[vertices[3], vertices[0], vertices[1]]);
        let triangle_2 = Mesh::new_triangle(&[vertices[3], vertices[1], vertices[2]]);
        let triangle_3 = Mesh::new_triangle(&[vertices[3], vertices[2], vertices[0]]);

        [triangle_0, triangle_1, triangle_2, triangle_3]
    }

    pub fn default_tetrahedron() -> [Triangle; 4] {
        Tetrahedron::new_tetrahedron(&[
            Vertex {position: [-0.5, -0.5, -0.5]},
            Vertex {position: [0.0, 0.5, -0.5]},
            Vertex {position: [0.5, -0.5, -0.5]},
            Vertex {position: [0.0, 0.0, 0.5]},
        ])
    }
}


impl Pyramid {
    pub fn new_pyramid(vertices: &[Vertex; 5]) -> [Triangle; 6] {
        let triangle_0 = Mesh::new_triangle(&[vertices[0], vertices[1], vertices[3]]);
        let triangle_1 = Mesh::new_triangle(&[vertices[1], vertices[2], vertices[3]]);

        let triangle_2 = Mesh::new_triangle(&[vertices[4], vertices[0], vertices[1]]);
        let triangle_3 = Mesh::new_triangle(&[vertices[4], vertices[1], vertices[2]]);
        let triangle_4 = Mesh::new_triangle(&[vertices[4], vertices[2], vertices[3]]);
        let triangle_5 = Mesh::new_triangle(&[vertices[4], vertices[3], vertices[0]]);

        [triangle_0, triangle_1, triangle_2, triangle_3, triangle_4, triangle_5]
    }

    pub fn default_pyramid() -> [Triangle; 6] {
        Pyramid::new_pyramid(&[
            Vertex {position: [-0.5, -0.5, -0.5]},
            Vertex {position: [-0.5, -0.5, 0.5]},
            Vertex {position: [0.5, -0.5, 0.5]},
            Vertex {position: [0.5, -0.5, -0.5]},
            Vertex {position: [0.0, 0.5, 0.0]},
        ])
    }
}