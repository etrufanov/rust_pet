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
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Color) -> Self
    where
        Self: Sized,
    {
        Self {
            center,
            radius,
            color,
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
        }
    }
}

impl RayToObjectHandler for Sphere {
    fn calc_ray_intersection(&self, pixel_vec: &Ray) -> Option<Vec<Vector>> {
        // 1. system of equations (underscores stands for vector):
        // --
        // |  radius^2 = ||_center_ - _r_||^2
        // |  _r_ = _pixel_vec_ * t
        // --
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
        //
        // 5. intersection coordinates are calculated by projecting the second equation on each axis:
        // _r_x = _pixel_vec_x * t
        // _r_y = _pixel_vec_y * t
        // _r_z = _pixel_vec_z * t

        let b = 2.0 * self.center.dot(pixel_vec);
        let a = pixel_vec.sq_norm();
        let c = self.center.sq_norm() - self.radius.powi(2);

        let discriminant = (b.powi(2) - 4.0 * a * c).sqrt();

        if discriminant.is_nan() {
            return None;
        }

        let t_1 = (-b - discriminant) / (2.0 * a);
        let t_2 = (-b + discriminant) / (2.0 * a);

        let mut intersection_coords_list = vec![pixel_vec * t_1, pixel_vec * t_2];

        sort_vectors_by_z(intersection_coords_list.as_mut());

        Some(intersection_coords_list)
    }
}

impl ObjectAppearance for Sphere {
    fn get_color(&self, pixel_vec: &Ray) -> super::Color {
        self.color
    }
}
