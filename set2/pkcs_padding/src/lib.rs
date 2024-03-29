/**
Implement PKCS#7 padding
A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. But we almost never want to transform a single block; we encrypt irregularly-sized messages.

One way we account for irregularly-sized messages is by padding, creating a plaintext that is an even multiple of the blocksize. The most popular padding scheme is called PKCS#7.

So: pad any block to a specific block length, by appending the number of bytes of padding to the end of the block. For instance,

"YELLOW SUBMARINE"
... padded to 20 bytes would be:

"YELLOW SUBMARINE\x04\x04\x04\x04"
*/

pub fn pad_to_str(block: &str, proposed_block_length: usize) -> String {
    let block_length = block.as_bytes().len();
    let padded = pad_to_bytes(block.as_bytes().to_owned(), proposed_block_length);
    String::from_utf8(padded).unwrap()
}

pub fn pad_to_bytes(block: Vec<u8>, proposed_block_length: usize) -> Vec<u8> {
    let v = proposed_block_length - block.len();
    pad_with_bytes(block, proposed_block_length, v as u8)
}

pub fn pad_with_str(block: &str, proposed_block_length: usize, pad_with: u8) -> String  {
    let padded = pad_with_bytes(block.as_bytes().to_owned(), proposed_block_length, pad_with);
    String::from_utf8(padded).unwrap()
}

pub fn pad_with_bytes(block: Vec<u8>, proposed_block_length: usize, pad_with: u8) -> Vec<u8> {
    assert!(proposed_block_length >= block.len());
    let mut padded = block.clone();
    while (padded.len()) < proposed_block_length {
        padded.push(pad_with); 
    }
    padded
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_padding_string() {
        let unpadded = "YELLOW SUBMARINE";
        let actual = "YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = pad_to_str(&unpadded, 20);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_padding_with() {
        let unpadded = "YELLOW SUBMARINE";
        let actual = "YELLOW SUBMARINE\x00\x00\x00\x00\x00";
        let result = pad_with_str(&unpadded, 21, 0 as u8);
        assert_eq!(result, actual);
    }

    #[test]
    fn test_padding_with_bytes() {
        let unpadded = "YELLOW SUBMARINE";
        let actual = "YELLOW SUBMARINE\x00\x00\x00\x00\x00";
        let padded = pad_with_bytes(unpadded.as_bytes().to_owned(), 21, 0 as u8);
        let result = String::from_utf8(padded).unwrap();
        assert_eq!(result, actual);
    }

    #[test]
    fn text_byte_padding() {
        let unpadded = "YELLOW SUBMARINE";
        let actual = "YELLOW SUBMARINE\x04\x04\x04\x04";
        let padded = pad_to_bytes(unpadded.as_bytes().to_owned(), 20);
        let result = String::from_utf8(padded).unwrap();
        assert_eq!(result, actual);
    }


}

