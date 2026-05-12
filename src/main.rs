mod vec3;

use log::info;

use crate::vec3::{ColorKind, PointKind, Vec3};


// Setting alias for kinds
type Color = Vec3<ColorKind>;
type Point = Vec3<PointKind>;

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
            let r = j as f64 / (image_width - 1) as f64;
            let g = i as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
    info!("\rDone.             \n");
}
