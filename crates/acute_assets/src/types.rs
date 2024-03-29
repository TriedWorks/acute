use downcast_rs::{impl_downcast, Downcast};
use image::io::Reader as ImageReader;
use image::{Bgra, DynamicImage, EncodableLayout, GenericImageView, ImageBuffer};

pub enum AssetKind {
    Image,
    Texture,
    Object,
    Shader(ShaderKind),
}

pub enum ShaderKind {
    Vertex,
    Fragment,
}

pub trait Asset: Downcast {
    fn as_bytes(&self) -> &[u8];
}

impl_downcast!(Asset);

pub struct Image {
    /// image is the underlying and editable image
    image: DynamicImage,
    /// acute requires Bgra8 images right now, so 'image' is edited and saved to 'exposed' as Bgra8
    exposed: ImageBuffer<Bgra<u8>, Vec<u8>>,
    path: String,
}

impl Image {
    pub fn load(path: &str) -> Self {
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        let exposed = image.to_bgra8();
        let path = path.to_string();
        Image {
            image,
            exposed,
            path,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.image.dimensions()
    }

    pub fn flip_horizontal(&mut self) -> &mut Self {
        self.image = self.image.fliph();
        self
    }

    pub fn flip_vertical(&mut self) -> &mut Self {
        self.image = self.image.flipv();
        self
    }

    pub fn rotate90(&mut self) -> &mut Self {
        self.image = self.image.rotate90();
        self
    }

    pub fn rotate180(&mut self) -> &mut Self {
        self.image = self.image.rotate180();
        self
    }

    pub fn set_contrast(&mut self, contrast: f32) -> &mut Self {
        self.image = self.image.adjust_contrast(contrast);
        self
    }

    pub fn set_blur(&mut self, blur: f32) -> &mut Self {
        self.image = self.image.blur(blur);
        self
    }

    pub fn set(&mut self) -> &mut Self {
        self.exposed = self.image.to_bgra8();
        self
    }
}

impl Asset for Image {
    fn as_bytes(&self) -> &[u8] {
        self.exposed.as_bytes()
    }
}

pub struct Shader {
    kind: ShaderKind,
    raw: String,
}

impl Shader {
    pub fn load(path: &str, kind: ShaderKind) -> Self {
        let raw = std::fs::read_to_string(path).unwrap();
        Self { kind, raw }
    }
}

impl Asset for Shader {
    fn as_bytes(&self) -> &[u8] {
        self.raw.as_bytes()
    }
}
