mod camera;
mod hittable;
mod hittable_list;
mod point;
mod rtweekend;
mod sphere;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::point::Point;
use crate::rtweekend::vec3;
use crate::sphere::Sphere;

fn main() {
    // World
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new_from(
        Point::new_from(0.0, -100.5, -1.0),
        100.0,
    )));
    world.add(Box::new(Sphere::new_from(
        Point::new_from(0.0, 0.0, -1.0),
        0.5,
    )));

    // Render
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.sample_per_pixel = 100;
    cam.render(&world);
}
