use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day02;

impl Day02 {}

impl Day02 {
    /// Parse an ID range, e.g. "11-22" becomes (11, 22)
    fn parse_id_range(segment: &str) -> (u64, u64) {
        let mut parts = segment.split("-");
        (
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }

    /// Check if a given number is invalid
    ///
    /// Invalid numbers are a group of digits repeated, e.g. `123123`.
    fn is_invalid_code(code: u64) -> bool {
        let code_str = format!("{}", code);

        let half_size = code_str.len() / 2;
        if half_size * 2 != code_str.len() {
            return false; // When the length is uneven the code cannot be invalid
        }
        // Check if the string is one part repeated
        // Note that we do byte position, not UTF-8 characters! But we know it's only numerals,
        // so we can get away with it here
        code_str[..half_size] == code_str[half_size..]
    }

    /// In a provided product range, count the number of invalid IDs
    fn sum_invalid_ids(from: u64, to: u64) -> u64 {
        (from..=to).filter(|&num| Self::is_invalid_code(num)).sum()
    }
}

impl Solution for Day02 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let line = self
            .get_file_reader(input_file)
            .lines()
            .next()
            .unwrap()
            .unwrap();

        let sum: u64 = line
            .split(",")
            .map(Self::parse_id_range)
            .map(|(from, to)| Self::sum_invalid_ids(from, to))
            .sum();

        Outcome::U64(sum)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("day 2 - part 2"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_invalid_code() {
        assert_eq!(Day02::is_invalid_code(11), true);
        assert_eq!(Day02::is_invalid_code(12), false);
        assert_eq!(Day02::is_invalid_code(22), true);
        assert_eq!(Day02::is_invalid_code(1234), false);
        assert_eq!(Day02::is_invalid_code(2121), true);
        assert_eq!(Day02::is_invalid_code(2111), false);
        assert_eq!(Day02::is_invalid_code(123123), true);
        assert_eq!(Day02::is_invalid_code(1230123), false);
    }

    #[test]
    fn sum_invalid_ids() {
        assert_eq!(Day02::sum_invalid_ids(11, 22), 11 + 22);
        assert_eq!(Day02::sum_invalid_ids(38593856, 38593862), 38593859);
    }

    #[test]
    fn part_1_sample() {
        let solver = Day02 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_02/sample.txt"));
        assert_eq!(result, Outcome::U64(1227775554));
    }
}
