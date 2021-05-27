use image::{ImageBuffer, Pixel};
use std::path::Path;
use std::ops::{DerefMut};

fn draw_line<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>(x0: u32, y0: u32, x1: u32, y1: u32, img: &mut ImageBuffer<P, Container>, pixel: P) {
    for ti in 0..100 {
        let t = ti as f32 / 100.;
        let x = x0 as f32 + (x1 - x0) as f32 * t;
        let y = y0 as f32 + (y1 - y0) as f32 * t;
        img.put_pixel(x as u32, y as u32, pixel)
    }
}

fn main() {
    let white = image::Rgb([255, 255, 255]);
    let mut image = image::ImageBuffer::new(100, 100);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([120, 0, 240]))
        }
    }
    draw_line(10, 10, 20, 20, &mut image, white);
    image.save(&Path::new("image.png")).unwrap();
}
