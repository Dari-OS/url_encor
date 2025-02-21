use crate::{Encoder, encode, decode};
/// Implementation of the `Encoder` trait for `String`.
impl Encoder for String {
    fn url_encode(&self) -> String {
        encode(self)
    }

    fn url_decode(&self) -> String {
        decode(self)
    }
}

impl Encoder for &str {
    fn url_encode(&self) -> String {
        encode(self)
    }

    fn url_decode(&self) -> String {
        decode(self)
    }
}



impl Encoder<Vec<String>> for Vec<String> {
    fn url_encode(&self) -> Vec<String> {
        self.iter().map(|element| {
            encode(element)
        }).collect()
    }

    fn url_decode(&self) -> Vec<String> {
        self.iter().map(|element| {
            encode(element)
        }).collect()
    }
}



