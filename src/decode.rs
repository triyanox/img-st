use image::{open, GenericImageView, Pixel};

fn decode_byte(pixel: u8) -> u8 {
    pixel & 0x01
}

pub fn decode_message(input_file: &str) -> Result<String, String> {
    let img = open(input_file).map_err(|_| "Failed to open input file")?;
    let pixels: Vec<_> = img.pixels().collect();
    let mut current_byte = 0;
    let mut decoded_bytes: Vec<u8> = Vec::new();

    while current_byte < 4 {
        let mut byte_value = 0u8;
        for i in 0..8 {
            let byte_index = current_byte * 8 + i;
            if byte_index >= pixels.len() {
                return Err("Insufficient pixels to decode the entire message".to_owned());
            }

            let pixel_value = pixels[byte_index].2.channels()[3];
            let decoded_bit = decode_byte(pixel_value);
            byte_value = (byte_value << 1) | decoded_bit;
        }

        decoded_bytes.push(byte_value);
        current_byte += 1;
    }

    let message_length = (decoded_bytes[0] as usize) << 24
        | (decoded_bytes[1] as usize) << 16
        | (decoded_bytes[2] as usize) << 8
        | (decoded_bytes[3] as usize);

    let mut message_bytes: Vec<u8> = Vec::new();
    for i in 0..message_length {
        let mut byte_value = 0u8;
        for j in 0..8 {
            let byte_index = 4 * 8 + i * 8 + j;
            if byte_index >= pixels.len() {
                return Err("Insufficient pixels to decode the entire message".to_owned());
            }

            let pixel_value = pixels[byte_index].2.channels()[3];
            let decoded_bit = decode_byte(pixel_value);
            byte_value = (byte_value << 1) | decoded_bit;
        }

        message_bytes.push(byte_value);
    }

    String::from_utf8(message_bytes).map_err(|_| "Failed to decode message".to_owned())
}
