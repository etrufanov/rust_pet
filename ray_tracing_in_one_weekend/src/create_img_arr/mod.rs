use crate::image_generators_catalog::{ImageShape, PixelData};

/// Creates an image representation in array (`Vec<Vec<[u8; 3]>>`)
/// with given `img_width` x `img_height` size
/// calling get_pixel_rgb to obtain each pixel data
pub fn create_img_arr<T: PixelData + ImageShape>(img: &T) -> Vec<Vec<[u8; 3]>> {
    let [img_width, img_height] = img.shape();

    let img_arr = (0..img_height)
        .into_iter()
        .map(|y| {
            (0..img_width)
                .into_iter()
                .map(|x| img.get_pixel_rgb(x, y))
                .collect::<Vec<[u8; 3]>>()
        })
        .collect::<Vec<Vec<[u8; 3]>>>();

    img_arr
}
