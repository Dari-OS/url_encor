pub trait Encoder<T = String> {
    fn encode_url(&self) -> T;

    fn decode_url(&self) -> T;
}