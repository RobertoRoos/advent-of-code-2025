use crate::shared::Solution;

use std::io::BufRead;
use std::path::PathBuf;

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
    fn run(&self, input_file: PathBuf) -> String {
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

        format!("{}", zeros_count)
    }
}
