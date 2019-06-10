extern crate single_byte_xor_cipher;
pub mod base_64;
extern crate xor;

use xor::xor as repeating_xor;

use std::fs;
use single_byte_xor_cipher::cipher::cipher::decrypt;

fn main() {
    let text = fs::read_to_string("text.txt").expect("Unable to read file");
    let text_bytes = base_64::decode(text.as_bytes());
    decrypt_repeating_xor(text_bytes);
}

fn decrypt_repeating_xor(decoded: Vec<u8>) -> String {
    let mut normalised_keys = (2..40)
        .collect::<Vec<i32>>()
        .into_iter()
        .map(|keysize| {
            // For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.
            let chunked_text = chunk_text_into_bytes(decoded.clone(), keysize);
            let hamming_dist = calculate_hamming_distance(chunked_text[0].clone(), chunked_text[1].clone());
            let normalised = hamming_dist / keysize as f32;
            (keysize, normalised)
        })
        .collect::<Vec<(i32, f32)>>();

    normalised_keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // The KEYSIZE with the smallest normalized edit distance is probably the key. You could proceed perhaps with the smallest 2-3 KEYSIZE values. Or take 4 KEYSIZE blocks instead of 2 and average the distances

    let decrypted = normalised_keys // TODO: need to figure out why this sizing is wrong (below we use decrypted[17] - should be able to use the first one or two entries here)
        .iter()
        .map(|probable_key| {
            // Now that you probably know the KEYSIZE: break the ciphertext into blocks of KEYSIZE length.
            let chunked_text = chunk_text_into_bytes(decoded.clone(), probable_key.0);

            // Now transpose the blocks: make a block that is the first byte of every block, and a block that is the second byte of every block, and so on.

            let transposed_text = transpose_text(chunked_text);

            // Solve each block as if it was single-character XOR. You already have code to do this.

            transposed_text
                .iter()
                .map(|block| {
                    // For each block, the single-byte XOR key that produces the best looking histogram is the repeating-key XOR key byte for that block. Put them together and you have the key.
                    decrypt(block.clone())
                })
                .map(|histogram| {
                    histogram.2 as u8
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();
    
    let repeating_key = decrypted[17].clone(); // TODO: see todo above

    let decrypted_byte_text = repeating_xor(&decoded, &decrypted[17].clone()); // TODO: implement my own repeating XOR
    let final_text = String::from_utf8(decrypted_byte_text).unwrap();

    println!("Final text {:?}", final_text);
    final_text
}

fn calculate_hamming_distance(string1_bytes: Vec<u8>, string2_bytes: Vec<u8>) -> f32 {
    let xored_bytes = xor(string1_bytes, string2_bytes);
    xored_bytes.iter().fold(0, |acc, x| acc + x.count_ones()) as f32
}

fn xor(input: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .zip(key.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

fn chunk_text_into_bytes(text: Vec<u8>, length: i32) -> Vec<Vec<u8>> {
    text
        .chunks(length as usize)
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<u8>>>()
}

fn transpose_text(text: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    // TODO: this probably won't work - the length of the inner vec isn't right, other than for square matrices (currenly using the length of the first vec - hoping the cryptopals people have been kind with their examples...)
    let mut transposed_text: Vec<Vec<u8>> =
        vec![vec![u8::min_value(); text.len()]; text[0].len()];
    
    for (text_index, vec) in text.iter().enumerate() {
        for (index, entry) in vec.iter().enumerate() {
            transposed_text[index][text_index] = *entry;
        }
    }
    transposed_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_repeating_xor() {
        let string1 = "this is a test".as_bytes().to_vec();
        let string2 = "wokka wokka!!!".as_bytes().to_vec();

        assert_eq!(
            calculate_hamming_distance(string1, string2),
            37.0
        );
    }

    #[test]
    fn test_transpose() {
        let transpose_me = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let actual = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(transpose_text(transpose_me), actual);
    }

    #[test]
    fn test_decryption() {
        let text = fs::read_to_string("text.txt").expect("Unable to read file");
        let text_bytes = base_64::decode(text.as_bytes());
        let result = decrypt_repeating_xor(text_bytes);
        let actual = "I\'m back and I\'m ringin\' the bell \nA rockin\' on the mike while the fly girls yell \nIn ecstasy in the back of me \nWell that\'s my DJ Deshay cuttin\' all them Z\'s \nHittin\' hard and the girlies goin\' crazy \nVanilla\'s on the mike, man I\'m not lazy. \n\nI\'m lettin\' my drug kick in \nIt controls my mouth and I begin \nTo just let it flow, let my concepts go \nMy posse\'s to the side yellin\', Go Vanilla Go! \n\nSmooth \'cause that\'s the way I will be \nAnd if you don\'t give a damn, then \nWhy you starin\' at me \nSo get off \'cause I control the stage \nThere\'s no dissin\' allowed \nI\'m in my own phase \nThe girlies sa y they love me and that is ok \nAnd I can dance better than any kid n\' play \n\nStage 2 -- Yea the one ya\' wanna listen to \nIt\'s off my head so let the beat play through \nSo I can funk it up and make it sound good \n1-2-3 Yo -- Knock on some wood \nFor good luck, I like my rhymes atrocious \nSupercalafragilisticexpialidocious \nI\'m an effect and that you can bet \nI can take a fly girl and make her wet. \n\nI\'m like Samson -- Samson to Delilah \nThere\'s no denyin\', You can try to hang \nBut you\'ll keep tryin\' to get my style \nOver and over, practice makes perfect \nBut not if you\'re a loafer. \n\nYou\'ll get nowhere, no place, no time, no girls \nSoon -- Oh my God, homebody, you probably eat \nSpaghetti with a spoon! Come on and say it! \n\nVIP. Vanilla Ice yep, yep, I\'m comin\' hard like a rhino \nIntoxicating so you stagger like a wino \nSo punks stop trying and girl stop cryin\' \nVanilla Ice is sellin\' and you people are buyin\' \n\'Cause why the freaks are jockin\' like Crazy Glue \nMovin\' and groovin\' trying to sing along \nAll through the ghetto groovin\' this here song \nNow you\'re amazed by the VIP posse. \n\nSteppin\' so hard like a German Nazi \nStartled by the bases hittin\' ground \nThere\'s no trippin\' on mine, I\'m just gettin\' down \nSparkamatic, I\'m hangin\' tight like a fanatic \nYou trapped me once and I thought that \nYou might have it \nSo step down and lend me your ear \n\'89 in my time! You, \'90 is my year. \n\nYou\'re weakenin\' fast, YO! and I can tell it \nYour body\'s gettin\' hot, so, so I can smell it \nSo don\'t be mad and don\'t be sad \n\'Cause the lyrics belong to ICE, You can call me Dad \nYou\'re pitchin\' a fit, so step back and endure \nLet the witch doctor, Ice, do the dance to cure \nSo come up close and don\'t be square \nYou wanna battle me -- Anytime, anywhere \n\nYou thought that I was weak, Boy, you\'re dead wrong \nSo come on, everybody and sing this song \n\nSay -- Play that funky music Say, go white boy, go white boy go \nplay that funky music Go white boy, go white boy, go \nLay down and boogie and play that funky music till you die. \n\nPlay that funky music Come on, Come on, let me hear \nPlay that funky music white boy you say it, say it \nPlay that funky music A little louder now \nPlay that funky music, white boy Come on, Come on, Come on \nPlay that funky music \n";
        assert_eq!(result, actual);
    }
}
