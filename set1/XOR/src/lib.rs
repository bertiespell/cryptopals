fn main() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    // let actual = "746865206b696420646f6e277420706c6179";
    xor(original_string, xor_string);
}

pub fn xor<'a>(string: &'a str, xor_against: &'a str) -> String {
    let mut xored: Vec<u8> = vec!();
    let hex_iterator = hex::decode(string).unwrap().into_iter().enumerate();
    for (index, byte) in hex_iterator {
        xored.push(byte^hex::decode(xor_against).unwrap()[index]);
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