use std::ops::{Add, Mul};

use crate::rtweekend::vec3::{ColorKind, Vec3};

// Alias
pub type Color = Vec3<ColorKind>;

// Display Trait
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.x();
        let g = self.y();
        let b = self.z();
        // Converting [0, 1] to [0, 255]
        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        write!(f, "{}, {}, {}", ir, ig, ib)
    }
}

// Operation overloading
impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::new_from(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::new_from(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}
