use util;
use std::collections::HashMap;

fn parse_all_frequencies(input_string: &str) -> Vec<i64> {
    input_string
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn count_final_frequency(frequency_vec: &Vec<i64>) -> i64 {
    frequency_vec.iter().sum()
}

// TODO: There should be a better way to do this!
fn calculate_first_repeating_frequency(frequency_vec: &Vec<i64>) -> i64 {
    let mut frequency_map = HashMap::new();
    let mut current_frequency: i64 = 0;
    // Insert default value
    frequency_map.insert(current_frequency, true);

    let mut idx = 0;
    let return_frequency;

    loop {
        let frequency_at_idx = frequency_vec[idx];
        current_frequency += frequency_at_idx;

        if let Some(_) = frequency_map.get(&current_frequency) {
            return_frequency = current_frequency;
            break;
        } else {
            frequency_map.insert(current_frequency, true);
        }

        idx += 1;
        if idx == frequency_vec.len() {
            idx = 0;
        }
    }

    return_frequency
}

pub fn run () {
    // Read input from a file
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/Code/rs/inp/inp1_2018");

    // Fetch all frequencies into a vec
    let frequency_vec = parse_all_frequencies(&input_string);

    // Count final frequency
    let final_frequency = count_final_frequency(&frequency_vec);

    // Count first repeating frequency
    let first_repeating_frequency = calculate_first_repeating_frequency(&frequency_vec);

    // Print result for Part One
    println!("[1.1]: {}", final_frequency);

    // Print result for Part Two
    println!("[1.2]: {}", first_repeating_frequency);
}

#[cfg(test)]
mod tests {
    use super::calculate_first_repeating_frequency;

    #[test]
    fn test_1() {
        assert_eq!(calculate_first_repeating_frequency(&vec![1, -2, 3, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(calculate_first_repeating_frequency(&vec![1, -1]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(calculate_first_repeating_frequency(&vec![3, 3, 4, -2, -4]), 10);
    }

    #[test]
    fn test_4() {
        assert_eq!(calculate_first_repeating_frequency(&vec![-6, 3, 8, 5, -6]), 5);
    }

    #[test]
    fn test_5() {
        assert_eq!(calculate_first_repeating_frequency(&vec![7, 7, -2, -7, -4]), 14);
    }
}
