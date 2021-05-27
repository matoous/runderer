use image::{ImageBuffer, Pixel};
use image::imageops::{flip_vertical_in_place};
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
    let dx = (x1 - x0) as f32;
    let dy = (y1 - y0) as f32;
    // how much further from the real line will we get in each iteration
    let derror = (dy / dx).abs();
    let mut error = 0.;
    let mut y = y0;
    for x in x0..x1 {
        match steep {
            true => img.put_pixel(y as u32, x, pixel),
            false => img.put_pixel(x, y as u32, pixel)
        }
        error += derror;
        if error > 0.5 {
            if y1 > y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error -= 1.;
        }
    }
}

fn main() {
    let white = image::Rgb([255, 255, 255]);
    let red = image::Rgb([255 , 0, 0]);
    let mut image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(100, 100);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([0, 0, 0]))
        }
    }
    draw_line(13, 20, 80, 40, &mut image, white); // gets over-drawn by the next red line
    draw_line(20, 13, 40, 80, &mut image, red);
    draw_line(80, 40, 13, 20, &mut image, red);
    flip_vertical_in_place(&mut image);
    image.save(&Path::new("image.png")).unwrap();
}
