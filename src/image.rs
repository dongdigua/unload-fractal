use image;
use crate::hash::hash;

pub fn generate(imgx: u32, imgy: u32, filename: &String) {
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let chunkx: i32 = (x as i32) - ((imgx / 2) as i32);
        let chunky: i32 = ((imgy / 2) as i32) - (y as i32);
        if chunkx == 0 && chunky == 0 {
            *pixel = image::Rgb([0, 255, 0]);
        } else {
            let hash = (255 - hash(chunkx, chunky)) as u8;
            *pixel = image::Rgb([hash, hash, hash]);
        }
    }

    imgbuf.save(filename).unwrap();
}
