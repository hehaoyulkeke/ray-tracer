use crate::vec3::Vec3;
use image::ImageBuffer;


pub fn gen_png(img: Vec<Vec<Vec3>>, filename: &str) -> std::io::Result<()>{
    let sizey = img.len() as u32;
    let sizex = img[0].len() as u32;
    let mut imgbuf = ImageBuffer::new(sizex, sizey);
    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.r() as u8, pixel.g() as u8, pixel.b() as u8]));
        }
    } 
    imgbuf.save(filename)?;
    println!("successfully wrote to {}", filename);
    Ok(())
}