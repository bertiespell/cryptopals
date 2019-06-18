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
use openssl::symm::{ encrypt, Cipher, Crypter, Mode};

pub fn decrypt_data(data: Vec<u8>, key: &[u8], iv: Vec<u8>) -> Result<Vec<u8>, String> {
    let cipher = Cipher::aes_128_ecb();

    let mut decryted = Crypter::new(
        cipher,
        Mode::Decrypt,
        key,
        Some(&iv),
    ).unwrap();

    let mut output = vec![0 as u8; data.len() + Cipher::aes_128_cbc().block_size()];
    let decrypted_result = decryted.update(&data, &mut output);

    match decrypted_result {
        Ok(_) => Ok(output),
        Err(e) => Err(format!("Error decrypting text: {}", e)),
    }
}

pub fn encrypt_data(data: Vec<u8>, key: &[u8], iv: Vec<u8>) -> Result<Vec<u8>, String>  {
    let cipher = Cipher::aes_128_ecb();
    
    let decrypted_text = encrypt(
        cipher,
        key,
        Some(&iv),
        &data
    );
    match decrypted_text {
        Ok(text) => Ok(text),
        Err(_) => Err(String::from("Error encrypting text")),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use decrypt_repeating::base_64;

    #[test]
    fn test_decryption() {
        let text = fs::read_to_string("text.txt").expect("Unable to read file");
        let iv = b"ABCDEFGHIJKLMNOP";
        let text_bytes = base_64::decode(text.as_bytes());
        let key = b"YELLOW SUBMARINE";
        let decrypted = decrypt_data(text_bytes, key, iv.to_vec());
        let result = String::from_utf8(decrypted.unwrap()).unwrap();

        let actual = fs::read_to_string("decrypted.txt").expect("Unable to read file");
        assert_eq!(result, actual);
    }

    #[test]
    fn test_encryption() {
        let encypted_text_original = fs::read_to_string("text.txt").expect("Unable to read file").replace("\n", "");
        let un_encrypted_text = fs::read_to_string("decrypted.txt").expect("Unable to read file");
        let key = b"YELLOW SUBMARINE";
        let iv = b"ABCDEFGHIJKLMNOP";
        let encrypted = encrypt_data(un_encrypted_text.as_bytes().to_owned(), key, iv.to_vec());
        let base_64_encoded = base_64::encode(&encrypted.unwrap());
        let result = String::from_utf8(base_64_encoded).unwrap();

        assert_eq!(result, encypted_text_original);
    }
}
