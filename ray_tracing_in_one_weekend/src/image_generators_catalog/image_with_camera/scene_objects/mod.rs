pub mod sphere;

use crate::vector::Vector;

/// Describes interaction between a ray from a pixel and an object in a scene
pub trait RayToObjectHandler {
	/// Calculates if the given ray from the pixel (which is defined by pixel_vec)
	/// intersects with object
	fn does_ray_intersect(&self, pixel_vec: Vector) -> bool;
}
