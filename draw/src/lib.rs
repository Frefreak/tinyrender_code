use tgaimage::{TGAImage, TGAColor};
use std::mem::swap;

pub fn line1(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, img: &mut TGAImage, color: TGAColor) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }
    for x in x0..=x1 {
        let t = (x - x0) as f64 / (x1 - x0) as f64;
        let y = y0 as f64 * (1. - t) + y1 as f64 * t;
        if steep {
            img.set(y as u16, x as u16, color);
        } else {
            img.set(x as u16, y as u16, color);
        }
    }
}
