mod preprocessing;

use preprocessing::{HEX_DIGITS, PREPROCESSED_ARRAY, HEX_BYTE_TO_HEX_VALUE};

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

pub fn decode(str_to_decode: &str) -> String {
    let mut decoded_bytes = Vec::with_capacity(str_to_decode.len());
    let bytes = str_to_decode.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(n1), Some(n2)) = (from_hex(bytes[i + 1]), from_hex(bytes[i + 2])) {
                decoded_bytes.push((n1 << 4) | n2);
                i += 3;
            } else {
                // Invalid percent-encoding, just push the original characters
                decoded_bytes.push(b'%');
                decoded_bytes.push(bytes[i + 1]);
                decoded_bytes.push(bytes[i + 2]);
                i += 3;
            }
        } else if bytes[i] == b'+' { //Some legacy systems decode a space as '+'
            decoded_bytes.push(b' ');
            i += 1;
        } else {
            decoded_bytes.push(bytes[i]);
            i += 1;
        }
    }

    String::from_utf8_lossy(&decoded_bytes).into_owned()
}

fn from_hex(c: u8) -> Option<u8> {
    HEX_BYTE_TO_HEX_VALUE[c as usize].try_into().ok()
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


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_preprocessed_hex_conversion() {
        assert_eq!(HEX_BYTE_TO_HEX_VALUE, preprocessing::from_hex_bytes_to_value_bytes())
    }

    #[test]
    fn test_preprocessed_ascii_convertable() {
        assert_eq!(PREPROCESSED_ARRAY, preprocessing::preprocess_ascii_convertable())
    }


    #[test]
    fn test_hex_values() {

        let hex_chars: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'
        ];

        for i in 0..16usize {
            assert_eq!(HEX_DIGITS[i], hex_chars[i] as u8);
        }
    }


    #[test]
    fn test_from_hex_function() {
        for i in 0..HEX_BYTE_TO_HEX_VALUE.len() {

            let result1 = from_hex(i as u8);
            let result2 = HEX_BYTE_TO_HEX_VALUE[i];
            if let (None, -1) = (result1, result2)  {
                continue;
            } else if result1.is_none() {
                assert!(false, "{}" ,format!("The function returned {:?} but {} was expected", result1, result2))
            }

            assert_eq!(result2, result1.unwrap() as i16)
        }
    }
}
