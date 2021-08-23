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
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror = (dy as f64 / dx as f64).abs();
    let mut error = 0.;
    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            img.set(y as u16, x as u16, color);
        } else {
            img.set(x as u16, y as u16, color);
        }
        error += derror;
        if error > 0.5 {
            y += if y1 > y0 {1} else {-1};
            error -= 1.;
        }
    }
}

pub fn line2(mut x0: i64, mut y0: i64, mut x1: i64, mut y1: i64, img: &mut TGAImage, color: TGAColor) {
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
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = (dy as i64).abs() * 2;
    let mut error2 = 0;
    let mut y = y0;
    let offset = if y1 > y0 {1} else {-1};
    for x in x0..=x1 {
        if steep {
            img.set(y as u16, x as u16, color);
        } else {
            img.set(x as u16, y as u16, color);
        }
        error2 += derror2;
        if error2 > dx {
            y += offset;
            error2 -= 2 * dx;
        }
    }
}

pub fn line(x0: u64, y0: u64, x1: u64, y1: u64, img: &mut TGAImage, color: TGAColor) {
    line1(x0 as i64, y0 as i64, x1 as i64, y1 as i64, img, color);
}
