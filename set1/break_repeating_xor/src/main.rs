extern crate hex;
extern crate single_byte_xor_cipher;
use single_byte_xor_cipher::cipher::cipher::decrypt;
use std::fs;
pub mod base_64;

    // println!("{:?}", x);
fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let decoded = base_64::decode(text.as_bytes());

    let decoded_text = hex::decode(&decoded);
    println!("{:?}", decoded);
    println!("{:?}", decoded_text);

    // We use the .atob() function (MDN docs) to decode the base64 string, then split the array into single characters, convert each character into its ASCII code (a base-10 decimal value) with .charCodeAt(), and finally convert each ASCII code to binary.

    let mut sized_keys = break_repeating_xor::find_smallest_key(decoded.clone());

    sized_keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // println!("SMALLEST {:?}", sized_keys);

    let keys: Vec<Vec<_>> = sized_keys[0..3]
        .iter()
        .map(|x| break_repeating_xor::chunk_text_into_bytes(decoded.clone(), x.1 as i32))
        .map(|x| break_repeating_xor::transpose_text(x))
        .map(|transposed_text| {
            transposed_text
                .iter()
                .map(|block| {
                    let decoded_hex = hex::encode(block); // really??
                    println!("decoded_hex!!!! {:?}", decoded_hex);
                    let mapped = block
                        .iter()
                        .map(|byte| {
                            // println!("FOUND A BYTE : {:?}", byte);
                            // let string = String::from_utf8(byte);
                            *byte as char
                        })
                        .collect::<Vec<_>>();

                    // println!("MAPPED : {:?}", mapped);
                    let string = String::from_utf8(block.clone()); // What should I decode here?
                    // For each block, 
                    // the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. 
                    // Put them together and you have the key.
                    // match decoded_hex {
                    //     Ok(t) => {
                            // println!("TRYING TO DECRYPT {:?}", t);
                            let decrypted = decrypt(&decoded_hex); // decoded hex?
                            match decrypted {
                                Ok(decrypted) => decrypted,
                                Err(_) => (0, String::from("BROKE"), '-')
                        //     }
                        // },
                        // Err(_) => (0, String::from("NONE"), '-')
                    }
                    // decrypt(&String::from_utf8(block.clone()).unwrap())
                })
                .collect()
        })
        .collect();

    let key: Vec<char> = vec!();
    println!("Keys: {:?}", keys);
    keys.iter().map(|key| {
        key.iter().map(|byte| {
            println!("BYTE   {:?}", byte);
            // key.push(byte.2);
        }).collect::<Vec<_>>();
    }).collect::<Vec<_>>();
    println!("KEY:: {:?}", key);
    // println!("FINAL: {:?}", decrypt(key.));
    // put all these together and then decrypt
}

pub mod break_repeating_xor {
    use single_byte_xor_cipher::cipher::cipher::xor;
    use std::collections::HashMap;

    pub fn transpose_text(text: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
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

    pub fn chunk_text_into_bytes(text: Vec<u8>, length: i32) -> Vec<Vec<u8>> {
        text
            .chunks(length as usize)
            .map(|chunk| chunk.into())
            .collect::<Vec<Vec<u8>>>()
    }

    pub fn find_smallest_key(text: Vec<u8>) -> Vec<(i32, f32)> { // keysize, normalised edit
        generate_keysizes()
            .iter()
            .map(|keysize| {
                let bytes = get_first_two_keysizes_of_bytes(text.clone(), *keysize);
                let hamming_distance = calculate_hamming_distance(bytes.0, bytes.1);
                let normalised_hamming_distance = normalise_distance(hamming_distance as f32, *keysize as f32);
                (*keysize, normalised_hamming_distance)
            })
            .collect::<Vec<(i32, f32)>>()
    }

    pub fn calculate_hamming_distance(string1_bytes: Vec<u8>, string2_bytes: Vec<u8>) -> i32 {
        let xored_bytes = xor(string1_bytes, string2_bytes);
        xored_bytes.iter().fold(0, |acc, x| acc + x.count_ones()) as i32
    }

    fn generate_keysizes() -> Vec<i32> {
        (2..40).collect()
    }

    fn get_first_two_keysizes_of_bytes(text: Vec<u8>, keysize: i32) -> (Vec<u8>, Vec<u8>) {
        let chunked_text = chunk_text_into_bytes(text, keysize).clone();
        (chunked_text[0].to_vec(), chunked_text[1].to_vec())
    }

    fn normalise_distance(edit_distance: f32, keysize: f32) -> f32 {
        edit_distance / keysize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_xor() {
        let string1 = "this is a test".as_bytes().to_vec();
        let string2 = "wokka wokka!!!".as_bytes().to_vec();

        assert_eq!(
            break_repeating_xor::calculate_hamming_distance(string1, string2),
            37
        );
    }

    #[test]
    fn test_transpose() {
        let transpose_me = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let actual = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(break_repeating_xor::transpose_text(transpose_me), actual);
    }
}
