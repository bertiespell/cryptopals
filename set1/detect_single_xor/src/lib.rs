use hex;
use single_byte_xor_cipher;

pub struct ScoredXorStrings {
    pub score: i32,
    pub encrypted_strings: String
}

impl ScoredXorStrings {
    fn new() -> ScoredXorStrings {
        ScoredXorStrings {
            score: 0,
            encrypted_strings: String::new()
        }
    }
}

pub fn detect_single_xor(data: String) -> ScoredXorStrings {
    data
        .lines()
        .map(|line| single_byte_xor_cipher::decrypt(hex::decode(line).unwrap()))
        .fold(ScoredXorStrings::new(), |acc, decrypted_line| {
            let score = single_byte_xor_cipher::score_xored_hashes(decrypted_line.decoded.as_bytes().to_vec());
            if score > acc.score {
                return ScoredXorStrings {
                    score,
                    encrypted_strings: decrypted_line.decoded
                };
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_detection() {
        let data = fs::read_to_string("encrypted_strings.txt").expect("Unable to read file");
        let decrypted = detect_single_xor(data);
        let actual = "Now that the party is jumping\n";
        assert_eq!(actual, &decrypted.encrypted_strings);
    }
}