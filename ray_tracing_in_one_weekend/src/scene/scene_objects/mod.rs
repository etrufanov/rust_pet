pub mod sphere;

use crate::vector::{Ray, Vector};

use super::Color;

/// Describes the interaction between a ray from a pixel and an object in a scene
pub trait RayToObjectHandler {
    /// Returns a list of intersection coordinates (coordinates are represented as `Vector`)
    /// between the given ray from the pixel (which is defined by `pixel_vec`) and a scene object.
    ///
    /// The intersection coordinates are sorted within the list by z-axis coordinate
    fn calc_ray_intersection(&self, ray: &Ray) -> Option<Vector>;

    /// Computes normal vector (unit length) at the given coordinates
    fn get_normal_vector(&self, coordinates: &Vector) -> Vector;
}

/// Describes an object's appearance
pub trait ObjectAppearance {
    /// Returns an object's color at the given pixel_vec
    fn get_color(&self, ray: &Ray) -> Color;

    fn get_diffusion(&self) -> f64;
}

/// Trait for an abstract scene object
pub trait SceneObject: RayToObjectHandler + ObjectAppearance {}
impl<T: RayToObjectHandler + ObjectAppearance> SceneObject for T {}
