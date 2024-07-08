use rand::Rng;
use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Squared vector norm
    pub fn sq_norm(&self) -> f64 {
        self.dot(&self)
    }

    pub fn normalize(&self) -> Self {
        self / self.sq_norm().sqrt()
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Vector::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    /// Generates a random unit length vector
    /// inside hemisphere defined by the given main normal
    /// (default main normal is (0, 0, 1))
    pub fn gen_rand_inside_hemisphere(n: Option<Vector>) -> Self {
        let mut rng = rand::thread_rng();

        let phi = rng.gen_range(-PI / 2.0..PI / 2.0);
        let theta = rng.gen_range(0.0..2.0 * PI);

        let mut random_vector =
            Vector::new(phi.sin() * theta.cos(), phi.sin() * theta.sin(), phi.cos());

        if let Some(normal_vector) = n {
            random_vector = random_vector + normal_vector - Vector::new(0.0, 0.0, 1.0);
        }

        random_vector
    }
}

impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Self::Output {
        Vector::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Add<f64> for &Vector {
    type Output = Vector;

    fn add(self, rhs: f64) -> Self::Output {
        Vector::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<f64> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: f64) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> Add<&'a Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * (-1.0))
    }
}

impl<'a> Sub<&'a Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self + &(rhs * (-1.0))
    }
}

/// Ray that is defined with origin point and unit length direction vector:
/// `_r_(t) = _origin_ + t * _direction_`
///
/// Stores `direction` field as unit length vector!
#[derive(Debug)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

impl Ray {
    /// Creates ray with the given origin and direction
    ///
    /// Normalizes the direction vector!
    pub fn new(origin: Vector, direction: Vector) -> Self
    where
        Self: Sized,
    {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Creates ray with origin at (0, 0, 0) and the given direction
    ///
    /// Normalizes the direction vector!
    pub fn with_default_origin(direction: Vector) -> Self
    where
        Self: Sized,
    {
        Self {
            origin: Vector::default(),
            direction: direction.normalize(),
        }
    }

    pub fn origin(&self) -> Vector {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }

    /// Converts ray into vector for the given `t`:
    /// `_output_vector_ = _r_(t) = _origin_ + t * _direction_`
    pub fn to_vector(&self, t: f64) -> Vector {
        self.origin() + self.direction() * t
    }
}

impl From<Ray> for Vector {
    /// Converts ray into vector with `t = 1`:
    /// `_output_vector_ = _r_(1) = _origin_ + 1 * _direction_ = _origin_ + _direction_`
    fn from(ray: Ray) -> Self {
        ray.to_vector(1.0)
    }
}
