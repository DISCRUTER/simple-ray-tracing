use std::ops::{Add, Div, Mul, Sub};

use crate::vec3::{PointKind, Vec3};

// Alias

pub type Point = Vec3<PointKind>;

// Display Trait

impl std::fmt::Display for Vec3<PointKind> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

// Utility

impl Point {
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[inline]
    pub fn dot(&self, other: &Point) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    #[inline]
    pub fn cross(&self, other: &Point) -> Point {
        Point::new_from(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }
    #[inline]
    pub fn unit_vector(&self) -> Point {
        self * (1.0 / self.length())
    }
    #[inline]
    pub fn random_unit_vector() -> Point {
        loop {
            let p: Point = Point::random_range(-1.0, 1.0);
            let lensq: f32 = p.length_squared();
            if lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }
    #[inline]
    pub fn random_on_hemisphere(normal: Point) -> Point {
        let on_unit_sphere: Point = Point::random_unit_vector();
        if on_unit_sphere.dot(&normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return on_unit_sphere * -1.0;
        }
    }
}

// Operators

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new_from(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new_from(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}
impl Mul<Point> for Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new_from(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
impl Mul<Point> for &Point {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        Point::new_from(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
impl Mul<f32> for Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Self::Output {
        Point::new_from(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}
impl Mul<f32> for &Point {
    type Output = Point;
    fn mul(self, rhs: f32) -> Self::Output {
        Point::new_from(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}
impl Div<f32> for Point {
    type Output = Point;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl Div<f32> for &Point {
    type Output = Point;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
