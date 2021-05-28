use image::{ImageBuffer, Pixel};
use image::imageops::{flip_vertical_in_place};
use std::path::Path;
use std::ops::{DerefMut};
use core::mem;
use nalgebra::{Vector2};
use tobj;


// TODO instead of u32 use generic
fn draw_triangle<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>
(mut p0: Vector2<u32>, mut p1: Vector2<u32>, mut p2: Vector2<u32>, img: &mut ImageBuffer<P, Container>, pixel: P) {
    // sort by y coordinate
    if p0.y > p1.y { mem::swap(&mut p0, &mut p1); }
    if p0.y > p2.y { mem::swap(&mut p0, &mut p2); }
    if p1.y > p2.y { mem::swap(&mut p1, &mut p2); }
    draw_line(p0.x, p0.y, p1.x, p1.y, img, pixel);
    draw_line(p0.x, p0.y, p2.x, p2.y, img, pixel);
    draw_line(p1.x, p1.y, p2.x, p2.y, img, pixel);
}

// TODO: instead of u32 use generic
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

fn render(source: &str, out: &str, width: u32, height: u32) {
    let (models, _) = tobj::load_obj(&Path::new(source), &tobj::LoadOptions::default()).unwrap();

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
    image.save(&Path::new(out)).unwrap();
}

fn main() {
    // render("models/african_head.obj", "african_head.png", 600, 600);
    let width = 200;
    let height = 200;
    let mut image: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_pixel(width, height, image::Rgb([0, 0, 0]));

    let t0 = vec![Vector2::new(10, 70), Vector2::new(50, 160), Vector2::new(70, 80)];
    let t1 = vec![Vector2::new(180, 50), Vector2::new(150, 1), Vector2::new(70, 180)];
    let t2 = vec![Vector2::new(180, 150), Vector2::new(120, 160), Vector2::new(130, 180)];
    draw_triangle(t0[0], t0[1], t0[2], &mut image, image::Rgb([255, 0, 0]));
    draw_triangle(t1[0], t1[1], t1[2], &mut image, image::Rgb([255, 255, 255]));
    draw_triangle(t2[0], t2[1], t2[2], &mut image, image::Rgb([0, 255, 0]));

    flip_vertical_in_place(&mut image);
    image.save(&Path::new("triangles.png")).unwrap();
}
