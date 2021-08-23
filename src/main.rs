use tgaimage::{TGAImage, TGAImageResult};
use tgaimage::{WHITE};
use draw::line;

fn main() -> TGAImageResult<()> {
    let model = wavefront::Obj::from_file(std::env::args().nth(1).unwrap())?;
    let width = 1600;
    let height = 1600;
    let mut image = TGAImage::new(width+1, height+1);
    for face in model.triangles() {
        for j in 0..3 {
            let v0 = face[j].position();
            let v1 = face[(j+1) % 3].position();
            let x0 = (v0[0] + 1.) * width as f32 / 2.;
            let y0 = (v0[1] + 1.) * height as f32 / 2.;
            let x1 = (v1[0] + 1.) * width as f32 / 2.;
            let y1 = (v1[1] + 1.) * height as f32 / 2.;
            // println!("{} {} {} {} {:?} {:?}", x0, y0, x1, y1, v0, v1);
            line(x0 as u64, y0 as u64, x1 as u64, y1 as u64, &mut image, WHITE);
        }
    }
    image.write_to_file("test.tga")?;
    Ok(())
}
