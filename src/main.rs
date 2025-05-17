use std::path::Path;

use image::{Rgb, Rgba};

fn main() {
    let filename = Path::new("/Users/godsiom/coding/rust/moving_spheres/saved/image_6.png");

    let width: u32 = 1600;
    let height: u32 = 900;

    let mut buffer: image::ImageBuffer<Rgb<u8>, Vec<_>> = image::ImageBuffer::new(width, height);

    for p in buffer.pixels_mut() {
        *p = Rgb([180, 0, 0]);
    }

    match buffer.save(filename) {
        Ok(_) => println!("success"),
        Err(e) => println!("failed -> {:?}", e),
    }
}
