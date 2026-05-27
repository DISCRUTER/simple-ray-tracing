use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::{interval::Interval, ray::Ray};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.get_max();

        for object in self.objects.iter() {
            if object.hit(
                ray,
                Interval::new_from(ray_t.get_min(), closest_so_far),
                rec,
            ) {
                hit_anything = true;
                closest_so_far = rec.get_t();
            }
        }

        hit_anything
    }
}
