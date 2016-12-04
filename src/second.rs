mod util;

// Contains the main function logic.
// We again use pattern matching for simplifying the problem solving by including border cases.
// This can be done in a much better way maybe by using a static 'keypad' for accesses.
// Given the current key and the sequence of operations to be performed, it returns the final key
// which is pressed.
fn next_key(key_seq: &str, init_key: i32) -> i32 {
    let mut curr_key = init_key;
    
    for direction in key_seq.chars() {
        match (direction, curr_key) {
            ('U', 1) |  ('L', 1) | ('L', 4) | ('L', 7) | ('D', 7) | ('U', 2) | ('D', 8) | ('U', 3) | ('R', 3) | ('R', 6) | ('R', 9) | ('D', 9) => {},
            ('U', val) => {curr_key = val - 3},
            ('D', val) => {curr_key = val + 3},
            ('L', val) => {curr_key = val - 1},
            ('R', val) => {curr_key = val + 1},
            (_, _) => panic!("Something is wrong!")
        }
    }

    curr_key
}
    
// Function which takes in a long string of operations, seperated by newline characters and applies
// the next_key() function on each of them.
// It returns the final code string as asked in the problem.
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
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/RAdventOfCode/inp/inp2");

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
