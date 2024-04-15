use super::{Image, ImageShape, PixelData, IMG_MAX_COLOR};

/// Black and white gradient image
pub struct BlackAndWhiteGradient {
    image: Image,
}

impl BlackAndWhiteGradient {
    pub fn new(aspect_ratio: f64, width: u16) -> Self {
        Self {
            image: Image::new(aspect_ratio, width),
        }
    }
}

impl PixelData for BlackAndWhiteGradient {
    /// Generates each pixel data to result in black and white gradient image
    fn get_pixel_rgb(&self, _x: u16, y: u16) -> [u8; 3] {
        let [image_width, _image_height] = self.image.shape();

        [
            ((IMG_MAX_COLOR as u16) * y / image_width) as u8,
            ((IMG_MAX_COLOR as u16) * y / image_width) as u8,
            ((IMG_MAX_COLOR as u16) * y / image_width) as u8,
        ]
    }
}
