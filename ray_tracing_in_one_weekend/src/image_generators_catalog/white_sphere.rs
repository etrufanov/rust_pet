use crate::vector::Vector;

use super::{image_with_camera::{scene_objects::{sphere::Sphere, RayToObjectHandler}, ImageWithCamera}, ImageShape, PixelData};

/// image with white sphere
pub struct WhiteSphere {
    image_with_camera: ImageWithCamera,
	object: Sphere,
}

impl WhiteSphere {
    pub fn new(image_with_camera: ImageWithCamera) -> Self {
		let center = Vector::new(0.0, 0.0, -1.0);
		let radius = 0.3;
		let color = [255, 255, 255];

		let object = Sphere::new(center, radius, color);

        Self { image_with_camera, object }
    }
}

impl ImageShape for WhiteSphere {
	fn shape(&self) -> [u16; 2] {
		self.image_with_camera.shape()
	}
}

impl PixelData for WhiteSphere {
    /// Generates each pixel data to result in image with white sphere
    fn get_pixel_rgb(&self, x: u16, y: u16) -> [u8; 3] {
		let ray = self.image_with_camera.get_pixel_vector(x, y);

		let pixel_rgb = if self.object.does_ray_intersect(ray) { self.object.color } else { [0, 0, 0] };
		pixel_rgb
    }
}
