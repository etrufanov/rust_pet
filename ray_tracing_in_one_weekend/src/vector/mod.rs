use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy, Debug)]
pub struct Vector {
	x: f64,
	y: f64,
	z: f64,
}

type Ray = Vector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self
	where
		Self: Sized,
	{
		Self { x, y, z }
	}

	pub fn dot(self, rhs: Self) -> f64 {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}

	pub fn sq_norm(self) -> f64 {
		self.dot(self)
	}
}

impl Add<f64> for Vector {
	type Output = Self;

	fn add(self, rhs: f64) -> Self {
		Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub<f64> for Vector {
	type Output = Self;

	fn sub(self, rhs: f64) -> Self {
		self + (-rhs)
    }
}

impl Mul<f64> for Vector {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self {
		Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
	type Output = Self;

	fn div(self, rhs: f64) -> Self {
		self * (1.0 / rhs)
    }
}


impl Add for Vector {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		self + rhs * (-1.0)
    }
}
