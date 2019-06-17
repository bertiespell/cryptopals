use pkcs_padding;
use std::fs;
use cbc_mode;
fn main() {
    let key = "YELLOW SUBMARINE";
    let IV =  "\x00\x00\x00";
    // initialisation vector needs to be the right length
    let IV_lengthened = pkcs_padding::pad_to(IV, key.as_bytes().len());
    dbg!(IV_lengthened);
    let encypted_text = fs::read_to_string("encrypted_data.txt").expect("Unable to read file");
    let decrypted_text = cbc_mode::decrypt();
    println!("Decrypted: {}", decrypted_text);
}