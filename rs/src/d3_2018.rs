use util;

use std::collections::HashSet;

extern crate ndarray;
use self::ndarray::Array2;

pub struct Claim {
    id: i64,
    x_coord: usize,
    y_coord: usize,
    width: usize,
    height: usize 
}

pub fn new_claim_from_string(claim_string: &str) -> Claim {
    let at_idx = claim_string
        .find('@')
        .unwrap();
    let id = &claim_string[1..at_idx]
        .trim()
        .parse::<i64>()
        .unwrap();

    let colon_idx = claim_string
        .find(':')
        .unwrap();
    let coord_string = &claim_string[at_idx+1..colon_idx];
    let comma_idx = coord_string
        .find(',')
        .unwrap();

    let x_coord = &coord_string[1..comma_idx]
        .parse::<i64>()
        .unwrap();
    let y_coord = &coord_string[comma_idx+1..]
        .parse::<i64>()
        .unwrap();

    let dimension_string = &claim_string[colon_idx+2..];
    let cross_idx = dimension_string
        .find('x')
        .unwrap();
    let width = &dimension_string[0..cross_idx]
        .parse::<i64>()
        .unwrap();
    let height = &dimension_string[cross_idx+1..]
        .parse::<i64>()
        .unwrap();

    Claim {
        id: *id,
        x_coord: *x_coord as usize,
        y_coord: *y_coord as usize,
        width: *width as usize,
        height: *height as usize
    }
}

fn generate_all_claims(input_string: &str) -> Vec<Claim> {
    let all_lines = input_string
        .lines()
        .collect::<Vec<_>>();

    let mut all_claims = Vec::new();
    for line in all_lines {
        all_claims.push(new_claim_from_string(&line));
    }

    all_claims
}

fn generate_no_overlap_id(claims_vec: &Vec<Claim>) -> i64 {
    let mut ids = HashSet::new();
    for claim in claims_vec {
        ids.insert(claim.id);
    }

    let mut x_coord_max = 0;
    let mut y_coord_max = 0;
    for claim in claims_vec {
        if claim.x_coord + claim.width > x_coord_max {
            x_coord_max = claim.x_coord + claim.width;
        }

        if claim.y_coord + claim.height > y_coord_max {
            y_coord_max = claim.y_coord + claim.height;
        }
    }

    // Algorithm: 
    // Taint the matrix at that index with the id of the claim
    // During taint, check if there exists another marker already
    // If yes -> Remove both ids from probabables
    // If no -> Add taint
    // We should ideally be left with a single ID once we are done.
    let mut claims_matrix = Array2::<i64>::zeros((x_coord_max+1, y_coord_max+1));
    for claim in claims_vec {
        for i in claim.x_coord..claim.x_coord+claim.width {
            for j in claim.y_coord..claim.y_coord+claim.height {
                if claims_matrix[[i, j]] == 0 {
                    claims_matrix[[i, j]] = claim.id;
                } else {
                    ids.remove(&claim.id);
                    ids.remove(&claims_matrix[[i, j]]);
                }
            }
        }
    }

    let remaining_ids = ids.iter().collect::<Vec<_>>();

    if remaining_ids.len() == 1 {
        **remaining_ids.get(0).unwrap()
    } else {
        panic!("[ERROR] Problem failed!");
    }
}

fn calculate_overlap(claims_vec: &Vec<Claim>) -> i64 {
    let mut x_coord_max = 0;
    let mut y_coord_max = 0;
    for claim in claims_vec {
        if claim.x_coord + claim.width > x_coord_max {
            x_coord_max = claim.x_coord + claim.width;
        }

        if claim.y_coord + claim.height > y_coord_max {
            y_coord_max = claim.y_coord + claim.height;
        }
    }

    // Algorithm:
    // Increment matrix count at that point
    // Finally, check those which have count > 1
    let mut claims_matrix = Array2::<i64>::zeros((x_coord_max+1, y_coord_max+1));
    for claim in claims_vec {
        for i in claim.x_coord..claim.x_coord+claim.width {
            for j in claim.y_coord..claim.y_coord+claim.height {
                claims_matrix[[i, j]] += 1;
            }
        }
    }

    let mut overlap_area = 0;
    for i in 0..x_coord_max {
        for j in 0..y_coord_max {
            if claims_matrix[[i, j]] > 1 {
                overlap_area += 1;
            }
        }
    }

    overlap_area
}

pub fn run() {
    // Read input from a file
    let input_string = util::read_into_string("/home/chinmay_dd/Projects/Code/rs/inp/inp3_2018");

    // Generate Claims vector
    let claims_vec: Vec<Claim> = generate_all_claims(&input_string);

    // Calculate overlaps
    let overlap_area = calculate_overlap(&claims_vec);

    // Print result for Part One
    println!("[3.1]: {}", overlap_area);

    let id_for_no_overlap_claim = generate_no_overlap_id(&claims_vec);

    // Print result for Part Two
    println!("[3.2]: {}", id_for_no_overlap_claim);
}

#[cfg(test)]
mod tests {
    use super::{Claim, calculate_overlap, new_claim_from_string, generate_no_overlap_id};

    #[test]
    fn test_1() {
        let claims_vec = vec![
            new_claim_from_string("#1 @ 1,3: 4x4"),
            new_claim_from_string("#2 @ 3,1: 4x4"),
            new_claim_from_string("#3 @ 5,5: 2x2")];

        assert_eq!(calculate_overlap(&claims_vec), 4);
    }

    #[test]
    fn test_2() {
        let claims_vec = vec![
            new_claim_from_string("#1 @ 1,3: 4x4"),
            new_claim_from_string("#2 @ 3,1: 4x4"),
            new_claim_from_string("#3 @ 5,5: 2x2")];

        assert_eq!(generate_no_overlap_id(&claims_vec), 3);
    }
}
