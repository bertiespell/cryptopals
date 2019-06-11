extern crate decrypt_repeating;
use std::fs;
pub mod base_64;
use decrypt_repeating::decrypt_repeating_xor;

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let text_bytes = base_64::decode(text.as_bytes());
    let decypted = decrypt_repeating_xor(text_bytes);
    println!("Decrypted: {:?}", decypted);
}