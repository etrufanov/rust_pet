pub mod scene_objects;

use crate::{camera::Camera, image_generators_catalog::{Image, ImageShape}, vector::Vector};
use super::PixelData;

/// Base class to define how image relates to camera 
pub struct ImageWithCamera {
	image: Image,
	camera: Camera,
	/// horizontal delta vector from pixel to pixel
	pixel_delta_u: Vector,
	/// vertical delta vector from pixel to pixel
	pixel_delta_v: Vector,
	/// the vieport's upper left corner location
	viewport_upper_left: Vector,
	/// the upper left pixel's location
	pixel00_loc: Vector,
}

impl ImageWithCamera {
    pub fn new(
		image: Image,
		camera: Camera,
	) -> Self
	where
		Self: Sized,
	{
		let [image_width, image_height] = image.shape();

		let pixel_delta_u = camera.viewport.viewport_u / (image_width as f64);
		let pixel_delta_v = camera.viewport.viewport_v / (image_height as f64);

		let viewport_upper_left = camera.camera_center -
			Vector::new(0.0, 0.0, camera.focal_length) -
			camera.viewport.viewport_u / 2.0 -
			camera.viewport.viewport_v / 2.0;
		let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

		// println!("\
		// viewport_u: {:?}, viewport_v: {:?}\n
		// pixel_delta_u: {pixel_delta_u:?}, pixel_delta_v: {pixel_delta_v:?}\n
		// viewport_upper_left: {viewport_upper_left:?}, pixel00_loc: {pixel00_loc:?}\
		// ", camera.viewport.viewport_u, camera.viewport.viewport_v);

		Self { image, camera, pixel_delta_u, pixel_delta_v, viewport_upper_left, pixel00_loc }
	}

	/// Returns delta vector from camera center to pixel
	pub fn get_pixel_vector(&self, x: u16, y: u16) -> Vector {
		let pixel_center = self.pixel00_loc + (self.pixel_delta_u * (x as f64)) + (self.pixel_delta_v * (y as f64));
		let vector = pixel_center - self.camera.camera_center;

		vector
	}
}

impl ImageShape for ImageWithCamera {
	fn shape(&self) -> [u16; 2] {
		self.image.shape()
	}
}