use tgaimage::{TGAImage, TGAColor, WHITE, RED};
use std::mem::swap;
use crate::line;
use crate::Vec2i;

pub fn triangle1(mut t0: Vec2i, mut t1: Vec2i, mut t2: Vec2i, image: &mut TGAImage, color: TGAColor) {
    if t0.1 > t1.1 {
        swap(&mut t0, &mut t1);
    }
    if t0.1 > t2.1 {
        swap(&mut t0, &mut t2);
    }
    if t1.1 > t2.1 {
        swap(&mut t1, &mut t2);
    }
    let dy01 = (t1.1 - t0.1) as f32;
    let dy02 = (t2.1 - t0.1) as f32;
    let dy12 = (t2.1 - t1.1) as f32;
    for y in t0.1..=t1.1 {
        let scale01 = (y - t0.1) as f32 / dy01;
        let scale02 = (y - t0.1) as f32 / dy02;
        let nx1 = (t1.0 as f32 - t0.0 as f32) as f32 * scale01 + t0.0 as f32;
        let nx2 = (t2.0 as f32 - t0.0 as f32) as f32 * scale02 + t0.0 as f32;
        line(nx1 as u64, y, nx2 as u64, y, image, color);
    }
    for y in t1.1..=t2.1 {
        let scale12 = (y - t1.1) as f32 / dy12;
        let scale02 = (y - t0.1) as f32 / dy02;
        let nx1 = (t2.0 as f32 - t1.0 as f32) * scale12 + t1.0 as f32;
        let nx2 = (t2.0 as f32 - t0.0 as f32) * scale02 + t0.0 as f32;
        line(nx1 as u64, y, nx2 as u64, y, image, color);
    }
}

pub fn triangle(t0: Vec2i, t1: Vec2i, t2: Vec2i, image: &mut TGAImage, color: TGAColor) {
    triangle1(t0, t1, t2, image, color);
}


#[cfg(test)]
mod test {
    extern crate tgaimage;
    use tgaimage::{RED, WHITE, GREEN};
    use super::*;

    #[test]
    fn test_basic() {
        let mut image = TGAImage::new(200, 200);
        let t0 = vec![(10, 70), (50, 160), (70, 80)];
        let t1 = vec![(180, 50), (150, 1), (70, 180)]; 
        let t2 = vec![(180, 150), (120, 160), (130, 180)]; 
        triangle(t0[0], t0[1], t0[2], &mut image, RED); 
        triangle(t1[0], t1[1], t1[2], &mut image, WHITE); 
        triangle(t2[0], t2[1], t2[2], &mut image, GREEN);
        let _ = image.write_to_file("test.tga");
    }
}

