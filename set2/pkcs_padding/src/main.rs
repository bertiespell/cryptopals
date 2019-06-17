use pkcs_padding;

fn main() {
    let unpadded = "YELLOW SUBMARINE";
    let result = pkcs_padding::pad_to(&unpadded, 20);
    println!("Found result: {}", result);
}