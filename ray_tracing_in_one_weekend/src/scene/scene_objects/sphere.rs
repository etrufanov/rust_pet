use crate::{
    scene::utils::sort_vectors_by_z::sort_vectors_by_z,
    vector::{Ray, Vector},
};

use super::{Color, ObjectAppearance, RayToObjectHandler};

#[derive(Clone, Copy)]
/// Sphere object inside a scene
pub struct Sphere {
    center: Vector,
    radius: f64,
    color: Color,
    diffusion: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Color, diffusion: Option<f64>) -> Self
    where
        Self: Sized,
    {
        Self {
            center,
            radius,
            color,
            diffusion: diffusion.unwrap_or(0.0),
        }
    }

    /// Default sphere object with unit radius centered at (0, 0, 0)
    pub fn default() -> Self
    where
        Self: Sized,
    {
        Self {
            center: Vector::default(),
            radius: 1.0,
            color: [255, 255, 255],
            diffusion: 0.0,
        }
    }
}

impl RayToObjectHandler for Sphere {
    fn calc_ray_intersection(&self, ray: &Ray) -> Option<Vector> {
        // 1. system of equations (underscores stand for vector):
        // --
        // |  radius^2 = ||_center_ - _r_||^2
        // |  _r_ = _origin_ + _direction_ * t
        // --
        // for sphere and ray respectively
        //
        // 2. that implies:
        // radius^2 = ||_center_ - _origin_||^2 + t^2||_direction_||^2 - 2t(_center_ - _origin_, _direction_)
        //
        // 3. quadratic equation has roots in Real space if and only if:
        // discriminant^2 = (-2 * <_center_ - _origin_, _direction_>)^2 - 4 * ||_direction_||^2 * (||_center_ - _origin_||^2 - radius^2) >= 0
        //                  (                b                      )^2 - 4 *         a         * (                 c                  )
        //
        // 4. therefore the ray does intersect the sphere if and only if discriminant >= 0
        //
        // 5. intersection coordinates are calculated by projecting the second equation on each axis:
        // _r_x = _origin_x + _direction_x * t
        // _r_y = _origin_y + _direction_y * t
        // _r_z = _origin_z + _direction_z * t

        let origin = ray.origin();
        let direction = ray.direction();

        let b = -2.0 * (self.center - origin).dot(&direction);
        let a = direction.sq_norm();
        let c = (self.center - origin).sq_norm() - self.radius.powi(2);

        let discriminant = (b.powi(2) - 4.0 * a * c).sqrt();

        if discriminant.is_nan() {
            return None;
        }

        let t_1 = (-b - discriminant) / (2.0 * a);

        let intersection_coords: Option<Vector> = if t_1 > 0.0001 {
            Some(origin + direction * t_1)
        } else {
            None
        };

        intersection_coords
    }

    fn get_normal_vector(&self, coordinates: &Vector) -> Vector {
        (coordinates - &self.center).normalize()
    }
}

impl ObjectAppearance for Sphere {
    fn get_color(&self, ray: &Ray) -> super::Color {
        self.color
    }

    fn get_diffusion(&self) -> f64 {
        self.diffusion
    }
}
