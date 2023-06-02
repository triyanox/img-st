use image::{open, GenericImageView, ImageBuffer, Pixel, Rgba};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn encode_byte(pixel: u8, byte: u8) -> u8 {
    let pixel_bit = pixel & 0x01;
    let byte_bit = (byte >> 7) & 0x01;
    ((pixel >> 1) << 1) | byte_bit | pixel_bit
}

pub fn encode_message(input_file: &str, output_file: &str, message: &str) -> Result<(), String> {
    let img = open(input_file).map_err(|_| "Failed to open input file")?;
    let mut rng = thread_rng();
    let mut output: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();

    let message_bytes = message.as_bytes();
    let message_length = message_bytes.len();
    let message_length_bytes: [u8; 4] = [
        ((message_length >> 24) & 0xFF) as u8,
        ((message_length >> 16) & 0xFF) as u8,
        ((message_length >> 8) & 0xFF) as u8,
        (message_length & 0xFF) as u8,
    ];

    let mut pixels: Vec<(u32, u32)> = img
        .pixels()
        .map(|(x, y, _)| (x, y))
        .collect::<Vec<(u32, u32)>>();

    pixels.shuffle(&mut rng);

    let mut current_pixel = 0;

    for byte in message_length_bytes.iter().chain(message_bytes.iter()) {
        if current_pixel >= pixels.len() {
            return Err("Insufficient pixels to encode the entire message".to_owned());
        }

        let (x, y) = pixels[current_pixel];
        let mut pixel = output.get_pixel_mut(x, y).to_owned();
        let channels = pixel.channels_mut();

        let encoded_byte = encode_byte(channels[3], *byte);
        channels[3] = encoded_byte;

        current_pixel += 1;
        output.put_pixel(x, y, pixel);
    }

    output
        .save(output_file)
        .map_err(|_| "Failed to save output file")?;

    Ok(())
}
