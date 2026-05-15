mod color;
mod point;
mod ray;
mod vec3;

use log::info;

use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;


fn hit_sphere(center: Point, radius: f32, ray: &Ray) -> f32 {
    let oc = center - ray.origin();
    let a = ray.direction().length_squared();
    let h = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - 4.0*a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (h - discriminant.sqrt()) / a;
    }
}

// Function to output ray color
fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point::new_from(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let n: vec3::Vec3<vec3::PointKind> = (ray.at(t) - Point::new_from(0.0, 0.0, -1.0)).unit_vector();
        return Color::new_from(n.x()+1.0, n.y()+1.0, n.z()+1.0) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    info!("Value of a: {}", a); // error in alpha
    Color::new_from(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_from(0.5, 0.7, 1.0) * a
}



fn main() {
    // Intializing Logger
    env_logger::init();

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let mut image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    image_height = image_height.clamp(1, image_height);

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
    let viewport_upper_left = camera_center - Point::new_from(0.0, 0.0, focal_length) - (viewport_u / 2.0)  - (viewport_v / 2.0);
    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);
    

    // Render
    println!("P3\n{} {} \n255\n", image_width, image_height);

    for i in 0..image_height {
        info!("\rScanlines remaining: {} ", image_height - i);
        for j in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * j as f32) + (pixel_delta_v * i as f32);
            let ray_direction = pixel_center - camera_center;
            info!("Ray direction: {}", ray_direction);
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray);
            println!("{}\n", pixel_color);
        }
    }
    info!("\rDone.             \n");
}
