use detect_single_xor;
use std::fs;

fn main() {
    let data = fs::read_to_string("encrypted_strings.txt").expect("Unable to read file");
    let decrypted = detect_single_xor::detect_single_xor(data);
    println!("{:?}", &decrypted.encrypted_strings);
}