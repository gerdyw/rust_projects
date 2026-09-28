use std::fmt::Display;

use crate::vec3::Vec3;

pub type Color = Vec3;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = to_rgb(self.x());
        let g = to_rgb(self.y());
        let b = to_rgb(self.z());
        write!(f, "{r} {g} {b}")
    }
}

fn to_rgb(x: f64) -> u8 {
    (x * 255.999).floor() as u8
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    assert!(r <= 1f64 && g <= 1f64 && b <= 1f64);
    Color::new(r, g, b)
}
