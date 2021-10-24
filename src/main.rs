use std::fs::File;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

fn main() {
    println!("Put in the text you want to be turned into a image, or the image path to say the text");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_string();
    if File::open(line.clone().trim()).is_ok() {
        let img = image::open(line).unwrap();
        let (width, height) = img.dimensions();
        let mut string = "".to_string();
        for x in 0..width {
            for y in 0..height {
                string.push_str(unsafe { std::str::from_utf8_unchecked(&[format!("{:?}", img.get_pixel(x, y).to_luma())[6..format!("{:?}", img.get_pixel(x, y).to_luma()).len()-2].parse::<u8>().unwrap()])});
            }
        }
        println!("{}", string);
    }
    else {
        let img: RgbImage = ImageBuffer::new((line.len() as f32).sqrt().ceil() as u32, (line.len() as f32).sqrt().ceil() as u32);
        let img = ImageBuffer::from_fn((line.len() as f32).sqrt().ceil() as u32, (line.len() as f32).sqrt().ceil() as u32, |x, y| -> image::Luma<u8> {
            let item = line.chars().nth(((y * (line.len() as f32).sqrt().ceil() as u32)+x) as usize);
            match item {
                Some(x) => image::Luma([x as u8]),
                None => image::Luma([0u8]),
            }
        });
        img.save("text-to-image.png").unwrap();
        println!("Saved to text-to-image.png");
    }
}
