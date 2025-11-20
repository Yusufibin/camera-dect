use image::{RgbImage, ImageBuffer};
use std::fs;

pub fn create_dummy_image(path: &str, offset: u32) {
    let width = 640;
    let height = 480;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = ((x + offset) % 255) as u8;
        let g = ((y + offset) % 255) as u8;
        let b = ((x + y) % 255) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    // Draw a moving square
    let square_size = 50;
    let square_x = (offset * 10) % (width - square_size);
    let square_y = (offset * 10) % (height - square_size);

    for x in square_x..(square_x + square_size) {
        for y in square_y..(square_y + square_size) {
            img.put_pixel(x, y, image::Rgb([255, 255, 255]));
        }
    }

    img.save(path).expect("Failed to save dummy image");
}

fn main() {
    // Create single sample
    create_dummy_image("assets/sample_frame.jpg", 0);

    // Create sequence
    let seq_dir = "assets/video_seq";
    fs::create_dir_all(seq_dir).expect("Failed to create seq dir");

    for i in 0..20 {
        let filename = format!("{}/frame_{:03}.jpg", seq_dir, i);
        create_dummy_image(&filename, i);
        println!("Generated {}", filename);
    }
}
