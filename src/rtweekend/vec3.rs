use std::marker::PhantomData;
use std::ops::{AddAssign, DivAssign, MulAssign};

// Empty markers
#[derive(Default, Debug, Copy, Clone)]
pub struct ColorKind;
#[derive(Default, Debug, Copy, Clone)]
pub struct PointKind;

// Base Vector struct
#[derive(Default, Debug, Copy, Clone)]
pub struct Vec3<T> {
    x: f32,
    y: f32,
    z: f32,
    _kind: PhantomData<T>,
}

// Implementation of common vector operations

impl<T> Vec3<T> {
    // Constructors
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            _kind: PhantomData,
        }
    }
    pub fn new_from(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            _kind: PhantomData,
        }
    }
    // Accessor
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
}

// Operator overloading
impl<T> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl<T> MulAssign<f32> for Vec3<T> {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl<T> DivAssign<f32> for Vec3<T> {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Point utility
// impl Add for Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn add(self, rhs: Self) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x + rhs.x,
//             self.y + rhs.y,
//             self.z + rhs.z
//         )
//     }
// }
// impl Sub for Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn sub(self, rhs: Self) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x - rhs.x,
//             self.y - rhs.y,
//             self.z - rhs.z
//         )
//     }
// }
// impl Mul<Vec3<PointKind>> for Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn mul(self, rhs: Vec3<PointKind>) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x * rhs.x,
//             self.y * rhs.y,
//             self.z * rhs.z
//         )
//     }
// }
// impl Mul<Vec3<PointKind>> for &Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn mul(self, rhs: Vec3<PointKind>) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x * rhs.x,
//             self.y * rhs.y,
//             self.z * rhs.z
//         )
//     }
// }
// impl Mul<f32> for Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x * rhs,
//             self.y * rhs,
//             self.z * rhs
//         )
//     }
// }
// impl Mul<f32> for &Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec3::<PointKind>::new_from(
//             self.x * rhs,
//             self.y * rhs,
//             self.z * rhs
//         )
//     }
// }
// impl Div<f32> for Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn div(self, rhs: f32) -> Self::Output {
//         self * (1.0 / rhs)
//     }
// }
// impl Div<f32> for &Vec3<PointKind> {
//     type Output = Vec3<PointKind>;
//     fn div(self, rhs: f32) -> Self::Output {
//         self * (1.0 / rhs)
//     }
// }

// impl Vec3<PointKind> {
//     pub fn length_squared(&self) -> f32 {
//         self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
//     }
//     pub fn length(&self) -> f32 {
//         self.length_squared().sqrt()
//     }
//     pub fn dot(&self, other: &Vec3<PointKind>) -> f32 {
//         self.x * other.x + self.y * other.y + self.z + other.z
//     }
//     pub fn cross(&self, other: &Vec3<PointKind>) -> Vec3<PointKind> {
//         Vec3::<PointKind>::new_from(
//             self.y * other.z - self.z * other.y,
//             self.z * other.x - self.x * other.z,
//             self.x * other.y - self.y * other.x
//         )
//     }
//     pub fn unit_vector(&self) -> Vec3<PointKind> {
//         self / self.length()
//     }
// }

// Color Utility
