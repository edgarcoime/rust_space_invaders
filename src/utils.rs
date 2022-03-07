use std::path::Path;
use bevy::{prelude::*, render::texture::ImageType};
use crate::SPRITE_DIR;


pub fn load_image(images: &mut ResMut<Assets<Image>>, path: &str) -> (Handle<Image>, Vec2) {
	let path = Path::new(SPRITE_DIR).join(path);
	let bytes = std::fs::read(&path).expect(&format!("Cannot find {}", path.display()));
	let image = Image::from_buffer(&bytes, ImageType::MimeType("image/png")).unwrap();
	let size = image.texture_descriptor.size;
	let size = Vec2::new(size.width as f32, size.height as f32);
	let image_handle = images.add(image);
	(image_handle, size)
}