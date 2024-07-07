mod preprocessing;

use preprocessing::{HEX_DIGITS, PREPROCESSED_ARRAY};

pub fn encode(str_to_encode: &str) -> String {
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

//TODO: Add multi-byte utf8 support
pub fn decode(str_to_decode: &str) -> String {
    let mut decoded_string = String::with_capacity(str_to_decode.len());

    let bytes = str_to_decode.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if let (Some(b'%'), Some(n1), Some(n2)) = (bytes.get(i), bytes.get(i + 1), bytes.get(i + 2))
        {

            let decoded_byte: Option<u8> = match (from_hex(*n1), from_hex(*n2)) {
                (Some(n1), Some(n2)) => Some(n1 << 4 | n2),
                _ => None,
            };

            if let Some(temp) = decoded_byte {
                decoded_string.push(temp as char);
                i += 3;
                continue;
            }
        }

        decoded_string.push(bytes[i] as char);
        i += 1;
    }
    decoded_string
}

fn from_hex(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'A'..=b'F' => Some(c - b'A' + 10),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

pub trait Encoder<T = String> {
    fn url_encode(&self) -> T;

    fn url_decode(&self) -> T;
}

impl Encoder for String {
    fn url_encode(&self) -> String {
        encode(self)
    }

    fn url_decode(&self) -> String {
        decode(self)
    }
}
