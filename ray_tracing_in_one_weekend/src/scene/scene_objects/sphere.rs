use crate::vector::Vector;

use super::RayToObjectHandler;

#[derive(Clone, Copy)]
/// sphere object inside a scene
pub struct Sphere {
	center: Vector,
	radius: f64,
	pub color: [u8; 3],
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: [u8; 3]) -> Self
	where
		Self: Sized,
	{
		Self { center, radius, color }
	}

	/// default sphere object with unit radius centered at (0, 0, 0)
	pub fn default() -> Self
	where
		Self: Sized,
	{
		Self { center: Vector::default(), radius: 1.0, color: [0, 0, 0] }
	}
}

impl RayToObjectHandler for Sphere {
	fn does_ray_intersect(&self, pixel_vec: &Vector) -> bool {
		// 1. system of equations (underscores stands for vector):
		//  - 
		// /  radius^2 = ||_center_ - _r_||^2
		// \  _r_ = _pixel_vec_ * t
		//  -
		// for sphere and ray respectively
		// 
		// 2. that implies:
		// radius^2 = ||_center_||^2 + t^2||_pixel_vec_||^2 - 2t(_center_, _pixel_vec_)
		// 
		// 3. quadratic equation has roots in Real space if and only if:
		// discriminant^2 = 2^2 * (_center_, _pixel_vec_)^2 - 4 * ||_pixel_vec_||^2 * (||_center_||^2 - radius^2) >= 0
		//                              b^2                 - 4 *         a         *                 c
		// 
		// 4. therefore the ray does intersect the sphere if and only if discriminant >= 0

		let b = 2.0 * self.center.dot(pixel_vec);
		let a = pixel_vec.sq_norm();
		let c = self.center.sq_norm() - self.radius.powi(2);

		let does_ray_intersect = b.powi(2) >= 4.0 * a * c;
		does_ray_intersect
	}
}
