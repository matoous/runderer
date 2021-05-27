use image::{ImageBuffer, Pixel};
use image::imageops::{flip_vertical_in_place};
use std::path::Path;
use std::ops::{DerefMut};
use core::mem;
use tobj;

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
    let dx = x1 as i32 - x0 as i32;
    let dy = y1 as i32 - y0 as i32;
    // how much further from the real line will we get in each iteration
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;
    for x in x0..x1 {
        match steep {
            true => img.put_pixel(y as u32, x, pixel),
            false => img.put_pixel(x, y as u32, pixel)
        }
        error2 += derror2;
        if error2 > 1 {
            if y1 > y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error2 -= dx * 2;
        }
    }
}

fn main() {
    let width = 600;
    let height = 600;
    let (models, _) =
        tobj::load_obj(
            &Path::new("models/african_head.obj"),
            &tobj::LoadOptions::default(),
        ).expect("Failed to OBJ load file");

    let model = models.get(0).unwrap();
    let mesh = &model.mesh;

    let mut image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, image::Rgb([0, 0, 0]))
        }
    }

    let white = image::Rgb([255, 255, 255]);
    for face in (0..mesh.indices.len()).step_by(3) {
        let vertices = &mesh.indices[face..face + 3];
        for i in 0..3 {
            let v0 = &mesh.positions[(vertices[i] * 3) as usize..(vertices[i] * 3 + 3) as usize];
            let v1 = &mesh.positions[(vertices[(i + 1) % 3] * 3) as usize..(vertices[(i + 1) % 3] * 3 + 3) as usize];
            let x0 = (v0[0] + 1.) * width as f32 / 2. - 1.;
            let y0 = (v0[1] + 1.) * height as f32 / 2. - 1.;
            let x1 = (v1[0] + 1.) * width as f32 / 2. - 1.;
            let y1 = (v1[1] + 1.) * height as f32 / 2. - 1.;
            draw_line(x0 as u32, y0 as u32, x1 as u32, y1 as u32, &mut image, white);
        }
    }
    flip_vertical_in_place(&mut image);
    image.save(&Path::new("african_head.png")).unwrap();
}
