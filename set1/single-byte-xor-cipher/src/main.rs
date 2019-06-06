/**
 * Single-byte XOR cipher
The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
 */
extern crate xor;
extern crate convert_hex_to_base64;
extern crate hex;

mod cipher;

use xor::xor;
use convert_hex_to_base64::split_and_strip_whitespace;

fn main() {
    let hex_encoded_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    cipher::cipher::decrypt(&hex_encoded_string);
    let frequency = String::from("ETAOIN SHRDLUetaoinshrdl");
    // println!("OK");
    let mut encoded_vecs: Vec<Vec<u8>> = vec!(); // each one letter...
    for letter in split_and_strip_whitespace(&frequency).into_iter() {
        let other = xor(hex_encoded_string, &hex::encode(&letter));
        let u8_decoded = hex::decode(other).unwrap();
        // println!("Other: {:?}", u8_decoded);
        encoded_vecs.push(u8_decoded);
        // println!("{:?}", String::from_utf8(other).unwrap()); // this is a vec of U8s..
    }

    let xor_results = encoded_vecs.iter().map(|x| xor_score(x.clone())).collect::<Vec<i32>>();
    // second one has highest indec
    // println!("XOR RESULTS: {:?}", xor_results);
    let found_result = encoded_vecs[7].clone();
    // println!("Found result: {:?}", found_result);
    // encoded_vecs.into_iter().map(|x| println!("{:?}", String::from_utf8(x).unwrap())).collect::<Vec<_>>();
    // println!("{:?}", String::from_utf8(found_result).unwrap());
}

// utf8 encodes in 1 - 4 bytes, depending on the number of significant bits
// one byte 0xxxxxxx
// two bytes 110xxxxx 10xxxxxx
// three bytes 1110xxxx 10xxxxxx 10xxxxxx
// four bytes 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

// The first 128 characters (US-ASCII) need one byte.

fn better_xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

fn xor_score(input: Vec<u8>) -> i32 {
  input.iter().fold(0, | score, &data_char | {
    score + get_char_score(data_char as char)
  })
}

fn get_char_score(input_char: char) -> i32 {
  match input_char {
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