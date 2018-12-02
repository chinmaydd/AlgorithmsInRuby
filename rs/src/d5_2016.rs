extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;
use std::collections::HashMap;
use std;

pub fn run() {
    let mut hasher = Md5::new();

    let key = "wtnhxymk".as_bytes();
    let mut count = 0;
    let mut first_char: char;
    let mut second_char: char;
    let mut solved_map = HashMap::new();

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0;16];
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            first_char = hasher.result_str().chars().nth(5).unwrap();
            second_char = hasher.result_str().chars().nth(6).unwrap();

            if !first_char.is_alphabetic() && !solved_map.contains_key(&first_char) {
                println!("{} {}", first_char, second_char);
                solved_map.insert(first_char, second_char);
                count += 1;
            }

            if count == 9 {
                break;
            }
        }
        hasher.reset();
    }
}
