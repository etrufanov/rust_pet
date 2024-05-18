mod create_img_arr;
mod image_generators_catalog;
mod write_img_arr_to_file;
// mod camera;
mod vector;
mod scene;

use std::path::Path;

use create_img_arr::create_img_arr;
use image_generators_catalog::{black_and_white_gradient::BlackAndWhiteGradient, image_with_camera::ImageWithCamera, white_sphere::WhiteSphere, Image};
use write_img_arr_to_file::write_img_arr_to_file;
// use camera::Camera;

const IMAGES_DIR: &str = "./src/images";

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    // let viewport_width = 2 so that viewport's x coordinate is bounded by (-1, 1)
    let viewport_width = 2.0;
    let camera = scene::camera::Camera::default(aspect_ratio, viewport_width);

    let mut scene = scene::Scene::new(camera);

    scene.add_object(scene::scene_objects::sphere::Sphere::default());
}

// fn main() {
//     let aspect_ratio = 16.0 / 9.0;

//     // let viewport_width = 2 so that viewport's x coordinate is bounded by (-1, 1)
//     let viewport_width = 2.0;
//     let camera = Camera::default(aspect_ratio, viewport_width);

//     let image_width = 640;
//     let image = Image::new(aspect_ratio, image_width);

//     let image_with_camera = ImageWithCamera::new(image, camera);

//     let white_sphere = WhiteSphere::new(image_with_camera);

//     let white_sphere_path: &Path =
//         &Path::new(IMAGES_DIR).join("white_sphere.ppm");

//     let white_sphere_arr = create_img_arr(&white_sphere);

//     write_img_arr_to_file(white_sphere_path, white_sphere_arr);
// }


// fn main() {
//     let black_and_white_gradient_path: &Path =
//         &Path::new(IMAGES_DIR).join("black_and_white_gradient.ppm");

//     let aspect_ratio = 1.0;
//     let width = 512;
//     let black_and_white_gradient = BlackAndWhiteGradient::new(aspect_ratio, width);
//     let black_and_white_gradient_arr = create_img_arr(&black_and_white_gradient);

//     write_img_arr_to_file(black_and_white_gradient_path, black_and_white_gradient_arr);
// }
