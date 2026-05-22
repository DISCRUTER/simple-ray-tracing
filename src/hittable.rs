use crate::{
    point::Point,
    rtweekend::{interval::Interval, ray::Ray},
};

#[derive(Default, Debug, Copy, Clone)]
pub struct Hit_Record {
    p: Point,
    normal: Point,
    t: f32,
    front_face: bool,
}

impl Hit_Record {
    pub fn set(&mut self, other: &Self) {
        self.set_t(other.get_t());
        self.set_p(other.get_p());
        self.set_normal(other.get_normal())
    }
    pub fn set_t(&mut self, t: f32) {
        self.t = t;
    }
    pub fn get_t(&self) -> f32 {
        self.t
    }
    pub fn set_normal(&mut self, normal: Point) {
        self.normal = normal;
    }
    pub fn get_normal(&self) -> Point {
        self.normal
    }
    pub fn set_p(&mut self, p: Point) {
        self.p = p;
    }
    pub fn get_p(&self) -> Point {
        self.p
    }
    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: Point) {
        // Check if the ray is coming from inside or outside
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut Hit_Record) -> bool;
}
