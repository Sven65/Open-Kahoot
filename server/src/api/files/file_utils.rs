use image::*;
use webp::{Encoder, WebPMemory};
use image::DynamicImage;

pub fn convert_to_webp(img: Vec<u8>) -> Vec<u8> {
	let img = image::load_from_memory(&img).expect("Failed to make image from image?");

    let (w, h) = img.dimensions();
    // Optionally, resize the existing photo and convert back into DynamicImage
    let size_factor = 1.0;
    let img = DynamicImage::ImageRgba8(imageops::resize(
        &img,
        (w as f64 * size_factor) as u32,
        (h as f64 * size_factor) as u32,
        imageops::FilterType::Triangle,
    ));

    // Create the WebP encoder for the above image
    let encoder: Encoder = Encoder::from_image(&img).unwrap();
    // Encode the image at a specified quality 0-100
    let webp: WebPMemory = encoder.encode(90f32);

	webp.to_vec()
}