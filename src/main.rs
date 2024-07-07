extern crate url_encor;

mod preprocessing;
use preprocessing::*;
use url_encor::Encoder;
fn main() {
    let test_string = String::from(r#"©ÃÃãýÿ‡"#);
    let encoded_string = test_string.url_encode();
    println!("{}", encoded_string);
    let decoded_string = encoded_string.url_decode();
    //let decoded_string = String::from("Hello %2G:");
    println!("{}", decoded_string);
    //printable_array(Vec::from(from_hex_bytes_to_value_bytes()), 6);


}
