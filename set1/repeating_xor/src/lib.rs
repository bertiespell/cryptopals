/**
Implement repeating-key XOR
Here is the opening stanza of an important work of the English language:

Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal
Encrypt it, under the key "ICE", using repeating-key XOR.

In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.

It should come out to:

0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f

0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272
a282b2f204363656963656963650063222663263b223f30633221262b690a652126243b632469203c24212425


Encrypt a bunch of stuff using your repeating-key XOR function. Encrypt your mail. Encrypt your password file. Your .sig file. Get a feel for it. I promise, we aren't wasting your time with this.
 */
use single_byte_xor_cipher;
use hex;

pub fn repeated_xor<'a>(original_string: &'a str, xor_encryption: &'a str) -> String {
    let bytes_string = original_string.as_bytes().to_vec();
    let mut xor_vec: Vec<u8> = vec!();
    while xor_vec.len() < bytes_string.len() {
        xor_encryption
            .to_owned()
            .into_bytes()
            .into_iter()
            .for_each(|bytes| xor_vec.push(bytes));
    }
    if xor_vec.len() != bytes_string.len() {
        xor_vec.resize(bytes_string.len(), b'H');
    }
    let encrypted = single_byte_xor_cipher::xor(bytes_string, xor_vec);
    hex::encode(encrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let original_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let xor_encryption = "ICE";
        let actual = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

        assert_eq!(repeated_xor(original_string, xor_encryption), actual);
    }
}