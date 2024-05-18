use crate::vector::Vector;

/// Base viewport class
pub struct Viewport {
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    /// Vector accross the horizontal edge
    pub viewport_u: Vector,
    /// Vector down the vertical edge
    pub viewport_v: Vector,
}

impl Viewport {
    pub fn new(viewport_width: f64, aspect_ratio: f64) -> Self
    where
        Self: Sized,
    {
        let viewport_height = viewport_width / aspect_ratio;
        let viewport_u = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector::new(0.0, -viewport_height, 0.0);

        Self {
            aspect_ratio,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
        }
    }
}

/// Base camera class
pub struct Camera {
    pub focal_length: f64,
    pub camera_center: Vector,
    pub viewport: Viewport,
}

impl Camera {
    /// Default camera with:  
    /// `focal_length` = 1  
    /// `viewport_width` = 2 so that viewport's x coordinate is bounded by (-1, 1)  
    /// `aspect_ratio` = 16 / 9  
    /// `camera_center` = (0, 0, 0)
    pub fn default() -> Self
    where
        Self: Sized,
    {
        let focal_length = 1.0;
        let viewport_width = 2.0;
        let aspect_ratio = 16.0 / 9.0;
        let viewport = Viewport::new(viewport_width, aspect_ratio);
        let camera_center = Vector::default();

        Self {
            focal_length,
            camera_center,
            viewport,
        }
    }

    pub fn new(
        focal_length: f64,
        aspect_ratio: f64,
        viewport_width: f64,
        camera_center: Vector,
    ) -> Self
    where
        Self: Sized,
    {
        let viewport = Viewport::new(viewport_width, aspect_ratio);

        Self {
            focal_length,
            camera_center,
            viewport,
        }
    }
}
