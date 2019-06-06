extern crate single_byte_xor_cipher;
extern crate hex;

fn main() {
    println!("Hello, world!");
}

mod break_repeating_xor {
    use single_byte_xor_cipher::cipher::cipher::xor;

    pub fn calculate_hamming_distance(string1: &str, string2: &str) -> u32 {
        let xored_bytes = xor(
            string1.as_bytes().to_vec(), 
            string2.as_bytes().to_vec()
        );
        xored_bytes
            .iter()
            .fold(0, |acc, x| acc + x.count_ones())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_xor() {
        let string1 = "this is a test";
        let string2 = "wokka wokka!!!";

        assert_eq!(break_repeating_xor::calculate_hamming_distance(string1, string2), 37);
    }
}