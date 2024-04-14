pub mod black_and_white_gradient;

/// 0-255 u8 value for each rgb component
const IMG_MAX_COLOR: u8 = 255;

pub trait PixelData {
    /// Generates each pixel data
    fn get_pixel_rgb(&self, x: u16, y: u16) -> [u8; 3];
}

pub trait ImageShape {
    /// Returns image shape
    fn shape(&self) -> [u16; 2];
}

/// Base image type with defined shape (aspect ratio and width)
pub struct Image {
    _aspect_ratio: f64,
    width: u16,
    height: u16,
}

impl Image {
    pub fn new(aspect_ratio: f64, width: u16) -> Self
    where
        Self: Sized,
    {
        let height = u16::try_from(((width as f64) / aspect_ratio).round() as i64)
            .expect("image height should be not greater than 65,536 px");
        Self {
            _aspect_ratio: aspect_ratio,
            width,
            height,
        }
    }
}
