extern crate url_encor;

use url_encor::Encoder;
fn main() {
let test_string = String::from(r#"Hello !"&/?)"§$`)(="§`$=) €@"#);
    println!("{}", test_string.url_encode())
}