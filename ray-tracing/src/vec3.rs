use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    arr: [f64; 3],
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.arr[0]
    }

    pub fn y(&self) -> f64 {
        self.arr[1]
    }

    pub fn z(&self) -> f64 {
        self.arr[2]
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { arr: [x, y, z] }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x()
        + self.y() * self.y()
        + self.z() * self.z()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x() * rhs.x()
        + self.y() * rhs.y()
        + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        vec3(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() *rhs.x()
        )
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            arr: Default::default(),
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        vec3(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1f64
    }
}

pub fn vec3(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}
