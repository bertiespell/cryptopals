use std::fs;
use decrypt_repeating::base_64;
use aes_in_ecb::decrypt_data;

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let text_bytes = base_64::decode(text.as_bytes());
    let key = b"YELLOW SUBMARINE";
    let iv = b"ABCDEFGHIJKLMNOP";
    let decrypted = decrypt_data(text_bytes, key, iv.to_vec());
    let final_text = String::from_utf8(decrypted.unwrap()).unwrap();
    println!("Decrypted text: {:?}", final_text);
}