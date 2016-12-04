mod util;

// This enum will help us give a direction to our path tracer.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

// A move can be defined using two params:
// 1. The direction in which the move takes place
// 2. The number of steps taken
#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    step: i32,
}

// Each token can be converted into a "Move".
// Given a token, this function converts into a Move struct.
fn convert_token_to_move(token: &str) -> Move {
    // Initialize a new_move
    let mut new_move = Move {direction: Direction::Left, step: 0};

    // The first character specifies the direction.
    // For eg. "R123" -> Direction is R
    let direction = token.chars().nth(0);

    // Pattern match on the direction.
    new_move.direction = match direction {
                            Some('R') => Direction::Right,
                            Some('L') => Direction::Left,
                            _ => panic!("Unknown direction!"),
                        };

    // Retrieve the number of steps taken and store that in the struct.
    new_move.step = token[1..].parse::<i32>().unwrap();

    // Return value
    new_move
}

// This function basically converts the given comma separated string,
// into a vector of Move(s).
fn tokenize(input: &str) -> Vec<Move> {
    let split = input.split(", ");
    let vec = split.collect::<Vec<_>>();

    let mut move_vec: Vec<Move> = Vec::new();

    // Iterate over each token in the vec
    for token in vec {
        move_vec.push(convert_token_to_move(token));
    }

    move_vec
}

fn main() {
    // Read input into a file.
    // In this case it is the input we need to solve for.
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp1"); 

    // Tokenize into a list of Moves -> move_vec
    let move_vec: Vec<Move> = tokenize(&input_string);
    
    // Calculate the number of blocks
    let num_of_blocks = blocks_away(move_vec.clone());

    // Print the result
    println!("{}", num_of_blocks);
}

// This function contains the main logic of the program.
// Given a vector of moves, it tells us how many blocks away the final position is.
fn blocks_away(path: Vec<Move>) -> i32 {
    let mut total_h = 0;
    let mut total_v = 0;

    // Initializer
    let mut curr_dir: Direction = path[0].direction;
    if curr_dir == Direction::Left {
        total_h = total_h - path[0].step;
    } else {
        total_h = total_h + path[0].step;
    }

    // For each move, take a decision based on the current direction and the new move.
    for single_move in path.iter().skip(1) {
        match (curr_dir, single_move.direction) {
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {
                curr_dir = Direction::Up;
                total_v = total_v + single_move.step;
            },
            (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {
                curr_dir = Direction::Down;
                total_v = total_v - single_move.step;
            },
            (Direction::Up, Direction::Right) | (Direction::Down, Direction::Left) => {
                curr_dir = Direction::Right;
                total_h = total_h + single_move.step;
            },
            (Direction::Up, Direction::Left) | (Direction::Down, Direction::Right) => {
                curr_dir = Direction::Left;
                total_h = total_h - single_move.step;
            }
            (_, _) => {
                panic!("We are not supposed to be going this way!");
            }
        }
    }

    // The total distance to be covered will be the sum absolute of the 
    // vertical and horizontal distances.
    total_h.abs() + total_v.abs()
}

// Given example tests.
#[cfg(test)]
mod tests {

    use super::tokenize;
    use super::blocks_away;

    #[test]
    fn test_1() {
        let test_1_str = "R2, L3";
        let t1 = tokenize(test_1_str);
        assert_eq!(blocks_away(t1), 5);
    }

    #[test]
    fn test_2() {
        let test_2_str = "R2, R2, R2";
        let t2 = tokenize(test_2_str);
        assert_eq!(blocks_away(t2), 2);
    }

    #[test]
    fn test_3() {
        let test_3_str = "R5, L5, R5, R3";
        let t3 = tokenize(test_3_str);
        assert_eq!(blocks_away(t3), 12);
    }
}
