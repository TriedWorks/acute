use ultraviolet::{
    Vec4,
    Vec3,
    Vec2,
};
use crate::components::material::Material;

/// Mesh struct contains the data and the topology settings
/// e.g. point, line or triangle for a mesh.
/// Materials are not added by default and must be explicitly added
/// Renderer displays a RBG value of 1, 0, 1 (pink)
/// if one material is added, the whole mesh is the same material
#[derive(Debug, Clone)]
pub struct Mesh {
    pub data: MeshData,
    pub primitive: wgpu::PrimitiveTopology,
}

/// MeshData struct contains the data for a mesh.
/// This struct is more of a helper struct and is not
/// intended to be used without a mesh as its parent.
/// if no indices are added, index rendering won't be used
#[derive(Debug, Clone)]
pub struct MeshData {
    /// All vertices of the mesh
    pub data: Vec<Vec3>,
    /// indices of the data vec, used to mark lines / triangles
    pub indices: Vec<u32>,
    // [("material_name", indices_group)]
    /// material for a single line / triangle of a mesh,
    /// the u32 is the group index of the indices vec
    pub material_groups: Vec<(String, u32)>,
    /// normal per vertex, empty if no normal exists
    pub normals: Vec<Vec3>,
}

impl Mesh {
    /// Creates a new triangle
    /// creates a default triangle if no data is supplied
    pub fn new_triangle(data: Option<&[Vec3; 3]>) -> Mesh {
        let data = match data {
            Some(data) => data.to_vec(),
            None => DEFAULT_TRIANGLE.to_vec(),
        };
        Mesh {
            data: MeshData {
                data,
                indices: vec![0, 1, 2],
                material_groups: vec![],
                normals: vec![],
            },
            primitive: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    pub fn new_quad(data: Option<&[Vec3; 4]>) -> Mesh {
        let data = match data {
            Some(data) => data.to_vec(),
            None => DEFAULT_QUAD.to_vec(),
        };
        Mesh {
            data: MeshData {
                data,
                indices: vec![0, 1, 3, 2, 3, 4],
                material_groups: vec![],
                normals: vec![],
            },
            primitive: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    /// creates a new tetrahedron with no material
    pub fn new_tetrahedron(data: Option<&[Vec3; 5]>) -> Mesh {
        let data = match data {
            Some(data) => data.to_vec(),
            None => DEFAULT_TETRAHEDRON.to_vec(),
        };

        Mesh {
            data: MeshData {
                data,
                indices: vec![
                    4, 0, 1,
                    4, 1, 2,
                    4, 2, 1,
                    0, 1, 2,
                ],
                material_groups: vec![],
                normals: vec![],
            },
            primitive: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    /// creates a new cube with no material
    /// via scaling the cube can be used to represent any rectangular cuboid
    pub fn new_cube(data: Option<&[Vec3; 8]>) -> Mesh {
        let data = match data {
            Some(data) => data.to_vec(),
            None => DEFAULT_HEXAHEDRON.to_vec(),
        };

        Mesh {
            data: MeshData {
                data,
                indices: vec![
                    0, 1, 3,    // top
                    1, 2, 3,
                    4, 5, 7,    // bottom
                    5, 6, 7,
                    3, 7, 0,    // behind
                    7, 4, 0,
                    1, 5, 2,    // front
                    5, 6, 2,
                    0, 4, 1,    // left
                    4, 5, 1,
                    2, 6, 3,    // right
                    6, 7, 3,
                ],
                material_groups: vec![],
                normals: vec![],
            },
            primitive: wgpu::PrimitiveTopology::TriangleList,
        }
    }
}

//    side
//      0
//    /  \
//   1----2
const DEFAULT_TRIANGLE: [Vec3; 3] = [
    Vec3 { x: 0.0, y: 0.5, z: 0.0 },
    Vec3 { x: -0.5, y: -0.5, z: 0.0 },
    Vec3 { x: 0.5, y: -0.5, z: 0.0 },
];

//     top
//   0----4
//   |    |
//   1----2
const DEFAULT_QUAD: [Vec3; 4] = [
    Vec3 { x: -0.5, y: 0.5, z: 0.0 },
    Vec3 { x: -0.5, y: -0.5, z: 0.0 },
    Vec3 { x: 0.5, y: -0.5, z: 0.0 },
    Vec3 { x: 0.5, y: 0.5, z: 0.0 },
];

//    side        top
//      0       1----4
//    /  \      |  0 |
//   1----2     2----3
const DEFAULT_TETRAHEDRON: [Vec3; 5] = [
    Vec3 { x: 0.0, y: 0.5, z: 0.0 },
    Vec3 { x: -0.5, y: -0.5, z: 0.5 },
    Vec3 { x: -0.5, y: -0.5, z: -0.5 },
    Vec3 { x: 0.5, y: -0.5, z: -0.5 },
    Vec3 { x: 0.5, y: -0.5, z: 0.5 },
];

//     top     bottom       side
//   0----3    4----7      1----2
//   |    |    |    |  ->  |    |
//   1----2    5----6      5----6
const DEFAULT_HEXAHEDRON: [Vec3; 8] = [
    Vec3 { x: -0.5, y: 0.5, z: 0.5 },
    Vec3 { x: -0.5, y: 0.5, z: -0.5 },
    Vec3 { x: 0.5, y: 0.5, z: -0.5 },
    Vec3 { x: 0.5, y: 0.5, z: 0.5 },
    Vec3 { x: -0.5, y: -0.5, z: 0.5 },
    Vec3 { x: -0.5, y: -0.5, z: -0.5 },
    Vec3 { x: 0.5, y: -0.5, z: -0.5 },
    Vec3 { x: 0.5, y: -0.5, z: 0.5 },
];