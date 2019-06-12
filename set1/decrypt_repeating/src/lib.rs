extern crate single_byte_xor_cipher;
pub mod base_64;
extern crate xor;

use xor::xor as repeating_xor;

use std::fs;
use single_byte_xor_cipher::cipher::cipher::decrypt;

pub fn decrypt_repeating_xor(decoded: Vec<u8>) -> String {
    let mut normalised_keys = (2..40)
        .collect::<Vec<i32>>()
        .into_iter()
        .map(|keysize| {
            // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
            let chunked_text = chunk_text_into_bytes(decoded.clone(), keysize);

            let mut hamming_distance = 0.0;
            chunked_text
                .iter()
                .enumerate()
                .for_each(|(index, chunk)| {
                    if index + 1 < chunked_text.len() {
                        hamming_distance += calculate_hamming_distance(chunk.clone(), chunked_text[index+1].clone());
                    }
                });

            let normalised = hamming_distance / (keysize as f32  * chunked_text.len() as f32);

            (keysize, normalised)
        })
        .collect::<Vec<(i32, f32)>>();

    normalised_keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances

    // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
    let chunked_text = chunk_text_into_bytes(decoded.clone(), normalised_keys[0].0);

    // Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.

    let transposed_text = transpose_text(chunked_text);

    // Solve each block as if it was single-character XOR. You already have code to do this.

    let decrypted = transposed_text
        .iter()
        .map(|block| {
            // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
            decrypt(block.clone())
        })
        .map(|histogram| {
            histogram.2 as u8
        })
        .collect::<Vec<u8>>();

    let decrypted_byte_text = repeating_xor(&decoded, &decrypted.clone()); // TODO: implement my own repeating XOR
    let final_text = String::from_utf8(decrypted_byte_text).unwrap();

    println!("Final text {:?}", final_text);
    final_text
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
    // TODO: this is brittle - should consider testing more edge cases
    let mut transposed_text: Vec<Vec<u8>> =
        vec![vec![u8::min_value(); text.len()]; text[0].len()];
    
    for (text_index, vec) in text.iter().enumerate() {
        for (index, entry) in vec.iter().enumerate() {
            transposed_text[index][text_index] = *entry;
        }
    }
    transposed_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_xor() {
        let string1 = "this is a test".as_bytes().to_vec();
        let string2 = "wokka wokka!!!".as_bytes().to_vec();

        assert_eq!(
            calculate_hamming_distance(string1, string2),
            37.0
        );
    }

    #[test]
    fn test_transpose() {
        let transpose_me = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let actual = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(transpose_text(transpose_me), actual);
    }

    #[test]
    fn test_decryption() {
        let text = fs::read_to_string("text.txt").expect("Unable to read file");
        let text_bytes = base_64::decode(text.as_bytes());
        let result = decrypt_repeating_xor(text_bytes);
        let actual = fs::read_to_string("decrypted.txt").expect("Unable to read file");;
        assert_eq!(result, actual);
    }
}
