extern crate single_byte_xor_cipher;
use std::fs;

use single_byte_xor_cipher::cipher::cipher::decrypt;
use single_byte_xor_cipher::cipher::cipher::find_best;
use single_byte_xor_cipher::cipher::cipher::score_xored_hashes;
fn main() {
    let data = fs::read_to_string("encrypted_strings.txt").expect("Unable to read file");
    let decrypted = data
        .lines()
        .map(|line| decrypt(line.as_bytes().to_vec()))
        .fold((0, String::new()), |acc, decrypted_line| {
            let score = score_xored_hashes(decrypted_line.1.as_bytes().to_vec());
            if score > acc.0 {
                return (score, decrypted_line.1);
            }
            acc
        });

    println!("{:?}", decrypted);
}
