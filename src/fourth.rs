mod util;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::char;

// Main program logic.
fn is_valid_room(room_code: &str) -> bool {
    let split = room_code.split("-");
    let mut vec = split.collect::<Vec<_>>();

    let checksum_string = vec.pop().unwrap();
    // NOTE: Hardcoding the length here. 
    let checksum = &checksum_string[4..9];

    let name_string = vec.concat();

    // Using a BTree as a HashMap.
    let mut count: HashMap<char, i32> = HashMap::new();

    for c in name_string.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    let mut values: Vec<_> = count.iter().collect();

    values.sort_by(|a, b| 
        if a.1.cmp(b.1) == Ordering::Equal {
            let first_char = *a.0 as i32;
            let second_char = *b.0 as i32;
            first_char.cmp(&second_char)
        } else {
            a.1.cmp(b.1).reverse()
        } 
    );

    let mut received_checksum = String::from("");

    for i in 0..5 {
        received_checksum.push(*values[i].0);
    }

    if received_checksum == checksum {
        true
    } else {
        false
    }
}

fn get_sector_id(room_code: &str) -> i32 {
    let split = room_code.split("-");
    let vec = split.collect::<Vec<_>>();

    let next_split = vec.last()
                        .unwrap()
                        .split("[");
    let code_vec = next_split.collect::<Vec<_>>();

    let id_string = code_vec.first()
                            .unwrap();

    id_string.parse::<i32>()
             .unwrap()
}

fn break_cipher(room_code: &str, sector_id: i32) -> String {
    let mut cipher: String = String::new();
    let shift: u32 = (sector_id as u32)%26;
    let mut new_char: u32;

    for character in room_code.chars() {
        if character == '-' {
            cipher.push(' ');
        } else {
            new_char = (character as u32) + shift;
            if new_char > 122 {
                cipher.push(char::from_u32(new_char - 26).unwrap());
            } else if new_char >= 97 && new_char <= 122 {
                cipher.push(char::from_u32(new_char).unwrap());
            }
        }
    }

    cipher
}

fn main() {
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp4");
    
    let split = input_string.split("\n");
    let vec = split.collect::<Vec<_>>();

    let mut sector_id: i32;
    let mut sum: i32 = 0;

    for room in vec {
        sector_id = get_sector_id(room.clone());
        if is_valid_room(room.clone()) {
            sum = sum + sector_id;
        }
        println!("{} {}", break_cipher(room.clone(), sector_id), sector_id);
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    
    use super::is_valid_room;
    use super::get_sector_id;
    use super::break_cipher;

    #[test]
    fn test_1() {
        let test_1_str = "aaaaa-bbb-z-y-x-123[abxyz]";
        assert_eq!(is_valid_room(test_1_str), true);
    }

    #[test]
    fn test_2() {
        let test_2_str = "a-b-c-d-e-f-g-h-987[abcde]";
        assert_eq!(is_valid_room(test_2_str), true);
    }

    #[test]
    fn test_3() {
        let test_3_str = "not-a-real-room-404[oarel]";
        assert_eq!(is_valid_room(test_3_str), true);
    }

    #[test]
    fn test_4() {
        let test_4_str = "totally-real-room-200[decoy]";
        assert_eq!(is_valid_room(test_4_str), false);
    }

    #[test]
    fn test_5() {
        let test_5_str = "totally-real-room-200[decoy]";
        assert_eq!(get_sector_id(test_5_str), 200);
    }

    #[test]
    fn test_6() {
        let test_6_str = "qzmt-zixmtkozy-ivhz-343[something]";
        assert_eq!(break_cipher(test_6_str, get_sector_id(test_6_str)), "very encrypted name");
    }
}
