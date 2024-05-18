pub mod camera;
pub mod render;
pub mod scene_objects;
mod utils;

use camera::Camera;
use scene_objects::SceneObject;

use self::render::Renderer;

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
