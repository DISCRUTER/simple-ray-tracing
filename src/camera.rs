use crate::hittable::{HitRecord, Hittable};
use crate::point::Point;
use crate::rtweekend::random_f32;
use crate::rtweekend::{color::Color, interval::Interval, ray::Ray};

#[derive(Debug, Default)]
pub struct Camera {
    // Public fields
    pub aspect_ratio: f32,          // Ratio of image width over height
    pub image_width: i32,           // Rendered image width in pixel count
    pub sample_per_pixel: i32,      // Count of random samples for each pixel
    pub max_depth: i32,             // Maximun number of ray bounces
    // Private fields
    image_height: i32,              // Rendered image width in pixel count
    camera_center: Point,           // Camera center
    pixel00_loc: Point,             // Location of the first pixel at left-top
    pixel_delta_u: Point,           // Distance between two horizontal pixel
    pixel_delta_v: Point,           // Distance between two vertical pixel
    pixel_samples_scale: f32,       // Averaging value for samped colors | Inversely proportional to `sample_per_pixel`
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        // Initalizing
        self.initialize();

        // Render
        println!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.sample_per_pixel {
                    let r: Ray = self.get_ray(j, i);
                    pixel_color += ray_color(&r, self.max_depth, world)
                }
                println!("{}\n", pixel_color * self.pixel_samples_scale);
            }
        }
    }

    fn initialize(&mut self) {
        // Setting Image height
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.clamp(1, self.image_height);
        
        // Setting pixel_samples_scale
        self.pixel_samples_scale = 1.0 / self.sample_per_pixel as f32;
        
        // Assigning camera center
        self.camera_center = Point::new();

        // Determine Viewport dimension
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 =
            viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate vector across viewport
        let viewport_u = Point::new_from(viewport_width, 0.0, 0.0);
        let viewport_v = Point::new_from(0.0, -viewport_height, 0.0);

        // Calculating horizontal & vertical delta
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculating upper-left pixel
        let viewport_upper_left = self.camera_center
            - Point::new_from(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Random sampling pixels around pixel i, j
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f32 + offset.x()))
            + (self.pixel_delta_v * (j as f32 + offset.y()));

        let ray_origin = self.camera_center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
}

// Return final color after hit
fn ray_color(ray: &Ray, depth: i32, world: &impl Hittable) -> Color {
    // If max_depth exceeded return
    if depth <= 0 {
        return Color::new();
    }
    let mut rec = HitRecord::default();
    if world.hit(ray, Interval::new_from(0.001, f32::INFINITY), &mut rec) {
        let direction  = rec.get_normal() + Point::random_unit_vector();
        return ray_color(&Ray::new(rec.get_p(), direction), depth-1, world) * 0.5
    }

    let unit_direction = ray.direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    return Color::new_from(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_from(0.5, 0.7, 1.0) * a;
}

fn sample_square() -> Point {
    // Return the vector to a random point in the [-.5, -.5]-[+.5, +.5] unit square
    Point::new_from(random_f32() - 0.5, random_f32() - 0.5, 0.0)
}
