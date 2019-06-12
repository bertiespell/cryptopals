extern crate single_byte_xor_cipher;
use std::fs;

use single_byte_xor_cipher::cipher::cipher::decrypt;
use single_byte_xor_cipher::cipher::cipher::score_xored_hashes;

struct ScoredXorStrings {
    pub score: i32,
    pub encrypted_strings: String
}

impl ScoredXorStrings {
    fn new() -> ScoredXorStrings {
        ScoredXorStrings {
            score: 0,
            encrypted_strings: String::new()
        }
    }
}
fn main() {
    let data = fs::read_to_string("encrypted_strings.txt").expect("Unable to read file");
    let decrypted = data
        .lines()
        .map(|line| decrypt(line.as_bytes().to_vec()))
        .fold(ScoredXorStrings::new(), |acc, decrypted_line| {
            let score = score_xored_hashes(decrypted_line.decoded.as_bytes().to_vec());
            if score > acc.score {
                return ScoredXorStrings {
                    score,
                    encrypted_strings: decrypted_line.decoded
                };
            }
            acc
        });

    println!("{:?}", decrypt(decrypted.encrypted_strings.as_bytes().to_vec()).decoded);
}
