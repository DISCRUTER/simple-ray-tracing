// Constants
const PI: f32 = 3.1415926535897932385;

// Utility Functions
#[inline]
pub fn degree_to_radians(degree: f32) -> f32 {
    return degree * PI / 180.0;
}

// Commmon Modules
pub mod color;
pub mod ray;
pub mod vec3;
