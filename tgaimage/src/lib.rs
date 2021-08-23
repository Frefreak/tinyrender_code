use std::{io::Write, path::Path};

use chrono::Utc;
use serde::Serialize;
use bincode;

#[derive(Serialize)]
struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    image_type: u8,
    color_map_specification: [u8; 5],
    image_specification: [u8; 10],
}

#[derive(Serialize)]
struct TGAData {
    image_id: String,
    color_map_specification: Vec<u8>,
    data: Vec<TGAColor>,

}

pub type TGAColor = (u8, u8, u8);

#[derive(Serialize)]
pub struct TGAImage {
    header: TGAHeader,
    data: TGAData,
    #[serde(skip)]
    width: u16,
    #[serde(skip)]
    height: u16,
}

pub const BLACK: TGAColor = (0, 0, 0);
pub const WHITE: TGAColor = (0xFF, 0xFF, 0xFF);
pub const RED: TGAColor = (0, 0, 0xFF);
pub const GREEN: TGAColor = (0, 0xFF, 0);
pub const BLUE: TGAColor = (0xFF, 0, 0);
pub const YELLOW: TGAColor = (0, 0xFF, 0xFF);
pub const CYAN: TGAColor = (0xFF, 0xFF, 0);
pub const PURPLE: TGAColor = (0xFF, 0, 0xFF);

impl TGAImage {
    pub fn new(width: u16, height: u16) -> Self {
        let id = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let header = TGAHeader {
            id_length: id.len() as u8,
            color_map_type: 0,
            image_type: 2,
            color_map_specification: [0, 0, 0, 0, 0],
            image_specification: [0, 0, 0, 0,
                (width & 0xFF) as u8, (width >> 8) as u8,
                (height & 0xFF) as u8, (height >> 8) as u8,
                24, 0],
        };
        let data = TGAData{
            image_id: id,
            color_map_specification: vec![],
            data: vec![BLACK; (width as u64 * height as u64) as usize],
        };
        TGAImage {
            header,
            data,
            width,
            height,
        }
    }

    pub fn write_to_file<F>(&self, fp: F) -> TGAImageResult<()>
        where F: AsRef<Path> {
        let bs = bincode::serialize(self)?;
        let mut f = std::fs::File::create(fp)?;
        f.write_all(&bs)?;
        Ok(())
    }

    pub fn set(&mut self, x: u16, y: u16, color: TGAColor) {
        self.data.data[(y as u64 * self.width as u64 + x as u64) as usize] = color;
    }
    pub fn get(&self, x: u16, y: u16) -> TGAColor {
        self.data.data[(y as u64 * self.width as u64 + x as u64) as usize]
    }
    pub fn flip_vertically(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width/2 {
                self.data.data.swap((
                    i as u64 * self.width as u64 + j as u64) as usize,
                    (i as u64 * self.width as u64 + (self.width - 1 - j) as u64) as usize);
            }
        }
    }
}

pub type TGAImageError = Box<dyn std::error::Error>;
pub type TGAImageResult<T> = Result<T, TGAImageError>;

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn deserialize() {
        let mut img = TGAImage::new(100, 100);
        img.set(52, 41, RED);
        let bs = bincode::serialize(&img).unwrap();
        let mut f = std::fs::File::create("test.tga").unwrap();
        f.write_all(&bs).unwrap();
        assert!(Path::exists(Path::new("test.tga")));
        std::fs::remove_file("test.tga").unwrap();
    }
}
