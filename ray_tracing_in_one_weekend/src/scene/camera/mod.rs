use crate::vector::Vector;

/// Base viewport class
pub struct Viewport {
	pub viewport_width: f64,
	pub viewport_height: f64,
	/// Vector accross the horizontal edge
	pub viewport_u: Vector,
	/// Vector down the vertical edge
	pub viewport_v: Vector,
}

impl Viewport {
    pub fn new(
		viewport_width: f64,
		aspect_ratio: f64,
	) -> Self
	where
		Self: Sized,
	{
        let viewport_height = viewport_width / aspect_ratio;
		let viewport_u = Vector::new(viewport_width, 0.0, 0.0);
		let viewport_v = Vector::new(0.0, -viewport_height, 0.0);

		Self {
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
    pub fn default(
		aspect_ratio: f64,
		viewport_width: f64,
	) -> Self
	where
		Self: Sized,
	{
		let focal_length = 1.0;
		let camera_center = Vector::default();
		let viewport = Viewport::new(viewport_width, aspect_ratio);

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
