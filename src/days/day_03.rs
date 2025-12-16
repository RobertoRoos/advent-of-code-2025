use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::path::PathBuf;

pub struct Day03;

impl Solution for Day03 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let sum: u64 = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| Self::parse_line(&line.unwrap()))
            .map(|list| Self::make_highest_number(&list) as u64)
            .sum();

        Outcome::U64(sum)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d3p2"))
    }
}

impl Day03 {
    /// Turn line of number characters into a vector of numbers
    fn parse_line(line: &str) -> Vec<u8> {
        const RADIX: u32 = 10;
        line.chars()
            .map(|c| c.to_digit(RADIX).unwrap() as u8)
            .collect()
    }

    /// Get the highest number of a list, ignoring elements at the start or end
    fn find_max_in(list: &[u8], skip_start: usize, skip_end: usize) -> (usize, u8) {
        list[skip_start..(list.len() - skip_end)]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, value)| value)
            .map(|(idx, &value)| (idx, value))
            .unwrap()
        // `max_by_key` will always prefer last values, but we really need to the first, hence
        // we reverse the iterator order
    }

    /// Return the highest number that can be composed by the list of single digits
    fn make_highest_number(list: &[u8]) -> u8 {
        let (d1_idx, d1) = Self::find_max_in(list, 0, 1);
        let (_, d2) = Self::find_max_in(list, d1_idx + 1, 0);
        const DIGIT_RANGE: RangeInclusive<u8> = 1..=9;
        assert!(DIGIT_RANGE.contains(&d1));
        assert!(DIGIT_RANGE.contains(&d2));
        d1 * 10 + d2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_highest_number() {
        assert_eq!(Day03::make_highest_number(&[1, 2, 3, 4, 5]), 45);
        assert_eq!(Day03::make_highest_number(&[5, 4, 3, 2, 1]), 54);
        assert_eq!(Day03::make_highest_number(&[5, 6, 3, 1, 2, 8, 9, 2, 1]), 92);
        // Make sure that we don't pick the last highest number:
        assert_eq!(
            Day03::make_highest_number(&[6, 7, 6, 5, 7, 5, 7, 1, 7, 5]),
            77
        );
    }

    // /// Alternative solution based on slow brute-forcing
    // fn make_highest_number_brute_force(list: &Vec<u8>) -> u8 {
    //     let mut highest: u8 = 0;
    //     for (i1, d1) in list[..list.len() - 1].iter().enumerate() {
    //         for d2 in list[(i1 + 1)..].iter() {
    //             let candidate = d1 * 10 + d2;
    //             if candidate > highest {
    //                 highest = candidate;
    //             }
    //         }
    //     }
    //     highest
    // }

    #[test]
    fn test_part_1_sample() {
        let solver = Day03 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_03/sample.txt"));
        assert_eq!(result, Outcome::U64(357));
    }
}
