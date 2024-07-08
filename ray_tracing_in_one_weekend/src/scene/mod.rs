pub mod camera;
pub mod render;
pub mod scene_objects;
mod utils;

use camera::Camera;
use scene_objects::SceneObject;

use crate::vector::Vector;

use self::render::Renderer;

/// Object's color as [r, g, b]
pub type Color = [u8; 3];

impl From<Vector> for Color {
    fn from(value: Vector) -> Self {
        let value_clamped = value.clamp(0.0, 1.0);
        [
            (value_clamped.x() * 255.0).round() as u8,
            (value_clamped.y() * 255.0).round() as u8,
            (value_clamped.z() * 255.0).round() as u8,
        ]
    }
}

/// Describes the whole scene:
/// - camera & viewport
/// - scene objects
/// - render params
/// - background
pub struct Scene {
    camera: Camera,
    objects: Vec<Box<dyn SceneObject>>,
    renderer: Renderer,
}

impl Scene {
    pub fn new(camera: Camera) -> Self {
        let objects = vec![];
        let renderer = Renderer::empty();

        Self {
            camera,
            objects,
            renderer,
        }
    }

    pub fn add_object(&mut self, object: Box<dyn SceneObject>) {
        self.objects.push(object);
    }

    pub fn add_objects(&mut self, objects: Vec<Box<dyn SceneObject>>) {
        self.objects.extend(objects);
    }
}
