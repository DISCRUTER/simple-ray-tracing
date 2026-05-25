use crate::hittable::{Hit_Record, Hittable};
use crate::point::Point;
use crate::rtweekend::{color::Color, interval::Interval, ray::Ray};

#[derive(Debug, Default)]
pub struct Camera {
    // Public fields
    pub aspect_ratio: f32,
    pub image_width: i32,
    // Private fields
    image_height: i32,
    camera_center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Point,
    pixel_delta_v: Point,
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        // Initalizing
        self.initialize();

        // Render
        println!("P3\n{} {} \n255\n", self.image_width, self.image_height);

        for i in 0..self.image_height {
            for j in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * j as f32)
                    + (self.pixel_delta_v * i as f32);
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);

                let pixel_color = ray_color(&ray, world);
                println!("{}\n", pixel_color);
            }
        }
    }

    fn initialize(&mut self) {
        // Setting Image height
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = self.image_height.clamp(1, self.image_height);
        

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
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    let mut rec = Hit_Record::default();
    if world.hit(ray, Interval::new_from(0.001, f32::INFINITY), &mut rec) {
        let normal = rec.get_normal();
        return Color::new_from(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    return Color::new_from(1.0, 1.0, 1.0) * (1.0 - a) + Color::new_from(0.5, 0.7, 1.0) * a;
}
