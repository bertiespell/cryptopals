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
use std::fs;
use pkcs_padding;

pub fn decrypt(encrypted_text: Vec<u8>, key: Vec<u8>, IV: Vec<u8>) -> Vec<u8> {
    // Implement CBC mode by hand by taking the ECB function you wrote earlier, making it encrypt instead of decrypt (verify this by decrypting whatever you encrypt to test), and using your XOR function from the previous exercise to combine them.

    // Decrypt first block
    // XOR this block against IV
    // Decrypt next block (XOR with previous ciphertext)
    let mut last_block = IV;

    encrypted_text
        .chunks(key.len()) // each block should be the length of the key
        .for_each(|entry| {
            let padded_entry = pkcs_padding::pad_to_bytes(entry.to_vec(), key.len()); // these are all now the length of the key
            // decrypt block
            // xor against last_block
            // update last block (last_block = entry;)
            dbg!(padded_entry.len());
        })
    ;
    vec!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let encypted_text = fs::read_to_string("encrypted_data.txt").expect("Unable to read file");

    }
}
