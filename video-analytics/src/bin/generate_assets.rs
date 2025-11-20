use image::{RgbImage, ImageBuffer};

pub fn create_dummy_image(path: &str) {
    let width = 640;
    let height = 480;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (x % 255) as u8;
        let g = (y % 255) as u8;
        let b = ((x + y) % 255) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    img.save(path).expect("Failed to save dummy image");
}

fn main() {
    create_dummy_image("assets/sample_frame.jpg");
}
