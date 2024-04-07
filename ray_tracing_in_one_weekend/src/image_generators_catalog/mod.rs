pub mod black_and_white_gradient;

/// 0-255 u8 value for each rgb component
const IMG_MAX_COLOR: u8 = 255;

pub trait PixelData {
    /// Generates each pixel data
    fn get_pixel_rgb(&self, x: usize, y: usize) -> [u8; 3];
}

pub trait ImageShape {
    /// Returns image shape
    fn shape(&self) -> [usize; 2];
}

/// Base image type with defined shape (width and height)
pub struct Image {
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self where Self: Sized {
        Self { width, height }
    }
}
