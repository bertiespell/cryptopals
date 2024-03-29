extern crate hex;
use std::collections::HashMap;

pub struct DecodedResult {
    pub score: i32,
    pub decoded: String,
    pub key: char,
}

impl DecodedResult {
    fn new_from_defaults() -> DecodedResult {
        DecodedResult {
            score: 0, 
            decoded: String::new(), 
            key: 'a'
        }
    }
}

pub fn decrypt(decoded_hex_string: Vec<u8>) -> DecodedResult {
    let xored_hashes = xor_against_chars(decoded_hex_string);

    find_best(xored_hashes)
}

pub fn find_best(xored_hashes: HashMap<u8, Vec<u8>>) -> DecodedResult {
    xored_hashes.iter().fold(DecodedResult::new_from_defaults(), |acc, hash| {
        let decoded_string = String::from_utf8(hash.1.clone()).unwrap_or(String::from("a"));
        let score = score_xored_hashes(decoded_string.as_bytes().to_vec());
        if score > acc.score {
            return DecodedResult {
                score, 
                decoded: decoded_string,
                key: *hash.0 as char
            };
        }
        acc
    })
}

pub fn xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

fn xor_against_chars(decoded_hex: Vec<u8>) -> HashMap<u8, Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let hex_encoded_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let decode_hex = hex::decode(hex_encoded_string).unwrap();
        let decrypted_string = decrypt(decode_hex);
        let actual = DecodedResult {
            score: 577,
            decoded: String::from("Cooking MC\'s like a pound of bacon"),
            key: 'X'
        };

        assert_eq!(decrypted_string.score, actual.score);
        assert_eq!(decrypted_string.decoded, actual.decoded);
        assert_eq!(decrypted_string.key, actual.key);
    }
}