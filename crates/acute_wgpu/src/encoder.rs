#[derive(Default, Debug)]
pub struct Encoder {
    encoder: Option<wgpu::CommandEncoder>,
}

impl Encoder {
    pub fn get_or_create(&mut self, device: &wgpu::Device) -> &mut wgpu::CommandEncoder {
        match self.encoder {
            Some(ref mut encoder) => encoder,
            None => {
                self.create_encoder(device);
                self.encoder.as_mut().unwrap()
            }
        }
    }

    pub fn create_encoder(&mut self, device: &wgpu::Device) {
        let encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.encoder = Some(encoder);
    }

    pub fn is_some(&self) -> bool {
        self.encoder.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.encoder.is_none()
    }

    pub fn take(&mut self) -> Option<wgpu::CommandEncoder> {
        self.encoder.take()
    }

    pub fn set(&mut self, encoder: wgpu::CommandEncoder) {
        self.encoder = Some(encoder)
    }

    pub fn finish(&mut self, queue: &mut wgpu::Queue) {
        queue.submit(std::iter::once(self.encoder.take().unwrap().finish()))
    }
}
