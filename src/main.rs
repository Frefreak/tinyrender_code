use tgaimage::{TGAImage, RED, TGAImageResult};

fn main() -> TGAImageResult<()> {
    let mut image = TGAImage::new(100, 100);
    image.set(52, 41, RED);
    image.flip_vertically();
    image.write_to_file("output.tga")?;
    Ok(())
}
