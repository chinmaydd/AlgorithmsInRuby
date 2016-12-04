mod util;

// Main program logic.
// Checks if a triangle is valid or not.
fn is_valid_triangle(sides: Vec<i32>) -> bool {
    // Sum of any two sides must be larger than the third.
    if (sides[0] + sides[1] > sides[2]) && (sides[1] + sides[2] > sides[0]) && (sides[2] + sides[0] > sides[1]) {
        true
    } else {
        false
    }
}

// Converts a list of numbers into a Vec of integers which would simplify processing.
fn process_list(list_of_sides: &str) -> Vec<i32> {
    let mut return_list = Vec::new();  

    let split = list_of_sides.split_whitespace();
    let vec = split.collect::<Vec<_>>();

    for side_length in vec {
        return_list.push(side_length.parse::<i32>().unwrap());
    }

    return_list
}

fn main() {
    // Read input into a file.
    // In this case it is the input we need to solve for.
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp3");

    // Split the entire input into Vec of strings
    let split = input_string.split("\n");
    let vec = split.collect::<Vec<_>>();

    let mut counter: i32 = 0;
    let mut sides_vec: Vec<i32>; 

    for list_of_sides in vec {
        sides_vec = process_list(list_of_sides.clone());

        if is_valid_triangle(sides_vec) {
            counter = counter + 1;
        }
    }

    println!("{}", counter);
}

// Given example tests
#[cfg(test)]
mod tests {
    
    use super::is_valid_triangle;

    #[test]
    fn test_1() {
        let test_1_sides = [5,10,25];
        assert_eq!(is_valid_triangle(test_1_sides), false);
    }

    #[test]
    fn test_2() {
        let test_2_sides = [5,5,5];
        assert_eq!(is_valid_triangle(test_2_sides), true);
    }
}
