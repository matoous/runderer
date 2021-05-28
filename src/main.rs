use image::{ImageBuffer, Pixel, Primitive};
use image::imageops::{flip_vertical_in_place};
use std::path::Path;
use std::ops::{DerefMut};
use core::mem;
use nalgebra::{Vector2, Vector3};
use tobj;
use std::cmp::{min, max};
use rand::Rng;

#[derive(Debug)]
pub struct Triangle(pub Vector2<i32>, pub Vector2<i32>, pub Vector2<i32>);

#[derive(Debug)]
pub struct Line(pub Vector2<i32>, pub Vector2<i32>);

#[derive(Debug)]
pub struct Box(pub Vector2<i32>, pub Vector2<i32>);

impl IntoIterator for Box {
    type Item = Vector2<i32>;
    type IntoIter = BoxIterator;

    fn into_iter(self) -> Self::IntoIter {
        BoxIterator {
            current: self.0,
            bbox: self,
        }
    }
}

pub struct BoxIterator {
    bbox: Box,
    current: Vector2<i32>,
}

impl Iterator for BoxIterator {
    type Item = Vector2<i32>;

    fn next(&mut self) -> Option<Vector2<i32>> {
        let x = self.current;
        self.current.x += 1;
        if self.current.x > self.bbox.1.x {
            self.current.x = self.bbox.0.x;
            self.current.y += 1;
        };
        if self.current.y > self.bbox.1.y {
            return None;
        }
        Some(x)
    }
}

impl Triangle {
    fn bounding_box(&self) -> Box where {
        let mut bboxmin = Vector2::new(i32::MAX, i32::MAX);
        let mut bboxmax = Vector2::new(i32::MIN, i32::MIN);
        for p in &[self.0, self.1, self.2] {
            bboxmin.x = min(bboxmin.x, p.x);
            bboxmax.x = max(bboxmax.x, p.x);
            bboxmin.y = min(bboxmin.y, p.y);
            bboxmax.y = max(bboxmax.y, p.y);
        };
        Box(bboxmin, bboxmax)
    }

    // triangle contains the point if all of the barycentric coordinates are positive or zero
    fn contains(&self, point: Vector2<i32>) -> bool {
        let coors = self.barycentric_coordinates(point);
        !(coors.x < 0. || coors.y < 0. || coors.z < 0.)
    }

    // compute barycentric coordinates of point p with respect to the triangle
    // https://en.wikipedia.org/wiki/Barycentric_coordinate_system
    fn barycentric_coordinates(&self, p: Vector2<i32>) -> Vector3<f32> {
        // vector AB_x, AC_x, and PA_x
        let x = Vector3::new(self.2.x - self.0.x, self.1.x - self.0.x, self.0.x - p.x);
        // vector AB_y, AC_y, and PA_y
        let y = Vector3::new(self.2.y - self.0.y, self.1.y - self.0.y, self.0.y - p.y);
        // find cross-product, i.e. orthogonal vector to both of the previous vectors
        let u = x.cross(&y);
        if u.z.abs() < 1 { return Vector3::new(-1., 1., 1.); }
        // and finally calculate the barycentric coordinates
        Vector3::new(1. - (u.x + u.y) as f32 / u.z as f32, u.y as f32 / u.z as f32, u.x as f32 / u.z as f32)
    }
}

// TODO instead of u32 use generic
fn draw_triangle<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>(mut t: Triangle, img: &mut ImageBuffer<P, Container>, pixel: P) {
    let mut bbox = t.bounding_box();
    if bbox.0.x < 0 { bbox.0.x = 0 };
    if bbox.1.x > img.width() as i32 - 1 { bbox.1.x = img.width() as i32 - 1 };
    if bbox.0.y < 0 { bbox.0.y = 0 };
    if bbox.1.y > img.height() as i32 - 1 { bbox.1.x = img.height() as i32 - 1 };
    for p in bbox.into_iter().filter(|&p| t.contains(p)) {
        img.put_pixel(p.x as u32, p.y as u32, pixel);
    }
}

// TODO: instead of u32 use generic
fn draw_line<P: 'static + Pixel, Container: DerefMut<Target=[P::Subpixel]>>(mut l: Line, img: &mut ImageBuffer<P, Container>, pixel: P) {
    // transpose if the line is steep
    let mut steep = false;
    if (l.1.x - l.0.x).abs() < (l.1.y - l.0.y).abs() {
        let tmp = l.0.x;
        l.0.x = l.0.y;
        l.0.y = tmp;
        let tmp = l.1.x;
        l.1.x = l.1.y;
        l.1.y = tmp;
        steep = true;
    }
    // left to right
    if l.0.x > l.1.x {
        mem::swap(&mut l.0, &mut l.1);
    }
    let d = l.1 - l.0;
    // how much further from the real line will we get in each iteration
    let derror2 = d.y.abs() * 2;
    let mut error2 = 0;
    let mut y = l.0.y;
    for x in l.0.x..l.1.x {
        match steep {
            true => img.put_pixel(y as u32, x as u32, pixel),
            false => img.put_pixel(x as u32, y as u32, pixel)
        }
        error2 += derror2;
        if error2 > 1 {
            if l.1.y > l.0.y { y += 1; } else { y -= 1; }
            error2 -= d.x * 2;
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

    for face in (0..mesh.indices.len()).step_by(3) {
        let vertices = &mesh.indices[face..face + 3];
        let scale = |p: &[f32]| -> Vector2<i32> {
            let x0 = (p[0] + 1.) * width as f32 / 2. - 1.;
            let y0 = (p[1] + 1.) * height as f32 / 2. - 1.;
            Vector2::new(x0 as i32, y0 as i32)
        };
        let get_point = |i| -> &[f32] {
            &mesh.positions[(i * 3) as usize..(i * 3 + 3) as usize]
        };
        let as_vector = |p: &[f32]| -> Vector3<f32> {
            Vector3::new(p[0], p[1], p[2])
        };
        let (x, y, z) = (as_vector(get_point(vertices[0])), as_vector(get_point(vertices[1])), as_vector(get_point(vertices[2])));
        let light_dir = Vector3::new(0., 0., -1.);
        let n = ((z - x).cross(&(y - x))).normalize();
        let intensity = n.dot(&light_dir);
        if intensity > 0. {
            let t = Triangle(scale(get_point(vertices[0])), scale(get_point(vertices[1])), scale(get_point(vertices[2])));
            draw_triangle(t, &mut image, image::Rgb([(intensity * 255.) as u8, (intensity * 255.) as u8, (intensity * 255.) as u8]));
        }
    }
    flip_vertical_in_place(&mut image);
    image.save(&Path::new(out)).unwrap();
}

fn main() {
    render("models/african_head.obj", "african_head.png", 600, 600);
}
