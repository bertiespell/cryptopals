/**
text.txt contains hex-encoded ciphertexts.

One of them has been encrypted with ECB.

Detect it.

Remember that the problem with ECB is that it is stateless and deterministic; the same 16 byte plaintext block will always produce the same 16 byte ciphertext.
 */

extern crate hex;
use std::fs;
use std::collections::HashMap;

fn main() {
    let hex_encoded_data = fs::read_to_string("text.txt").expect("Unable to read file");
    let ciphertexts = hex_encoded_data
        .split("\n")
        .map(|text| hex::decode(text).unwrap())
        .collect::<Vec<_>>();

    // the same 16 byte plaintext block will always produce the same 16 byte ciphertext.

    // common 16 byte plain text block - repeats commonly...
    let mut needle = ciphertexts
        .iter()
        .map(|text| {
            text
                .chunks(16)
                .fold(HashMap::new(), |mut acc, chunk| {
                    let chunk_entry = acc
                        .entry(chunk)
                        .or_insert(0);
                    *chunk_entry += 1;
                    acc
                })
        })
        .collect::<Vec<HashMap<_, _>>>();
    
    needle
        .sort_by(|a, b| {
            a.values().fold(0, |acc, entry| acc + entry).cmp(&b.values().fold(0, |acc, entry| acc + entry))
        });
        
    println!("Needle: {:?}", needle[0]);
}
