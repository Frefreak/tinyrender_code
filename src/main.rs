use tgaimage::{TGAImage, TGAImageResult};
use rand::random;
use draw::{triangle, Vec2i};

fn main() -> TGAImageResult<()> {
    let model = wavefront::Obj::from_file(std::env::args().nth(1).unwrap())?;
    let width = 1600;
    let height = 1600;
    let mut image = TGAImage::new(width+1, height+1);
    for face in model.triangles() {
        let coords: Vec<Vec2i> = (0..3).map(|j| {
            let v = face[j].position();
            (((v[0] + 1.) * width as f32 / 2.) as u64, ((v[1] + 1.) * height as f32 / 2.) as u64)
        }).collect();
        triangle(coords[0], coords[1], coords[2], &mut image,
                (random::<u8>(), random::<u8>(), random::<u8>()));
    }
    image.write_to_file("test.tga")?;
    Ok(())
}
