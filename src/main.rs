use image::{ImageBuffer, Pixel};
use std::path::Path;
use std::ops::{DerefMut};
use core::mem;

fn draw_line<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>(mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32, img: &mut ImageBuffer<P, Container>, pixel: P) {
    // transpose if the line is steep
    let mut steep = false;
    if (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs() {
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    // left to right
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    for x in x0..x1 {
        let dx = (x - x0) as f32 / (x1 - x0) as f32;
        let y = y0 as f32 * 1. - dx + y1 as f32 * dx;
        match steep {
            true => img.put_pixel(y as u32, x, pixel),
            false => img.put_pixel(x, y as u32, pixel)
        }
    }
}

fn main() {
    let white = image::Rgb([255, 255, 255]);
    let red = image::Rgb([255, 0, 0]);
    let mut image = image::ImageBuffer::new(100, 100);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([0, 0, 0]))
        }
    }
    draw_line(13, 20, 80, 40, &mut image, white); // gets over-drawn by the next red line
    draw_line(20, 13, 40, 80, &mut image, red);
    draw_line(80, 40, 13, 20, &mut image, red);
    image.save(&Path::new("image.png")).unwrap();
}
