use util;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashSet;

fn parse_all_ids(input_string: &str) -> Vec<&str> {
    input_string
        .lines()
        .collect::<Vec<_>>()
}

fn get_count_map(id: &str) -> HashMap<char, i64> {
    let mut count_map = HashMap::new();

    for chr in id.chars() {
        let count = match count_map.entry(chr) {
            Vacant(entry) => entry.insert(0),
            Occupied(entry) => entry.into_mut(),
        };

        *count += 1;
    }

    count_map
}

fn calculate_checksum(ids_vec: &Vec<&str>) -> i64 {
    let mut count_twos = 0;
    let mut count_threes = 0;

    let mut counted_two;
    let mut counted_three;

    for id in ids_vec {
        counted_two = false;
        counted_three = false;
        
        let count_map = get_count_map(&id);

        for count in count_map.values() {
            if *count == 2 && !counted_two {
                count_twos += 1;
                counted_two = true;
            } else if *count == 3 && !counted_three {
                count_threes += 1;
                counted_three = true;
            }
        }
    }

    count_twos * count_threes
}

// TODO: Look at other interesting algorithms -> suffix and prefix trees for example
fn find_common_letters(ids_vec: &Vec<&str>) -> String {
    // Algorithm-1 description:
    // In the first pass, create a HashSet keyed on the first half
    // Find strings which differ by one character.
    //
    // If not found, create HashSet keyed on second half
    // Find strings which differ by one character.
    let mut common_letters = "".to_string();
    
    let mut first_half_keyed = HashMap::new();
    let mut first_half;

    const HALF_COUNT: usize = 13;

    for id in ids_vec {
        first_half = &id[..HALF_COUNT];

        match first_half_keyed.entry(first_half) {
            Vacant(entry) => { 
                entry.insert(id);
            }, 
            Occupied(entry) => {
                let id_to_be_compared_chars: Vec<char> = entry
                    .get()
                    .chars()
                    .collect();
                let id_chars: Vec<char> = id
                    .chars()
                    .collect();

                let mut differing_count = 0;
                // If the differing_count is indeed 1, we can capture the 
                // exact index and use it later for calculating result
                let mut differing_index = 0;
                for idx in HALF_COUNT..id_chars.len() {
                    if &id_chars[idx] != &id_to_be_compared_chars[idx] {
                        differing_count += 1;
                        differing_index = idx;
                    }
                }

                if differing_count == 1 {
                    let mut common_letters_vec = id
                        .clone()
                        .chars()
                        .collect::<Vec<char>>();

                    common_letters_vec.remove(differing_index);

                    common_letters = common_letters_vec
                        .iter()
                        .collect();
                }
            },
        };
    }

    // TODO: Implement part 2 also sometime later.
    // Abstract away the above code into a new function
    // find_one_diff_strings (ids_vec, idx1, idx2) -> Option<String>
    
    // Algorithm-2 description
    // Trust in Rust's HashSet difference algorithm. LMAO
    
    common_letters
}

pub fn run() {
    // Read input from file
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/Code/rs/inp/inp2_2018");

    // Parse input string into a vec of id strings
    let ids_vec = parse_all_ids(&input_string);

    // Calculate checksum given the ids
    let checksum = calculate_checksum(&ids_vec);

    // Print result for Part One
    println!("[2.1]: {}", checksum);

    // Count common_letters in "correct_box_ids"
    let common_letters = find_common_letters(&ids_vec);
    
    // Print result for Part Two
    println!("[2.2]: {}", common_letters);
}

#[cfg(test)]
mod tests {
    use super::calculate_checksum;

    #[test]
    fn test_1() {
        let ids_vec = vec!["abdcef", "bababc", "abbcde", "abcccd", "aabcdd"];
        assert_eq!(calculate_checksum(&ids_vec), 6);
    }
}
