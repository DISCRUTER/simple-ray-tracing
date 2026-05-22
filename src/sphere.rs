use crate::{
    hittable::{Hit_Record, Hittable},
    point::Point,
    rtweekend::{interval::Interval, ray::Ray},
};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point::default(),
            radius: 0.0,
        }
    }
    pub fn new_from(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut Hit_Record) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        // Adding sphere to hitrecord
        rec.set_t(root);
        rec.set_p(ray.at(rec.get_t()));
        let outward_normal = (rec.get_p() - self.center) / self.radius;
        rec.set_front_face(ray, outward_normal);
        return true;
    }
}
