use crate::shared::{Outcome, Solution};

use std::io::BufRead;
use std::path::PathBuf;

/// Solver for day 1
pub struct Day01;

impl Day01 {
    /// Convert a piece of string like `L8` to `-8`
    fn step_to_number(line: &str) -> i16 {
        let mut chars = line.chars();
        let dir = chars.next().expect("Line is empty");
        let mut step: i16 = chars.as_str().parse().expect("Failed to parse to int");
        if dir == 'L' {
            step *= -1;
        }
        step
    }
}

impl Solution for Day01 {
    /// Main method to get the solution
    fn run(&self, input_file: PathBuf) -> Outcome {
        let mut zeros_count = 0;

        self.get_file_reader(input_file)
            .lines()
            .fold(50, |acc, line| {
                let step = Self::step_to_number(&line.unwrap());
                let next = (acc + step).rem_euclid(100);
                if next == 0 {
                    zeros_count += 1
                }
                next
            });

        Outcome::Number(zeros_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_to_number() {
        assert_eq!(Day01::step_to_number("R4"), 4);
        assert_eq!(Day01::step_to_number("L11"), -11);
    }

    #[test]
    fn sample() {
        let solver = Day01;
        let result = solver.run(PathBuf::from("tests/day_01/sample.txt"));
        assert_eq!(result, Outcome::Number(3));
    }
}
