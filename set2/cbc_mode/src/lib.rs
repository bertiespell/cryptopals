/**
Implement CBC mode
CBC mode is a block cipher mode that allows us to encrypt irregularly-sized messages, despite the fact that a block cipher natively only transforms individual blocks.

In CBC mode, each ciphertext block is added to the next plaintext block before the next call to the cipher core.

The first plaintext block, which has no associated previous ciphertext block, is added to a "fake 0th ciphertext block" called the initialization vector, or IV.

Implement CBC mode by hand by taking the ECB function you wrote earlier, making it encrypt instead of decrypt (verify this by decrypting whatever you encrypt to test), and using your XOR function from the previous exercise to combine them.

The file here is intelligible (somewhat) when CBC decrypted against "YELLOW SUBMARINE" with an IV of all ASCII 0 (\x00\x00\x00 &c)

Don't cheat.
Do not use OpenSSL's CBC code to do CBC mode, even to verify your results. What's the point of even doing this stuff if you aren't going to learn from it?
 */
use pkcs_padding;
use aes_in_ecb;
use single_byte_xor_cipher;
use decrypt_repeating::base_64;

pub fn decrypt_string(encrypted_text: &str, key: &str, iv: &str) -> String {

    let iv_lengthened = pkcs_padding::pad_with_bytes(iv.as_bytes().to_owned(), key.as_bytes().len(), 0 as u8);
    let decrypted_bytes = decrypt_bytes(
        encrypted_text.as_bytes().to_owned(), 
        key.as_bytes().to_owned(), 
        iv_lengthened
    );

    decrypted_bytes
        .iter()
        .fold(String::new(), |mut acc, entry| {
            acc.push_str(&String::from_utf8(entry.clone()).unwrap());
            acc
        })
}

pub fn decrypt_bytes(encrypted_text: Vec<u8>, key: Vec<u8>, iv: Vec<u8>) -> Vec<Vec<u8>> {
    let mut last_block = iv;

    let base_64_decoded = base_64::decode(&encrypted_text);

    let mut decrypted: Vec<Vec<u8>> = vec!();

    base_64_decoded
        .chunks(key.len()) // each block should be the length of the key
        .for_each(|entry| {
            let padded_entry = pkcs_padding::pad_to_bytes(entry.to_vec(), key.len()); // these are all now the length of the key

            let decrypted_block = aes_in_ecb::decrypt_data(
                padded_entry, 
                &key, 
                last_block.clone()
            ).unwrap();

            let xor_result = single_byte_xor_cipher::xor(
                decrypted_block.clone()[0..16].to_vec(), 
                last_block.clone()
            );
            decrypted.push(xor_result.clone());

            last_block = entry.to_vec();
        })
    ;

    decrypted
        .into_iter()
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let encypted_text = fs::read_to_string("encrypted_data.txt").expect("Unable to read file");

    }
}
