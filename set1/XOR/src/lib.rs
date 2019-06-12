fn main() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    xor(original_string, xor_string);
}

pub fn xor<'a>(string: &'a str, xor_against: &'a str) -> String {
    let mut xored: Vec<u8> = vec!();
    let hex_iterator = hex::decode(string).unwrap().into_iter().enumerate();
    
    // make them the same length
    let mut repeated_xor_against = String::from(xor_against);
    while repeated_xor_against.len() < string.len() {
        repeated_xor_against.push_str(xor_against);
    }

    // TODO: this works for a single charachter input, but otherwise this might made a key (repeated_xor_against) which is too long. We need to trim this to be the original length (but Rust currently complains about not knowing the size at compile-time - probably need to use a Box)

    let xor_decoded = hex::decode(repeated_xor_against).unwrap(); 

    for (index, byte) in hex_iterator { // these can be zipped up together and then iterated on
        xored.push(byte^xor_decoded[index]);
    }
    hex::encode(String::from_utf8(xored).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let original_string = "1c0111001f010100061a024b53535009181c";
        let xor_string = "686974207468652062756c6c277320657965";
        let actual = "746865206b696420646f6e277420706c6179";

        assert_eq!(xor(original_string, xor_string), actual);
    }
}