use ultraviolet::{
    Vec4,
};


pub struct Material {
    name: String,
    material_type: MaterialType,
}

// TODO: ADD COMPLEX MATERIAL (roughness, metal, etc.)
pub enum MaterialType {
    BasicMaterial(BasicMaterial),
    ComplexMaterial,
}

pub enum BasicMaterial {
    MaterialColor(BasicMaterialColor),
    MaterialTexture(BasicMaterialTexture),
    MaterialColorTexture(BasicMaterialColorTexture),
}

// TODO: CREATE OWN COLOR CRATE BECAUSE WHY NOT (allow different conventions e.g. more than rgba)
pub struct BasicMaterialColor {
    pub color: wgpu::Color,
}

pub struct BasicMaterialTexture {
    pub texture_name: String,
}

pub struct BasicMaterialColorTexture {
    pub color: wgpu::Color,
    pub texture: String,
}