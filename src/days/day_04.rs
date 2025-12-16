use crate::shared::{Outcome, Solution};
use std::path::PathBuf;

pub struct Day04;

impl Solution for Day04 {
    fn run_part_1(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d4p1"))
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d4p2"))
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
