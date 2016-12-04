mod util;
use std::collections::HashMap;
use std::cmp::Ordering;

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

fn main() {
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp4");
    
    let split = input_string.split("\n");
    let vec = split.collect::<Vec<_>>();

    let mut sum: i32 = 0;

    for room in vec {
        if is_valid_room(room.clone()) {
            sum = sum + get_sector_id(room.clone());
        }
    }

    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    
    use super::is_valid_room;
    use super::get_sector_id;

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
}
