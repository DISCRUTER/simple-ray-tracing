use std::marker::PhantomData;
use std::ops::{AddAssign, DivAssign, MulAssign};
use crate::rtweekend::{random_f32, random_range_f32};

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
    // Random vector generator
    pub fn random() -> Self {
        Self {
            x: random_f32(),
            y: random_f32(),
            z: random_f32(),
            _kind: PhantomData,
        }
    }
    pub fn random_range(min: f32, max: f32) -> Self {
        Self {
            x: random_range_f32(min, max),
            y: random_range_f32(min, max),
            z: random_range_f32(min, max),
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
