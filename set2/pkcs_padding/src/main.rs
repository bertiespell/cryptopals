use pkcs_padding;

fn main() {
    let unpadded = "YELLOW SUBMARINE";
    let result = pkcs_padding::pad_to_str(&unpadded, 20);
    println!("Found result: {}", result);
}