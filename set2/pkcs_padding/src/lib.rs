/**
Implement PKCS#7 padding
A block cipher transforms a fixed-sized block (usually 8 or 16 bytes) of plaintext into ciphertext. But we almost never want to transform a single block; we encrypt irregularly-sized messages.

One way we account for irregularly-sized messages is by padding, creating a plaintext that is an even multiple of the blocksize. The most popular padding scheme is called PKCS#7.

So: pad any block to a specific block length, by appending the number of bytes of padding to the end of the block. For instance,

"YELLOW SUBMARINE"
... padded to 20 bytes would be:

"YELLOW SUBMARINE\x04\x04\x04\x04"
*/

pub fn pad_to(block: &str, proposed_block_length: usize) -> String {
    let block_length = block.as_bytes().len();
    assert!(proposed_block_length > block_length);
    let v = proposed_block_length - block_length;
    let mut padded_string = String::from(block).as_bytes().to_owned();
    while (padded_string.len()) < proposed_block_length {
        padded_string.push(v as u8); // coercion here is very useful!
    }
    String::from_utf8(padded_string).unwrap()
}

pub fn pad_with(block: &str, proposed_block_length: usize, pad_with: u8) -> String {
    let block_length = block.as_bytes().len();
    assert!(proposed_block_length > block_length);
    let mut padded_string = String::from(block).as_bytes().to_owned();
    while (padded_string.len()) < proposed_block_length {
        padded_string.push(pad_with as u8); 
    }
    String::from_utf8(padded_string).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_padding() {
        let unpadded = "YELLOW SUBMARINE";
        let actual = "YELLOW SUBMARINE\x04\x04\x04\x04";
        let result = pad_to(&unpadded, 20);
        assert_eq!(result, actual);
    }
}

