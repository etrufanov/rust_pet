use std::ops::{Add, Div, Mul, Sub};

#[derive(Default, Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

pub type Ray = Vector;

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

    pub fn sq_norm(&self) -> f64 {
        self.dot(&self)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.x
    }

    pub fn z(&self) -> f64 {
        self.x
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
