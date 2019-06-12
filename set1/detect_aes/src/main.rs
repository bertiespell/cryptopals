/**
text.txt contains hex-encoded ciphertexts.

One of them has been encrypted with ECB.

Detect it.

Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte plaintext block will always produce the same 16 byte ciphertext.
 */

extern crate hex;
use std::fs;
use std::collections::HashMap;
use std::str;

fn main() {
    let hex_encoded_data = fs::read_to_string("text.txt")
        .expect("Unable to read file");
    let ciphertexts = hex_encoded_data
        .split("\n")
        .collect::<Vec<&str>>();

    // the same 16 byte plaintext block will always produce the same 16 byte ciphertext. i.e. common 16 byte plain text block - repeats commonly...

    let mut needles = ciphertexts
        .iter()
        .map(|text| {
            text
                .as_bytes()
                .chunks(16)
                .map(str::from_utf8)
                .fold((HashMap::new(), String::from(*text)), |mut acc, chunk| {
                    let chunk_entry = acc.0
                        .entry(chunk.unwrap())
                        .or_insert(0);
                    
                    *chunk_entry += 1;
                    acc
                })
        })
        .collect::<Vec<_>>();

    fn add_scores_above_1(acc: i32, entry: &i32) -> i32 {
        if *entry > 1 {
            return acc + *entry;
        }
        acc
    }

    needles
        .sort_by(|a, b| b.0.values().fold(0, add_scores_above_1).cmp(&a.0.values().fold(0, add_scores_above_1)));

    println!("Found needle: {:?}", needles[0]);
}

//d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a