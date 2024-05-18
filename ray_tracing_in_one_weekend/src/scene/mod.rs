pub mod camera;
pub mod scene_objects;

use camera::Camera;
use scene_objects::SceneObject;

/// Describes the whole scene:
/// - camera & viewport
/// - scene objects
/// - background
pub struct Scene {
	camera: Camera,
	objects: Vec<Box<dyn SceneObject>>,
	// background: Image
}

impl Scene {
	pub fn new(camera: Camera) -> Self {
		let objects = vec![];

		Self { camera, objects }
	}

	pub fn add_object<T: SceneObject + 'static>(&mut self, object: T) {
		let obj = Box::new(object);
		self.objects.push(obj);
	}
}
