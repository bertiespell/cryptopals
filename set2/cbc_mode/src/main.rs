use pkcs_padding;
use std::fs;
use cbc_mode;
fn main() {
    let key = "YELLOW SUBMARINE";
    let IV =  "\x00\x00\x00";
    // let IV_lengthened = pkcs_padding::pad_with_bytes(IV.as_bytes().to_owned(), key.as_bytes().len(), 0 as u8);
    let encypted_text = fs::read_to_string("encrypted_data.txt").expect("Unable to read file");
    // dbg!(encypted_text.as_bytes().to_owned());
    let decrypted_text = cbc_mode::decrypt_string(
        &encypted_text, 
        key, 
        IV
    );
    // let decrypted_text = String::from_utf8(decrypted_bytes).unwrap();
    println!("decrypted_text: {:?}", decrypted_text);
}