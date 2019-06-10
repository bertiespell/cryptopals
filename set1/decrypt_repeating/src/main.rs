pub mod base_64;
use std::fs;
extern crate single_byte_xor_cipher;

use single_byte_xor_cipher::cipher::cipher::decrypt;

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let decoded = base_64::decode(text.as_bytes());
    let mut normalised_keys = (2..40)
        .collect::<Vec<i32>>()
        .into_iter()
        .map(|keysize| {
            // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
            let chunked_text = chunk_text_into_bytes(decoded.clone(), keysize);
            let hamming_dist = calculate_hamming_distance(chunked_text[0].clone(), chunked_text[1].clone());
            let normalised = hamming_dist / keysize as f32;
            (keysize, normalised)
        })
        .collect::<Vec<(i32, f32)>>();

    normalised_keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances
    // let probable_key = normalised_keys[15];

    let decrypted = normalised_keys // TODO: need to figure out why this sizing is wrong
        .iter()
        .map(|probable_key| {
            // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
            let chunked_text = chunk_text_into_bytes(decoded.clone(), probable_key.0);

            // Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.

            let transposed_text = transpose_text(chunked_text);

            // Solve each block as if it was single-character XOR. You already have code to do this.

            transposed_text
                .iter()
                .map(|block| {
                    // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
                    decrypt(block.clone())
                })
                .map(|histogram| {
                    histogram.2
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<_>>();
    
    decrypted.iter().for_each(|x| println!("SORTED {:?}", x));
    println!("decrypted {:?}", decrypted.len());

    // decrypt text with key
}

fn calculate_hamming_distance(string1_bytes: Vec<u8>, string2_bytes: Vec<u8>) -> f32 {
    let xored_bytes = xor(string1_bytes, string2_bytes);
    xored_bytes.iter().fold(0, |acc, x| acc + x.count_ones()) as f32
}

fn xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

fn chunk_text_into_bytes(text: Vec<u8>, length: i32) -> Vec<Vec<u8>> {
    text
        .chunks(length as usize)
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<u8>>>()
}

fn transpose_text(text: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // TODO: this probably won't work - the length of the inner vec isn't right, other than for square matrices (currenly using the length of the first vec - hoping the cryptopals people have been kind with their examples...)
    let mut transposed_text: Vec<Vec<u8>> =
        vec![vec![u8::min_value(); text.len()]; text[0].len()];
    
    for (text_index, vec) in text.iter().enumerate() {
        for (index, entry) in vec.iter().enumerate() {
            transposed_text[index][text_index] = *entry;
        }
    }
    transposed_text
}