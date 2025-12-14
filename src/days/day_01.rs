use crate::shared::{Outcome, Solution};

use std::io::BufRead;
use std::path::PathBuf;

/// Solver for day 1
pub struct Day01;

impl Day01 {
    /// Convert a piece of string like `L8` to `-8`
    fn step_to_number(line: &str) -> i32 {
        let mut chars = line.chars();
        let dir = chars.next().expect("Line is empty");
        let mut step: i32 = chars.as_str().parse().expect("Failed to parse to int");
        if dir == 'L' {
            step *= -1;
        }
        step
    }

    /// Update code with step, wrapping in [0, 100> and count the number of zero-passes
    ///
    /// Note that if `code` starts at 0, this won't be counted!
    fn wrap_step(code: i32, step: i32) -> (i32, i32) {
        let mut new_code = code;
        let mut zeros_count: i32;

        if step >= 0 {
            new_code += step;
            zeros_count = new_code / 100; // Simply divide (rounding down is implicit)
            new_code = new_code.rem_euclid(100);
        } else {
            new_code += step;
            // Division quotient (will equal -1 when e.g. `new_code = -1`)
            zeros_count = new_code.div_euclid(100).abs();
            if code == 0 {
                zeros_count -= 1; // Don't count when we started at zero already
            }
            new_code = new_code.rem_euclid(100);
            if new_code == 0 {
                zeros_count += 1; // Also count the final 0
            }
        }
        (new_code, zeros_count)
    }
}

impl Solution for Day01 {
    /// Part 1 solution
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let mut zeros_count: i32 = 0;

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

        Outcome::I32(zeros_count)
    }

    /// Part 2 solution
    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let (_final_code, zeros) =
            self.get_file_reader(input_file)
                .lines()
                .fold((50, 0), |acc, line| {
                    let (code, zeros) = acc;
                    let step = Self::step_to_number(&line.unwrap());
                    let (next_code, extra_zeros) = Self::wrap_step(code, step);
                    (next_code, zeros + extra_zeros)
                });
        Outcome::I32(zeros)
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
    fn part_1_sample() {
        let solver = Day01 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_01/sample.txt"));
        assert_eq!(result, Outcome::I32(3));
    }

    #[test]
    fn wrap_step() {
        // Sample:
        assert_eq!(Day01::wrap_step(50, -68), (82, 1));
        assert_eq!(Day01::wrap_step(82, -30), (52, 0));
        assert_eq!(Day01::wrap_step(52, 48), (0, 1));
        assert_eq!(Day01::wrap_step(0, -5), (95, 0));
        assert_eq!(Day01::wrap_step(95, 60), (55, 1));
        assert_eq!(Day01::wrap_step(55, -55), (0, 1));
        assert_eq!(Day01::wrap_step(0, -1), (99, 0));
        assert_eq!(Day01::wrap_step(99, -99), (0, 1));
        assert_eq!(Day01::wrap_step(0, 14), (14, 0));
        assert_eq!(Day01::wrap_step(14, -82), (32, 1));

        // Increment:
        assert_eq!(Day01::wrap_step(0, 10), (10, 0));
        assert_eq!(Day01::wrap_step(0, 101), (1, 1));
        assert_eq!(Day01::wrap_step(80, 20), (0, 1));
        assert_eq!(Day01::wrap_step(80, 25), (5, 1));
        assert_eq!(Day01::wrap_step(50, 1000), (50, 10));

        // Decrement:
        assert_eq!(Day01::wrap_step(50, -30), (20, 0));
        assert_eq!(Day01::wrap_step(1, -2), (99, 1));
        assert_eq!(Day01::wrap_step(1, -102), (99, 2));
        assert_eq!(Day01::wrap_step(0, -10), (90, 0));
        assert_eq!(Day01::wrap_step(0, -110), (90, 1));
        assert_eq!(Day01::wrap_step(50, -1000), (50, 10))
    }

    #[test]
    fn part_2_sample() {
        let solver = Day01 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_01/sample.txt"));
        assert_eq!(result, Outcome::I32(6));
    }
}
