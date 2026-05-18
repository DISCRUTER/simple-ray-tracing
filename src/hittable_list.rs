use crate::hittable::{Hit_Record, Hittable};
use crate::rtweekend::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // pub fn hittable_list(object: Box<dyn Hittable>) {
    //     self.add(object);
    // }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut Hit_Record) -> bool {
        let mut temp_rec = Hit_Record::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(ray, ray_tmin, ray_tmax, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.get_t();
                rec.set(&temp_rec);
            }
        }

        return hit_anything;
    }
}
