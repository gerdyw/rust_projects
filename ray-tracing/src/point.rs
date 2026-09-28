use crate::vec3::Vec3;

pub type Point3 = Vec3;

pub fn point(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}
