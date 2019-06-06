extern crate hex;

pub mod cipher {
    use std::collections::HashMap;

    pub fn decrypt(hex_string: &str) {
        let decoded_hex = hex::decode(hex_string).unwrap();
        let xored_hashes = xor_against_chars(decoded_hex);

        let mut best_scored_value = (0, String::new());

        for (key, value) in xored_hashes.iter() {
            let decoded_string = String::from_utf8(value.clone()).unwrap();
            let score = score_xored_hashes(decoded_string.as_bytes().to_vec());
            if score > best_scored_value.0 {
                best_scored_value = (score, decoded_string);
            }
        }

        println!("Decoded string: {:?}", best_scored_value);
    }

    pub fn xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
        input
            .iter()
            .zip(key.iter())
            .map(|(x, y)| x ^ y)
            .collect()
    }

    pub fn xor_against_chars(decoded_hex: Vec<u8>) -> HashMap<u8, Vec<u8>> {
        let mut results = HashMap::new();

        for letter in 0..128 {
            let xored_result = xor(decoded_hex.clone(), vec![letter; decoded_hex.len()]);
            results.insert(letter, xored_result);
        }
        
        results
    }

    pub fn score_xored_hashes(xored_hashes: Vec<u8>) -> i32 {
        xored_hashes.iter().fold(0, |acc, &entry| acc + score_char(entry as char))
    }

    pub fn score_char(test_char: char) -> i32 {
        match test_char {
            'e' => 27,
            't' => 26,
            ' ' => 25,
            'a' => 24,
            'o' => 23,
            'n' => 22,
            'r' => 21,
            'i' => 20,
            's' => 19,
            'h' => 18,
            'd' => 17,
            'l' => 16,
            'f' => 15,
            'c' => 14,
            'm' => 13,
            'u' => 12,
            'g' => 11,
            'y' => 10,
            'p' => 9,
            'b' => 8,
            'v' => 6,
            'k' => 5,
            'j' => 4,
            'x' => 3,
            'q' => 2,
            'z' => 1,
            '\x00'...'\x19' => -10,
            _ => 0
        }
    }
}
