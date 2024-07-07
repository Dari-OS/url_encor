use std::fmt::Display;

/// # Preprocessed array of values that needs to be converted
/// This is the result of calling the `printable_array(Vec::from(preprocess_ascii_convertable()), 6);`
///
/// It shows which chars should get encoded.
///
/// - `1` lets the program know to **NOT** encode the char
/// - `0` lets the program know to encode the char
pub const PREPROCESSED_ARRAY: [u8; 256] = [0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 1, 0,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1,
    1, 0, 0, 0, 0, 1, 0,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 0, 0,
    0, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0 ];

pub const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// # Preprocessed Hexadecimal in bytes and their values
/// This is the result of calling the `printable_array(Vec::from(from_hex_bytes_to_value_bytes()), 6);`
///
/// This "takes" a byte representing a hexadecimal char and gives its decimal value
///
/// - `-1` lets the program know that the provided byte is not hex
/// - `everything else` lets the program know the hex value of the given byte
pub const HEX_BYTE_TO_HEX_VALUE: [i16; 256] =
        [-1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        0, 1, 2, 3, 4, 5, 6,
        7, 8, 9, -1, -1, -1, -1,
        -1, -1, -1, 10, 11, 12, 13,
        14, 15, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        10, 11, 12, 13, 14, 15, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1 ];


/// This function converts bytes that represent hex values in ascii to their
/// respective values
#[allow(dead_code)]
pub fn from_hex_bytes_to_value_bytes() -> [i16; 256] {
    let mut array = [-1; 256];

    for index in 0..256usize {
        match index as u8{
            b'0'..=b'9' => array[index] = (index as u8 - b'0') as i16,
            b'A'..=b'F' => array[index] = (index as u8 - b'A' + 10) as i16,
            b'a'..=b'f' => array[index] = (index as u8 - b'a' + 10) as i16,
            _ => continue ,
        }
    }

    array
}

/// This prints an array in a more readable way
///
#[allow(dead_code)]
pub fn printable_array<T>(array: Vec<T>, number_before_linebreak: u8)
where T: Display{
    let mut current_number: u8 = 0;
    print!("[");
    for (index, current) in array.iter().enumerate() {
        if current_number >= number_before_linebreak {
            current_number = 0;
            println!();
        } else {
            current_number += 1;
        }
        print!(
            "{}{} ",
            current,
            if index == array.len() - 1 { "" } else { "," }
        );
    }

    print!("]");
}

/// # Returns
/// An array that shows if the ascii char should get encoded:
///
/// - `1` - Char should not get encoded
/// - `0` - Char should get encoded
///
#[allow(dead_code)]
pub fn preprocess_ascii_convertable() -> [u8; 256] {
    let mut array = [0_u8; 256];
    for index in 0..=255u8 {
        match index as char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                array[index as usize] = 1u8
            }
            _ => array[index as usize] = 0u8,
        }
    }

    array
}

