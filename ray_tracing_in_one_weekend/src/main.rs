mod scene;
mod utils;
mod vector;

use std::path::Path;

use scene::{
    camera::Camera,
    scene_objects::{sphere::Sphere, SceneObject},
    Scene,
};
use utils::write_img_arr_to_file::write_img_arr_to_file;
use vector::Vector;

const IMAGES_DIR: &str = "./src/images";

fn main() {
    let camera = Camera::default();

    let mut scene = Scene::new(camera);

    let scene_objects: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(
            Vector::new(0.0, 0.0, -1.0),
            0.3,
            [255, 255, 255],
            Some(1.0),
        )),
        Box::new(Sphere::new(
            Vector::new(0.0, -100.3, -1.0),
            100.0,
            [255, 255, 255],
            Some(1.0),
        )),
    ];
    scene.add_objects(scene_objects);

    let img_width = 480;
    let antialiasing_iters = Some(100);
    let reflection_max_iters = Some(50);

    if let Some(render_result) = scene.render(img_width, antialiasing_iters, reflection_max_iters) {
        let white_sphere_path: &Path = &Path::new(IMAGES_DIR).join("diffuse_sphere.ppm");

        write_img_arr_to_file(white_sphere_path, render_result);
    }
}
