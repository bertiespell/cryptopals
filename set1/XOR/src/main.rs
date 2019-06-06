use xor::xor;

fn main() {
    let original_string = "1c0111001f010100061a024b53535009181c";
    let xor_string = "686974207468652062756c6c277320657965";
    // let actual = "746865206b696420646f6e277420706c6179";
    xor(original_string, xor_string);
}