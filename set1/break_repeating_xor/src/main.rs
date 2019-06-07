extern crate single_byte_xor_cipher;
extern crate hex;
use std::fs;

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");

    let keysize = break_repeating_xor::find_smallest_key(&text);

    println!("Keysize found: {:?}", keysize);
}

mod break_repeating_xor {
    use single_byte_xor_cipher::cipher::cipher::xor;

    pub fn find_smallest_key(text: &str) -> (i32, i32) { // EDIT: actually want to capture smallest 3 values
        generate_keysizes()
            .iter()
            .fold((900, 0), |acc, key| { // hamming distance, then keysize // use hashmap?
                let bytes = get_first_two_keysizes_of_bytes(text, *key);
                let hamming_distance = calculate_hamming_distance(bytes.0, bytes.1);
                let normalised_hamming_distance = normalise_distance(hamming_distance, *key);
                println!("Keysize testing: {:?}, Hamming distance calculated at: {:?}, normalised to {:?}", *key, hamming_distance, normalised_hamming_distance);
                if (hamming_distance) < acc.0 {
                    return (hamming_distance, *key);
                }
                acc
            })
    }

    pub fn calculate_hamming_distance(string1_bytes: Vec<u8>, string2_bytes: Vec<u8>) -> i32 {
        let xored_bytes = xor(
            string1_bytes, 
            string2_bytes
        );
        xored_bytes
            .iter()
            .fold(0, |acc, x| acc + x.count_ones()) as i32
    }

    fn generate_keysizes() -> Vec<i32> {
        (2..40).collect()
    }

    fn get_first_two_keysizes_of_bytes(text: &str, keysize: i32) -> (Vec<u8>, Vec<u8>) {
        let mut result: (Vec<u8>, Vec<u8>) = (vec!(), vec!());
        text
            .as_bytes()
            .to_vec()
            .chunks(keysize as usize)
            .enumerate()
            .for_each(|(index, chunk)| {
                if index == 0 {
                    result.0 = chunk.to_vec();
                } else if index == 1 {
                    result.1 = chunk.to_vec();
                }
            });
        result
    }

    fn normalise_distance(edit_distance: i32, keysize: i32) -> i32 {
        edit_distance/keysize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_xor() {
        let string1 = "this is a test";
        let string2 = "wokka wokka!!!";

        // assert_eq!(break_repeating_xor::calculate_hamming_distance(string1, string2), 37);
    }
}