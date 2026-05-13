use crate::vec3::{Vec3, ColorKind};

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