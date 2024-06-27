mod preprocessing;

use preprocessing::{PREPROCESSED_ARRAY, HEX_DIGITS};


pub fn url_encode(str_to_encode: &str) -> String  {
    let mut encoded_string = String::with_capacity(str_to_encode.len() * 3);

    for &current_byte in str_to_encode.as_bytes() {
        if PREPROCESSED_ARRAY[current_byte as usize] == 0 {
            encoded_string.push('%');
            encoded_string.push(HEX_DIGITS[(current_byte >> 4) as usize] as char);
            encoded_string.push(HEX_DIGITS[(current_byte & 0xF) as usize] as char);
        } else {
            encoded_string.push(current_byte as char)
        }
    }

    return encoded_string;
}

pub trait Encoder<T = String> {
    fn encode_url(&self) -> T;

    fn decode_url(&self) -> T;
}

impl Encoder for String {
    fn encode_url(&self) -> String {
        url_encode(self)
    }

    fn decode_url(&self) -> String {
        todo!();
    }
}