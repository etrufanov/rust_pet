pub mod image_with_camera;
pub mod black_and_white_gradient;
pub mod white_sphere;

/// 0-255 u8 value for each rgb component
const IMG_MAX_COLOR: u8 = 255;

pub trait PixelData {
    /// Generates each pixel data
    fn get_pixel_rgb(&self, x: u16, y: u16) -> [u8; 3];
}

pub trait ImageShape {
    /// Returns image shape `[width, height]`
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

impl ImageShape for Image {
    fn shape(&self) -> [u16; 2] {
        [self.width, self.height]
    }
}
