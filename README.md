
# url_encor :globe_with_meridians:

A small and lightweight:feather: rust library to encode and decode URLs!


## Goal :white_check_mark:

**url_encor** aims to provide fast :rocket: url _encoding_ and _decoding_  
It achieves this by doing most of the heavy lifting :muscle: using ___preprocessed___ data.  

The following things get ___preprocessed:___
 - Decimal to Hex conversion
 - Hex to Decimal conversion
 - Deciding whether the character should get encoded

Take a look at [this file](./src/preprocessing.rs) and see what gets ___preprocessed___!

## Usage :gear:
  
   
#### _Encoding_ a String is as easy as it gets
```rust
use url_encor::Encoder;

fn main() {
    let string_to_encode = String::from("Hello, World!");
    println!("{}", string_to_encode.url_encode());
    //OUTPUT: Hello%2C%20World%21
    
    assert_eq!(string_to_encode.url_encode(), "Hello%2C%20World%21")
}
```
  
  
#### _Decoding_ is easy, too
```rust
use url_encor::Encoder;

fn main() {
    let string_to_decode = String::from("Hello%2C%20World%21");
    println!("{}", string_to_decode.url_decode());
    //OUTPUT: Hello, World!

    assert_eq!(string_to_decode.url_decode(), "Hello, World!")
}
```
  
  
#### Implementing custom _encoding_ logic is easy as well
```rust
use std::fmt::{Debug, Formatter};
use url_encor::{Encoder, encode};

fn main() {
    let custom_type_to_encode = CharVector(vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!' ]);
    println!("{:?}", custom_type_to_encode.url_encode());
    //OUTPUT: ['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1']


    assert_eq!(custom_type_to_encode.url_encode().0, vec!['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1'])
}

pub struct CharVector(Vec<char>);

impl Debug for CharVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Encoder<CharVector> for CharVector {
    fn url_encode(&self) -> CharVector {
        CharVector(encode(&self.0.iter().collect::<String>()).chars().collect())
    }

    fn url_decode(&self) -> CharVector {
        todo!()
    }
}
```
  

#### Implementation of custom _decoding_ logic 
```rust
use std::fmt::{Debug, Formatter};
use url_encor::{Encoder, decode};

fn main() {
    let custom_type_to_decode = CharVector(vec!['H', 'e', 'l', 'l', 'o', '%', '2', 'C', '%', '2', '0', 'W', 'o', 'r', 'l', 'd', '%', '2', '1']);
    println!("{:?}", custom_type_to_decode.url_decode());
    //OUTPUT: ['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!']


    assert_eq!(custom_type_to_decode.url_decode().0, vec!['H', 'e', 'l', 'l', 'o', ',', ' ', 'W', 'o', 'r', 'l', 'd', '!' ])
}

pub struct CharVector(Vec<char>);

impl Debug for CharVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Encoder<CharVector> for CharVector {
    fn url_encode(&self) -> CharVector {
        todo!()
    }

    fn url_decode(&self) -> CharVector {
        CharVector(decode(&self.0.iter().collect::<String>()).chars().collect())
    }
}
```
## Related links :link:

 - [crates.io](https://crates.io/crates/url_encor/)
 - [lib.rs](https://lib.rs/crates/url_encor)


## Issues :interrobang:

If you encounter any problem, bug or issue, please open a new [issue](https://github.com/Dari-OS/url_encor/issues/new)

