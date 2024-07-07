extern crate url_encor;

use url_encor::Encoder;
fn main() {
    let test_string = String::from(r#"Hello, World! "#);
    let encoded_string = test_string.url_encode();
    println!("{}", encoded_string);
    //let decoded_string = encoded_string.url_decode();
    let decoded_string = String::from("Hello %2G:");
    println!("{}", decoded_string)
}
