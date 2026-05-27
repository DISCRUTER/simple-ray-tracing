// Constants
const PI: f32 = 3.1415926535897932385;

// Utility Functions
#[inline]
pub fn degree_to_radians(degree: f32) -> f32 {
    return degree * PI / 180.0;
}

#[inline]
pub fn random_f32() -> f32 {
    fastrand::f32()
}

#[inline]
pub fn random_range_f32(min: f32, max: f32) -> f32 {
    min + (max-min) * random_f32()
}

// Commmon Modules
pub mod color;
pub mod interval;
pub mod ray;
pub mod vec3;
