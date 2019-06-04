extern crate convert_hex_to_base64;

use convert_hex_to_base64::decode_string;
use convert_hex_to_base64::split_and_strip_whitespace;

fn main() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    // let actual = "746865206b696420646f6e277420706c6179";
    xor(original_string, xor_string);
}

pub fn xor<'a>(string: &'a str, xor_against: &'a str) -> &'a str {
    // Plain message
    // 1001
    // Key (repeated)
    // 1010
    // Encrypted message
    // 0011
    let decoded_string = decode_string(string);
    let decoded_xor = decode_string(xor_against);
    // turn these into binary
    let binary_number = decoded_string
        .as_bytes()
        .iter()
        .map(|x| format!("{:b}", x))
        .map(|x| format!("{:0>#8}", x))
        .fold(String::new(), |acc, x| acc + &x.to_owned())
        .chars()
        .collect::<Vec<char>>();

    let binary_xor_number = decoded_xor
        .as_bytes()
        .iter()
        .map(|x| format!("{:b}", x))
        .map(|x| format!("{:0>#8}", x))
        .fold(String::new(), |acc, x| acc + &x.to_owned())
        .chars()
        .collect::<Vec<char>>();

    // now compare each - if the values are different - return '1' else return '0'

    println!("{:?}, {:?}", binary_number, binary_xor_number);
    string
}

pub fn string_to_byters_str() {

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