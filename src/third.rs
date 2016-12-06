// Main program logic.
// Checks if a triangle is valid or not.
fn is_valid_triangle(sides: Vec<i32>) -> bool {
    // Sum of any two sides must be larger than the third.
    return (sides[0] + sides[1] > sides[2]) && (sides[1] + sides[2] > sides[0]) && (sides[2] + sides[0] > sides[1])
}

fn proces_by_collumns(all_sides: Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;

    for i in 0..all_sides.len()/3 {
        let row = i*3;
        for col in 0..3 {
            let sides_vec = vec![all_sides[row][col], all_sides[row+1][col], all_sides[row+2][col]];
            if is_valid_triangle(sides_vec) {
                count += 1;
            }
        }
    }

    count
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
    let mut all_sides: Vec<Vec<i32>> = Vec::new();

    for list_of_sides in &vec {
        sides_vec = process_list(list_of_sides.clone());
    
        all_sides.push(sides_vec.clone());

        if is_valid_triangle(sides_vec) {
            counter = counter + 1;
        }
    }

    println!("{}", counter);

    println!("{}", proces_by_collumns(all_sides.clone()));
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
