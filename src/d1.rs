use util;

/// A direction is being represented here. 
/// A move can be in one of the directions represented by this enum.
/// This will help us in simulating a path tracer.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

/// A move is being represented here.
/// It consists of a direction and a step attribute.
#[derive(Debug, Clone, Copy)]
struct Move {
    direction: Direction,
    step: i32,
}

impl Move {
    /// Returns a move with the specified directoin and steps
    ///
    /// # Arguments
    /// * `token` - A string slice that holds the move direction and steps.
    ///
    /// # Example
    ///
    /// ```
    /// let new_move = Move::generate_move_from_token("R12");
    /// ```
    pub fn generate_move_from_token(token: &str) -> Move {
        // Initialize a new_move
        let mut new_move = Move {direction: Direction::Left, step: 0};

        // The first character specifies the direction.
        // For eg. "R123" -> Direction is R
        let direction = token.chars().nth(0);

        // (Pattern) Match on the direction.
        new_move.direction = match direction {
                                Some('R') => Direction::Right,
                                Some('L') => Direction::Left,
                                _ => panic!("Unknown direction!"),
                            };

        // Retrieve the number of steps taken and store that in the struct.
        new_move.step = token[1..].parse::<i32>().unwrap();

        new_move
    }
}

/// Describes an Axis on the plane of the tracer.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Axis {
    X,
    Y
}

/// Describes a line segment on the plane.
/// We implement an optimization here wherein we leverage the fact that
/// the line can either be vertical or horizontal.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    axis: Axis,
    other_coord: i32,
    start: i32,
    end: i32
}

impl Line {
    /// Returns a new line with the specified data. This line is a line segment which is
    /// representative of a segment of the path traced.
    ///
    /// # Arguments
    /// * `axis`: Specifies if the line is parallel to the X or Y axis.
    /// * `other_coord`: Assuming that it is parallel to either, we choose the other constant
    /// coordinate.
    /// * `start`: Starting point of the segment.
    /// * `end`: End point of the segment.
    ///
    /// # Example
    ///
    /// ```
    /// // For creating a line segment from (3, 2) to (3, 5):
    /// let new_line = Line::new(Axis::Y, 3, 2, 5)
    /// ```
    ///
    /// A Line will always have the property, start < end.
    fn new(axis: Axis, other_coord: i32, start: i32, end: i32) -> Line {
        Line {
            axis: axis,
            other_coord: other_coord,
            start: if start < end {start} else {end},
            end: if start < end {end} else {start}
        }
    }
    
    /// Returns true if the line is parallel to the Y Axis
    fn is_vertical(&self) -> bool {
        self.axis == Axis::Y
    }

    /// Returns true if the line is parallel to the X Axis.
    fn is_horizontal(&self) -> bool {
        self.axis == Axis::X
    }
    
    /// Returns true if the lines are parallel to each other.
    fn is_parallel(&self, other: &Line) -> bool {
        (self.is_horizontal() && other.is_horizontal()) ||
        (self.is_vertical() && other.is_vertical())
    }
    
    /// Returns an Option<(i32, i32)> which unwraps as `None` if the lines do not intersect
    /// otherwise, it contains the intersection point.
    fn intersect(&self, other: &Line) -> Option<(i32, i32)> {
        if self.is_parallel(other) {
            return None;
        }
        
        if (self.other_coord > other.start) && (self.other_coord < other.end) &&
            (other.other_coord > self.start) && (other.other_coord < self.end) {
                if self.axis == Axis::X {
                    return Some((other.other_coord, self.other_coord));
                } else {
                    return Some((self.other_coord, other.other_coord));
                }
        } else {
            return None
        }
    }
}

/// Returns a vector of Moves by taking an entire string of comma seperated tokens.
fn tokenize(input: &str) -> Vec<Move> {
    let split = input.split(", ");
    let vec = split.collect::<Vec<_>>();

    let mut move_vec: Vec<Move> = Vec::new();

    // Iterate over each token in the vec
    for token in vec {
        move_vec.push(Move::generate_move_from_token(token));
    }

    move_vec
}

pub fn run() {
    // Read input into a file.
    // In this case it is the input we need to solve for.
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/r_aoc/inp/inp1"); 

    // Tokenize into a list of Moves -> move_vec
    let move_vec: Vec<Move> = tokenize(&input_string);
    
    // Calculate the number of blocks
    let num_of_blocks = blocks_away(move_vec.clone());

    // Print the result
    println!("[1.1]: {}", num_of_blocks);
}

/// Given a vector of moves, it tells us how many blocks away the final position is.
fn blocks_away(path: Vec<Move>) -> i32 {
    let mut total_h = 0;
    let mut total_v = 0;


    // For problem part 2
    let mut total_path: Vec<Line> = Vec::new();
    let mut solved = false;

    // Initializer
    let mut curr_dir: Direction = path[0].direction;
    if curr_dir == Direction::Left {
        total_h = total_h - path[0].step;
    } else {
        total_h = total_h + path[0].step;
    }
    total_path.push(Line::new(Axis::X, 0, total_h, 0));


    let mut path_segment: Line; 

    // For each move, take a decision based on the current direction and the new move.
    for single_move in path.iter().skip(1) {
        match (curr_dir, single_move.direction) {
            
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => {    
                path_segment = Line::new(Axis::Y, total_h, total_v, total_v + single_move.step);
                curr_dir = Direction::Up;
                total_v = total_v + single_move.step;
            },

            (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => {    
                path_segment = Line::new(Axis::Y, total_h, total_v - single_move.step , total_v);
                curr_dir = Direction::Down;
                total_v = total_v - single_move.step;
            },

            (Direction::Up, Direction::Right) | (Direction::Down, Direction::Left) => {
                path_segment = Line::new(Axis::X, total_v, total_h, total_h + single_move.step);
                curr_dir = Direction::Right;
                total_h = total_h + single_move.step;
            },

            (Direction::Up, Direction::Left) | (Direction::Down, Direction::Right) => {    
                path_segment = Line::new(Axis::X, total_v, total_h - single_move.step, total_h);
                curr_dir = Direction::Left;
                total_h = total_h - single_move.step;
            }

            (_, _) => {
                panic!("We are not supposed to be going this way!");
            }
        };
        
        if !solved {
            for line in &total_path {
                match path_segment.intersect(&line) {
                    Some((x, y)) => 
                    {   
                        println!("[1.2]: {}", x.abs() + y.abs());
                        solved = true;
                    },
                    _ => {}
                }
            }
        }
        
        total_path.push(path_segment);
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
    use super::Axis;
    use super::Line;

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

    #[test]
    fn test_4() {
        let line1 = Line::new(Axis::X, 2, 0, 5);
        let line2 = Line::new(Axis::Y, 2, 0, 4);

        assert_eq!(line1.intersect(&line2), Some((2, 2)));
    }
}
