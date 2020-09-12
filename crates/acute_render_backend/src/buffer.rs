use uuid::Uuid;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct BufferId(Uuid);

impl BufferId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub fn new_buffer(device: &wgpu::Device, descriptor: &wgpu::BufferDescriptor) -> wgpu::Buffer {
    device.create_buffer(descriptor)
}