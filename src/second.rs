use std::fs::File;
use std::io::Read;

// Given a path to a file, this function reads it into a string and returns it.
// NOTE: Should ideally move this function to a util.rs file.
fn read_into_string(filepath: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(filepath).expect("Unable to open required file.");
    f.read_to_string(&mut input).expect("Unable to read string.");

    // Removing trailing newline.
    input.pop().expect("Could not remove trailing newline.");

    input
}

fn next_key(key_seq: &str, init_key: i32) -> i32 {
    let mut curr_key = init_key;
    
    for direction in keq_seq.chars() {
    }
    
    // dummy value
    curr_key
}

fn find_code(input: String) -> String {
    let mut final_code = String::from("");
    
    let split = input.split("\n");
    let vec = split.collect::<Vec<_>>();

    let mut curr_key = 5;

    for sequence in vec {
        curr_key = next_key(sequence.clone(), curr_key);
        final_code.push_str(&curr_key.to_string());
    }

    final_code
}

fn main() {
    // Read input into a file.
    // In this case it is the input we need to solve for.
    let input_string = read_into_string("/home/chinmay_dd/Projects/RAdventOfCode/inp/inp2");

    // Find the final code sequence
    let code = find_code(input_string.clone());

    // Print the result
    println!("{}", code);
}

// Given example tests.
#[cfg(test)]
mod tests {
    
    use super::next_key;

    #[test]
    fn test_1() {
        let test_1_str = "ULL";
        let test_1_init = 5;
        assert_eq!(next_key(test_1_str, test_1_init), 1);
    }

    #[test]
    fn test_2() {
        let test_2_str = "RRDDD";
        let test_2_init = 1;
        assert_eq!(next_key(test_2_str, test_2_init), 9);
    }

    #[test]
    fn test_3() {
        let test_3_str = "LURDL";
        let test_3_init = 9;
        assert_eq!(next_key(test_3_str, test_3_init), 8);
    }

    #[test]
    fn test_4() {
        let test_4_str = "UUUUD";
        let test_4_init = 8;
        assert_eq!(next_key(test_4_str, test_4_init), 5);
    }
}
