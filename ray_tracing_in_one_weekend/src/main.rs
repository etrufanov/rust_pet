mod create_img_arr;
mod image_generators_catalog;
mod write_img_arr_to_file;

use std::path::Path;

use create_img_arr::create_img_arr;
use image_generators_catalog::black_and_white_gradient::BlackAndWhiteGradient;
use write_img_arr_to_file::write_img_arr_to_file;

const IMAGES_DIR: &str = "./src/images";

fn main() {
    let black_and_white_gradient_path: &Path =
        &Path::new(IMAGES_DIR).join("black_and_white_gradient.ppm");

    let aspect_ratio = 1.0;
    let width = 512;
    let black_and_white_gradient = BlackAndWhiteGradient::new(aspect_ratio, width);
    let black_and_white_gradient_arr = create_img_arr(&black_and_white_gradient);

    write_img_arr_to_file(black_and_white_gradient_path, black_and_white_gradient_arr);
}
