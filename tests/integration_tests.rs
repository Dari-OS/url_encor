use url_encor::{encode, decode, Encoder};

#[cfg(test)]
mod encoding_tests {
    use super::*;

    #[test]
    fn test_encode_ascii() {
        assert_eq!(encode("Hello, World!"), "Hello%2C%20World%21");
        assert_eq!(encode("abc123"), "abc123");
        assert_eq!(encode("user@example.com"), "user%40example.com");
        assert_eq!(encode("https://www.example.com/path?query=value"), "https%3A%2F%2Fwww.example.com%2Fpath%3Fquery%3Dvalue");
    }

    #[test]
    fn test_encode_special_characters() {
        assert_eq!(encode(r#"!@#$%^&*()_+{}[]|\:;"'<>,.?/"#), "%21%40%23%24%25%5E%26%2A%28%29_%2B%7B%7D%5B%5D%7C%5C%3A%3B%22%27%3C%3E%2C.%3F%2F");
        assert_eq!(encode(r#"@ſ€ðđð@ ¶ŋ¼³¬¼ŧ§§\"$\"\""#), "%40%C5%BF%E2%82%AC%C3%B0%C4%91%C3%B0%40%20%C2%B6%C5%8B%C2%BC%C2%B3%C2%AC%C2%BC%C5%A7%C2%A7%C2%A7%5C%22%24%5C%22%5C%22");
    }

    #[test]
    fn test_encode_unicode() {
        assert_eq!(encode("こんにちは"), "%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF");
        assert_eq!(encode("🚀✨🌈"), "%F0%9F%9A%80%E2%9C%A8%F0%9F%8C%88");
        assert_eq!(encode("Größe"), "Gr%C3%B6%C3%9Fe");
    }

    #[test]
    fn test_encode_mixed_content() {
        assert_eq!(encode("Hello, 世界!"), "Hello%2C%20%E4%B8%96%E7%95%8C%21");
        assert_eq!(encode("12345 αβγδε"), "12345%20%CE%B1%CE%B2%CE%B3%CE%B4%CE%B5");
    }
}

#[cfg(test)]
mod decoding_tests {
    use super::*;

    #[test]
    fn test_decode_ascii() {
        assert_eq!(decode("Hello%2C%20World%21"), "Hello, World!");
        assert_eq!(decode("abc123"), "abc123");
        assert_eq!(decode("user%40example.com"), "user@example.com");
        assert_eq!(decode("https%3A%2F%2Fwww.example.com%2Fpath%3Fquery%3Dvalue"), "https://www.example.com/path?query=value");
    }

    #[test]
    fn test_decode_special_characters() {
        assert_eq!(decode("%21%40%23%24%25%5E%26%2A%28%29_%2B%7B%7D%5B%5D%7C%5C%3A%3B%22%27%3C%3E%2C.%3F%2F"), "!@#$%^&*()_+{}[]|\\:;\"'<>,.?/");
        assert_eq!(decode("%20%7E%60"), " ~`");
    }

    #[test]
    fn test_decode_unicode() {
        assert_eq!(decode("%E3%81%93%E3%82%93%E3%81%AB%E3%81%A1%E3%81%AF"), "こんにちは");
        assert_eq!(decode("%F0%9F%9A%80%E2%9C%A8%F0%9F%8C%88"), "🚀✨🌈");
        assert_eq!(decode("Gr%C3%B6%C3%9Fe"), "Größe");
    }

    #[test]
    fn test_decode_mixed_content() {
        assert_eq!(decode("Hello%2C%20%E4%B8%96%E7%95%8C%21"), "Hello, 世界!");
        assert_eq!(decode("12345%20%CE%B1%CE%B2%CE%B3%CE%B4%CE%B5"), "12345 αβγδε");
    }

    #[test]
    fn test_decode_invalid_sequences() {
        assert_eq!(decode("%"), "%");
        assert_eq!(decode("%2"), "%2");
        assert_eq!(decode("%2G"), "%2G");
        assert_eq!(decode("100%"), "100%");
    }

    #[test]
    fn test_decode_plus_sign() {
        assert_eq!(decode("Hello+World"), "Hello World");
        assert_eq!(decode("1+2+3"), "1 2 3");
    }
}


fn encode_decode_test(input: &str) {
   let encoded = encode(input);
   let decoded = decode(&encoded);
   assert_eq!(decoded, input, "Failed on input: {}", input);
 }
#[cfg(test)]
mod combined_encoding_decoding_tests {
    use super::*;


    #[test]
    fn test_ascii_roundtrip() {
        encode_decode_test("Hello, World!");
        encode_decode_test("The quick brown fox jumps over the lazy dog.");
        encode_decode_test("!@#$%^&*()_+{}[]|\\:;\"'<>,.?/");
    }

    #[test]
    fn test_unicode_roundtrip() {
        encode_decode_test("こんにちは世界");
        encode_decode_test("🚀✨🌈🌍🔬🎨");
        encode_decode_test("Größe and 크기");
    }

    #[test]
    fn test_mixed_content_roundtrip() {
        encode_decode_test("Hello, 世界! 123 αβγ");
        encode_decode_test("1️⃣2️⃣3️⃣ ABC xyz");
        encode_decode_test("Résumé for John Doe: 專業技能");
    }

    #[test]
    fn test_complex_unicode_roundtrip() {
        // Test with various Unicode blocks and scripts
        encode_decode_test("العربية ‎ עִבְרִית ‎ 中文 ‎ हिन्दी ‎ 日本語 ‎ 한국어");
        encode_decode_test("Çırpıcı çayırı çayırında sürü çayırlaya çayırlaya yürüyordu");
        encode_decode_test("Ṱ̺̺̕h̼͓̲̦̳̘̲e͇̣̰̦̬͎ ̢̼̻̱̘h͚͎͙̙̣̗̗i̦̲̣̰̤v̻͍e̺̭̳̪̰-m̢iͅn̖̺̞̲̯̰d̵̼̟͙̩̼̘̳ ̞̥̱̳̭r̛̗̘e͙p͠r̼̞̻̭̗e̺̠̣͟s̘͇̳͍̝͉e͉̥̯̞̲͚̬͜ǹ̬͎͎̟̖͇̤t͍̬̤͓̼̭͘ͅi̪̱n͠g̴͉ ͏͉ͅc̬̟h͡a̫̻̯͘o̫̟̖͍̙̝͉s̗̦̲.̨̹͈̣");
        encode_decode_test("🏳️‍🌈 👨‍👩‍👧‍👦 🇺🇳 🏴‍☠️");
    }

    #[test]
    fn test_long_text_roundtrip() {
        let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

本文用于测试长文本的编码和解码。它包含各种字符，包括中文、日文和韩文。エンコーディングとデコーディングのテストのための長いテキスト。이것은 긴 텍스트의 인코딩 및 디코딩을 테스트하기 위한 것입니다.

1️⃣2️⃣3️⃣4️⃣5️⃣6️⃣7️⃣8️⃣9️⃣🔟

Σὲ γνωρίζω ἀπὸ τὴν κόψη
τοῦ σπαθιοῦ τὴν τρομερή,
σὲ γνωρίζω ἀπὸ τὴν ὄψη
ποὺ μὲ βία μετράει τὴ γῆ.

À propos de W.Æ.Bates & Co., ça va bien. Это предложение на русском языке.";

        encode_decode_test(long_text);
    }

    #[test]
    fn test_all_ascii_characters() {
        let all_ascii: String = (0..128).map(|i| i as u8 as char).collect();
        encode_decode_test(&all_ascii);
    }

    #[test]
    fn test_trait_implementation() {
        let original = "Hello, 世界! 🌍".to_string();
        let encoded = original.url_encode();
        let decoded = encoded.url_decode();
        assert_eq!(original, decoded);
    }
}
