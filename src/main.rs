use tgaimage::{TGAImage, TGAImageResult};
use tgaimage::{WHITE, RED};
use draw::line1;

fn main() -> TGAImageResult<()> {
    let mut image = TGAImage::new(100, 100);
    for _ in 1..1000000 {
        line1(13, 20, 80, 40, &mut image, WHITE);
        line1(20, 13, 40, 80, &mut image, RED); 
        line1(80, 40, 13, 20, &mut image, RED);
    }
    // image.write_to_file("test.tga")?;
    Ok(())
}
