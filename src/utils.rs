use std::path::Path;
use bevy::{prelude::*, render::texture::ImageType};

#[derive(Component)]
pub struct RenderedAssetInfo {
    pub size: Vec2,
}
impl RenderedAssetInfo {
    pub fn new(size: Vec2) -> Self {
        Self { size }
    }
}

pub fn load_image(images: &mut ResMut<Assets<Image>>, dir: &str, filename: &str) -> (Handle<Image>, Vec2) {
	let path = Path::new(dir).join(filename);
	let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
	let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
	let size = image.texture_descriptor.size;
	let size = Vec2::new(size.width as f32, size.height as f32);
	let image_handle = images.add(image);
	(image_handle, size)
}