pub mod sphere;

use crate::vector::Vector;

pub trait RayToObjectHandler {
	fn does_ray_intersect(&self, pixel_vec: Vector) -> bool;
}
