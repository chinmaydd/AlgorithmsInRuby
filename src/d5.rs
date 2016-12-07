extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::md5::Md5;
use std;

pub fn run() {
    let mut hasher = Md5::new();

    let key = "wtnhxymk".as_bytes();
    let mut count = 0;

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0;16];
        hasher.result(&mut output);

        let mut answer: String = String::from("");

        let first_five = output[0] as i32 + output[1] as i32 + (output[2] >> 4) as i32;
        if first_five == 0 {
            answer.push(hasher.result_str()
                              .chars()
                              .nth(5)
                              .unwrap());

            count += 1;
            if count == 8 {
                println!("[5.1]: {}", answer);
                break;
            }
        }
        hasher.reset();
    }
}
