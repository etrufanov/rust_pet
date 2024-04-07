use super::{Image, ImageShape, PixelData, IMG_MAX_COLOR};

/// Custom image type containing base `Image`
pub struct BlackAndWhiteGradient{
    image: Image
}

impl BlackAndWhiteGradient {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            image: Image::new(width, height)
        }
    }
}

impl PixelData for BlackAndWhiteGradient {
    /// Generates each pixel data to result in black and white gradient image
    fn get_pixel_rgb(&self, _x: usize, y: usize) -> [u8; 3] {
        [
            ((IMG_MAX_COLOR as usize) * y / self.image.width) as u8,
            ((IMG_MAX_COLOR as usize) * y / self.image.width) as u8,
            ((IMG_MAX_COLOR as usize) * y / self.image.width) as u8,
        ]
    }
}

impl ImageShape for BlackAndWhiteGradient {
    fn shape(&self) -> [usize; 2] {
        [self.image.height, self.image.width]
    }
}
