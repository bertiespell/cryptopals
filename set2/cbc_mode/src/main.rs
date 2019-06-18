use std::fs;
use cbc_mode;
fn main() {
    let key = "YELLOW SUBMARINE";
    let iv =  "\x00\x00\x00";
    let encypted_text = fs::read_to_string("encrypted_data.txt").expect("Unable to read file");
    let decrypted_text = cbc_mode::decrypt_string(
        &encypted_text, 
        key, 
        iv
    );
    println!("decrypted_text: {:?}", decrypted_text);
}