mod vec3;
mod color;
mod point;
mod ray;

use log::info;

use crate::color::Color;
// use crate::point::Point;
// use crate::ray::Ray;


fn main() {
    // Intializing Logger

    env_logger::init();

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{} {} \n255\n", image_width, image_height);

    for i in 0..image_height {
        info!("\rScanlines remaining: {} ", image_height-i);
        for j in 0..image_width {

            let pixel_color = Color::new_from(
                j as f32 / (image_width - 1) as f32,
                j as f32 / (image_width - 1) as f32,
                0.0);

            println!("{}\n", pixel_color);
        }
    }
    info!("\rDone.             \n");
}
