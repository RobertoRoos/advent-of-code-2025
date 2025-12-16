use crate::shared::{Outcome, Solution};
use std::path::PathBuf;

pub struct Day09;

impl Solution for Day09 {
    fn run_part_1(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d9p1"))
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d9p2"))
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
