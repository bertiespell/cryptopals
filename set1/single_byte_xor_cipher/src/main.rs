/**
 * Single-byte XOR cipher
The hex encoded string:

1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
... has been XOR'd against a single character. Find the key, decrypt the message.

You can do this by hand. But don't: write code to do it for you.

How? Devise some method for "scoring" a piece of English plaintext. Character frequency is a good metric. Evaluate each output and choose the one with the best score.
 */
use single_byte_xor_cipher;

fn main() {

    let hex_encoded_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let decode_hex = hex::decode(hex_encoded_string).unwrap();

    let decrypted_string = single_byte_xor_cipher::decrypt(decode_hex);
    println!("Decoded string: {:?}", decrypted_string.decoded);
}