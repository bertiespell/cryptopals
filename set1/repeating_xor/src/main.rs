use repeating_xor;
use hex;

fn main() {
    let original_string = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let xor_encryption = "ICE";
    let encrypted = repeating_xor::repeated_xor(original_string, xor_encryption);
    println!("Encyrpted key: {:?}", hex::encode(encrypted));
}