// The Base64-encoded content in text.txt has been encrypted via AES-128 in ECB mode under the key "YELLOW SUBMARINE"

// Symmetric key algorithm

// Modern CPUs have AES instructions built into them! (These are much faster and more secure)

// If you implement your own - people can look up CPU caches!

// a block cipher - encrypts 128 bits / 16 bytes

// treats 16 bytes as 4x4 grid

// initialisation steps - key expansions then rounds

// we can have different sized keys - 128 (10 cycles), 192 (12 cycles) or 256 bits (14 cycles)

// Algorithm
/**
 * Message -> Key Expansion -> Add Round Key (xor)
 * Rounds => SubBytes -> Shift Rows -> Mix Columns -> Add Round Key REPEAT
 * Final Round => SubBytes -> Shift Rows -> Add Round Key
 */

extern crate decrypt_repeating;
use std::fs;
use decrypt_repeating::base_64;
extern crate openssl;
use openssl::symm::{encrypt, decrypt, Cipher};

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let text_bytes = base_64::decode(text.as_bytes());
    let decrypted = decrypt_data(text_bytes);
    let final_text = String::from_utf8(decrypted.unwrap()).unwrap();
    println!("Decrypted text: {:?}", final_text);
}

fn decrypt_data(data: Vec<u8>) -> Result<Vec<u8>, String> {
    let cipher = Cipher::aes_128_ecb();
    let key = b"YELLOW SUBMARINE";
    let iv = b"ABCDEFGHIJKLMNOP";
    // let ciphertext = encrypt(
    //     cipher,
    //     key,
    //     Some(iv),
    //     &data
    // ).unwrap();
    
    let decrypted_text = decrypt(
        cipher,
        key,
        Some(iv),
        &data
    );
    match decrypted_text {
        Ok(text) => Ok(text),
        Err(_) => Err(String::from("Error decrypting text")),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decryption() {
        let text = fs::read_to_string("text.txt").expect("Unable to read file");
        let text_bytes = base_64::decode(text.as_bytes());
        let decrypted = decrypt_data(text_bytes);
        let result = String::from_utf8(decrypted.unwrap()).unwrap();

        let actual = fs::read_to_string("decrypted.txt").expect("Unable to read file");
        assert_eq!(result, actual);
    }
}
