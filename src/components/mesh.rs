use ultraviolet::{
    Vec4,
    Vec3,
    Vec2
};

#[derive(Debug, Clone)]
pub struct Mesh {
    pub data: MeshData,
    pub primitive: wgpu::PrimitiveTopology,
}

pub struct MeshData {
    pub data: Vec<Vec3>,
    pub indices: Vec<u32>,
    // [("material_name", indices_group)]
    pub material_groups: Vec<(String, u32)>,
    pub normals: Vec<Vec3>,
}