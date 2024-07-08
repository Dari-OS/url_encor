extern crate url_encor;

use url_encor::Encoder;
fn main() {
    let test_string = String::from(r#"Hello, 🌎! こんにちは。Привет! ✨🦄✨"#);
    let encoded_string = test_string.url_encode();
    println!("{}", encoded_string);
    let decoded_string = encoded_string.url_decode();
    //let decoded_string = String::from("Hello%2C%20%F0%9F%8C%8E%21%20%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF%E3%80%82%20%D0%9F%D1%80%D0%B8%D0%B2%D0%B5%D1%82%21%20%E2%9C%A8%F0%9F%A6%84%E2%9C%A8").url_decode();
    println!("{}", decoded_string);
    //OUTPUT: Â©Ã Ã Ã£Ã½Ã¿â ¡


}
