use std::fs::File;
use std::io::Read;

pub fn blocks_away(path: &str) -> u64{
    // dummy value
    1
}

#[cfg(test)]
mod tests {

    use super::blocks_away;

    #[test]
    fn test_1() {
        let test_1_str = "R2, L3";
        assert_eq!(blocks_away(test_1_str), 5);
    }

    #[test]
    fn test_2() {
        let test_2_str = "R2, R2, R2";
        assert_eq!(blocks_away(test_2_str), 2);
    }

    #[test]
    fn test_3() {
        let test_3_str = "R5, L5, R5, R3";
        assert_eq!(blocks_away(test_3_str), 12);
    }
}
