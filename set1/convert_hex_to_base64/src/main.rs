fn main() {
    
}

fn decode_string(hex_string: &str) -> String {        
    split_and_strip_whitespace(hex_string)
        .chunks(2) // split up string into 2 digit parts 
        .map(|entry| entry[0].to_owned() + entry[1]) // then join these back together
        .map(|x| i64::from_str_radix(&x, 16).unwrap()) // not sure if this step is cheating - could try to reimplement
        .map(|x| format!("{:b}", x)) // converts to binary
        .map(|x| format!("{:0>#8}", x)) // they should be 8 bits - this adds left 0 padding to 8 characters
        .fold(String::new(), |acc, x| acc + &x.to_owned())
        .chars()
        .collect::<Vec<char>>()
        .chunks(6) // why does windows not do what I think?!
        .map(|x| x.iter().collect::<String>()).collect::<Vec<String>>()
        .into_iter()
        .map(|x| i64::from_str_radix(&x, 2).unwrap()) // converts from the binary to decimal
        .map(|x| find_base64_char(&x))
        .collect::<Vec<_>>()
        .join("")
}

fn find_base64_char(characher_to_find: &i64) -> String {
    let base64 = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"); // we'll split on this to decode

    let base64_enumerator = split_and_strip_whitespace(&base64).into_iter().enumerate();
    
    let mut found_character = String::new();
    for (index, item) in base64_enumerator {
        if index == *characher_to_find as usize {
            found_character.push_str(item);
        }
    }
    found_character
}

fn split_and_strip_whitespace(string: &str) -> Vec<&str> {
    let mut f: Vec<_> = string.split("").collect();
    // for some reason it has empty strings at the beginning and end
    let mut u: Vec<_> = f.drain(1..).collect(); // removes the first empty ''
    u.pop(); // remove '' from the end
    u
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

        assert_eq!(decode_string(hex), base64);
    }
}

