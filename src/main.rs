use image::{ImageBuffer, Pixel};
use std::path::Path;
use std::ops::{DerefMut};

fn draw_line<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut ImageBuffer<P, Container>, pixel: P) {
    for x in x0..x1 {
        let dx = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * dx + (y1 - y0) as f32 * dx;
        img.put_pixel(x, y as u32, pixel);
    }
}

fn main() {
    let white = image::Rgb([255, 255, 255]);
    let red = image::Rgb([255, 0, 0]);
    let mut image = image::ImageBuffer::new(100, 100);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([120, 0, 240]))
        }
    }
    draw_line(13, 20, 80, 40, &mut image, white);
    draw_line(20, 13, 40, 80, &mut image, red);
    draw_line(80, 40, 13, 20, &mut image, red);
    image.save(&Path::new("image.png")).unwrap();
}
