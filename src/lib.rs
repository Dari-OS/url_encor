//! # url_encor Crate
//!
//! This crate provides functionality for URL encoding and decoding.
//! It includes both standalone functions and a trait implementation for convenient use with `String` types.
//! The trait uses generics, therefore the return value can be manipulated!
//!
//! # Examples
//!
//! #### _Encoding_ a String is as easy as it gets
//! ```rust
//! use url_encor::Encoder;
//!
//! fn main() {
//!     let string_to_encode = String::from("Hello, World!");
//!     println!("{}", string_to_encode.url_encode());
//!     //OUTPUT: Hello%2C%20World%21
//!
//!     assert_eq!(string_to_encode.url_encode(), "Hello%2C%20World%21")
//! }
//! ```
//!
//!
//! #### _Decoding_ is easy, too
//! ```rust
//! use url_encor::Encoder;
//!
//! fn main() {
//!     let string_to_decode = String::from("Hello%2C%20World%21");
//!     println!("{}", string_to_decode.url_decode());
//!     //OUTPUT: Hello, World!
//!
//!     assert_eq!(string_to_decode.url_decode(), "Hello, World!")
//! }
//! ```
//!
//!
//! #### Implementing custom _encoding_ logic is easy as well
//! ```rust
//! use std::fmt::{Debug, Formatter};
//! use url_encor::{Encoder, encode};
//!
//! fn main() {
//!     let custom_type_to_encode = CharVector(vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!' ]);
//!     println!("{:?}", custom_type_to_encode.url_encode());
//!     //OUTPUT: ['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1']
//!
//!
//!     assert_eq!(custom_type_to_encode.url_encode().0, vec!['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1'])
//! }
//!
//! pub struct CharVector(Vec<char>);
//!
//! impl Debug for CharVector {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         self.0.fmt(f)
//!     }
//! }
//!
//! impl Encoder<CharVector> for CharVector {
//!     fn url_encode(&self) -> CharVector {
//!         CharVector(encode(&self.0.iter().collect::<String>()).chars().collect())
//!     }
//!
//!     fn url_decode(&self) -> CharVector {
//!         todo!()
//!     }
//! }
//! ```
//!
//!
//! #### Implementation of custom _decoding_ logic
//! ```rust
//! use std::fmt::{Debug, Formatter};
//! use url_encor::{Encoder, decode};
//!
//! fn main() {
//!     let custom_type_to_decode = CharVector(vec!['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1']);
//!     println!("{:?}", custom_type_to_decode.url_decode());
//!     //OUTPUT: ['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!']
//!
//!
//!     assert_eq!(custom_type_to_decode.url_decode().0, vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!' ])
//! }
//!
//! pub struct CharVector(Vec<char>);
//!
//! impl Debug for CharVector {
//!     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//!         self.0.fmt(f)
//!     }
//! }
//!
//! impl Encoder<CharVector> for CharVector {
//!     fn url_encode(&self) -> CharVector {
//!         todo!()
//!     }
//!
//!     fn url_decode(&self) -> CharVector {
//!         CharVector(decode(&self.0.iter().collect::<String>()).chars().collect())
//!     }
//! }
//! ```
//! # Issues?!
//!
//! If you encounter any problem, bug or issue, please open a new [issue](https://github.com/Dari-OS/url_encor/issues/new)

mod preprocessing;

use preprocessing::{HEX_DIGITS, PREPROCESSED_ARRAY, HEX_BYTE_TO_HEX_VALUE};

/// Encodes a string using url_encor.
///
/// This function iterates through each byte of the input string and encodes it if necessary.
/// Characters that don't need encoding are left as-is.
///
/// # Arguments
///
/// * `str_to_encode` - A string slice that holds the text to be URL encoded.
///
/// # Returns
///
/// A new `String` containing the URL encoded text.
///
/// # Examples
///
/// ```
/// use url_encor::encode;
///
/// let encoded = encode("Hello, World!");
/// assert_eq!(encoded, "Hello%2C%20World%21");
/// ```
pub fn encode(str_to_encode: &str) -> String {
    let mut encoded_string = String::with_capacity(str_to_encode.len() * 3);

    for &current_byte in str_to_encode.as_bytes() {
        if PREPROCESSED_ARRAY[current_byte as usize] == 0 {
            // If the byte needs encoding, add a percent sign followed by two hex digits
            encoded_string.push('%');
            encoded_string.push(HEX_DIGITS[(current_byte >> 4) as usize] as char);
            encoded_string.push(HEX_DIGITS[(current_byte & 0xF) as usize] as char);
        } else {
            // If the byte doesn't need encoding, add it as-is
            encoded_string.push(current_byte as char)
        }
    }

    encoded_string
}

/// Decodes a URL-encoded string.
///
/// This function iterates through the input string, decoding percent-encoded characters
/// and handling the special case of '+' being decoded as a space.
///
/// # Arguments
///
/// * `str_to_decode` - A string slice that holds the text to be URL decoded.
///
/// # Returns
///
/// A new `String` containing the decoded text.
///
/// # Examples
///
/// ```
/// use url_encor::decode;
///
/// let decoded = decode("Hello%2C%20World%21");
/// assert_eq!(decoded, "Hello, World!");
/// ```
pub fn decode(str_to_decode: &str) -> String {
    let mut decoded_bytes = Vec::with_capacity(str_to_decode.len());
    let bytes = str_to_decode.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(n1), Some(n2)) = (from_hex(bytes[i + 1]), from_hex(bytes[i + 2])) {
                // Decode a valid percent-encoded sequence
                decoded_bytes.push((n1 << 4) | n2);
                i += 3;
            } else {
                // Invalid percent-encoding, just push the original characters
                decoded_bytes.push(b'%');
                decoded_bytes.push(bytes[i + 1]);
                decoded_bytes.push(bytes[i + 2]);
                i += 3;
            }
        } else if bytes[i] == b'+' {
            // Some legacy systems decode a space as '+'
            decoded_bytes.push(b' ');
            i += 1;
        } else {
            // Non-encoded character, push as-is
            decoded_bytes.push(bytes[i]);
            i += 1;
        }
    }

    String::from_utf8_lossy(&decoded_bytes).into_owned()
}

/// Converts a hexadecimal character (represented as byte) to its corresponding decimal value (represented as byte as well).
///
/// # Arguments
///
/// * `c` - A byte representing a hexadecimal character.
///
/// # Returns
///
/// An `Option<u8>` containing the numeric value if valid, or `None` if invalid.
fn from_hex(c: u8) -> Option<u8> {
    HEX_BYTE_TO_HEX_VALUE[c as usize].try_into().ok()
}

/// A trait for types that can be URL encoded and decoded.
    pub trait Encoder<T = String> {
        /// Encodes the value using URL encoding.
        fn url_encode(&self) -> T;

        /// Decodes the value from URL encoding.
        fn url_decode(&self) -> T;
    }

/// Implementation of the `Encoder` trait for `String`.
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