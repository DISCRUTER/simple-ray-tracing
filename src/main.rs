mod hittable;
mod hittable_list;
mod point;
mod rtweekend;
mod sphere;

use log::info;

use crate::hittable::{Hit_Record, Hittable};
use crate::hittable_list::HittableList;
use crate::point::Point;
use crate::rtweekend::{color::Color, interval::Interval, ray::Ray, vec3};
use crate::sphere::Sphere;

// Function to output ray color
fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut rec = Hit_Record::default();
    if world.hit(ray, Interval::new_from(0.0, f32::INFINITY), &mut rec) {
        let normal = rec.get_normal();
        return Color::new_from(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    return Color::new_from(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_from(0.5, 0.7, 1.0) * a;
}

fn main() {
    // Intializing Logger
    env_logger::init();

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    image_height = image_height.clamp(1, image_height);

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

    // Camera
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point::new();

    // Calculate vector across viewport
    let viewport_u = Point::new_from(viewport_width, 0.0, 0.0);
    let viewport_v = Point::new_from(0.0, -viewport_height, 0.0);

    // Calculating horizontal & vertical delta
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculating upper-left pixel
    let viewport_upper_left = camera_center
        - Point::new_from(0.0, 0.0, focal_length)
        - (viewport_u / 2.0)
        - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    // Render
    println!("P3\n{} {} \n255\n", image_width, image_height);

    for i in 0..image_height {
        info!("\rScanlines remaining: {} ", image_height - i);
        for j in 0..image_width {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * j as f32) + (pixel_delta_v * i as f32);
            let ray_direction = pixel_center - camera_center;
            info!("Ray direction: {}", ray_direction);
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            println!("{}\n", pixel_color);
        }
    }
    info!("\rDone.             \n");
}
