use std::ops::{Add, Mul};

use crate::rtweekend::{
    interval::{Interval},
    vec3::{ColorKind, Vec3}
};

// Alias
pub type Color = Vec3<ColorKind>;

// Display Trait
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();
        // Converting from linear to gamma
        r = linear_to_gamma(r);
        g = linear_to_gamma(g);
        b = linear_to_gamma(b);
        // Converting [0, 1] to [0, 255]
        let intensity = Interval::new_from(0.0, 0.999);
        let ir = (256.0 * intensity.clamp(r)) as u8;
        let ig = (256.0 * intensity.clamp(g)) as u8;
        let ib = (256.0 * intensity.clamp(b)) as u8;

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


// Helper functions
fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    return 0.0;
}