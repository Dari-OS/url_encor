/// # Preprocessed array of values that needs to be converted
/// This is the result of calling the `printable_ascii_convertable(number);`
///
/// It shows which chars should get encoded.
///
/// - `1` lets the program know to **NOT** encode the chat
/// - `0` lets the program know to encode the chat
pub const PREPROCESSED_ARRAY: [u8; 256] =
    [0, 0, 0, 0, 0, 0,
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



/// This prints the array of ascii chars that should get encoded (or not) in a more readable way
///
#[allow(dead_code)]
pub fn printable_ascii_convertable(number_before_linebreak: u8) {
let mut current_number: u8 = 0;
let array = preprocess_ascii_convertable();
    print!("[");
for (index, current) in array.iter().enumerate() {
    if current_number >= number_before_linebreak {
        current_number = 0;
        println!();
    } else {
        current_number += 1;
    }
    print!("{}{} ", current, if index == array.len() -1 {""} else {","});
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
fn preprocess_ascii_convertable() -> [u8; 256] {
    let mut array = [0_u8; 256];
    for index in 0..=255u8 {
        match index as char {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => array[index as usize] = 1u8,
            _ => array[index as usize] = 0u8,
        }
    }

    array
}