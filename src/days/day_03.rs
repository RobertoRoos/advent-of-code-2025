use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day03;

impl Solution for Day03 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        self.run_with_digits(input_file, 2)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        self.run_with_digits(input_file, 12)
    }
}

impl Day03 {
    fn run_with_digits(&self, input_file: PathBuf, digits: usize) -> Outcome {
        let sum: u64 = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| Self::parse_line(&line.unwrap()))
            .map(|list| Self::make_highest_number(&list, digits))
            .sum();

        Outcome::U64(sum)
    }

    /// Turn line of number characters into a vector of numbers
    fn parse_line(line: &str) -> Vec<u8> {
        const RADIX: u32 = 10;
        line.chars()
            .map(|c| c.to_digit(RADIX).unwrap().try_into().unwrap())
            .collect()
    }

    /// Get the highest number of a list, ignoring elements at the start or end
    fn find_max_in(list: &[u8], skip_start: usize, skip_end: usize) -> (usize, u8) {
        list[skip_start..(list.len() - skip_end)]
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, value)| value)
            .map(|(idx, &value)| (idx + skip_start, value))
            .unwrap()
        // `enumerate` will create an index _after_ the slice, hence we need to offset it again
        // later by `skip_start`.
        // `max_by_key` will always prefer last values, but we really need to the first, hence
        // we reverse the iterator order.
    }

    /// Return the highest number that can be composed by the list of single digits
    ///
    /// This is done by first finding the highest digit (except the last `digits - 1` items) and
    /// then combining it with the highest digit that follows it (except the last `digits - 2`
    /// items), etc.
    fn make_highest_number(list: &[u8], digits: usize) -> u64 {
        let mut num: u64 = 0;
        let mut idx = 0;

        for i in (0..digits).rev() {
            let next_digit: u8;
            (idx, next_digit) = Self::find_max_in(list, idx, i);
            idx += 1;
            assert!((1..=9).contains(&next_digit));
            num += u64::from(next_digit) * 10_u64.pow(i.try_into().unwrap());
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_highest_number() {
        assert_eq!(Day03::make_highest_number(&[1, 2, 3, 4, 5], 2), 45);
        assert_eq!(Day03::make_highest_number(&[5, 4, 3, 2, 1], 2), 54);
        assert_eq!(
            Day03::make_highest_number(&[5, 6, 3, 1, 2, 8, 9, 2, 1], 2),
            92
        );
        // Make sure that we don't pick the last highest number:
        assert_eq!(
            Day03::make_highest_number(&[6, 7, 6, 5, 7, 5, 7, 1, 7, 5], 2),
            77
        );
    }

    #[test]
    fn test_make_highest_number_more_digits() {
        assert_eq!(Day03::make_highest_number(&[1, 2, 3, 4, 5], 3), 345);
        assert_eq!(Day03::make_highest_number(&[5, 4, 3, 2, 1], 4), 5432);
        assert_eq!(
            Day03::make_highest_number(&[5, 6, 3, 1, 2, 8, 9, 2, 1], 4),
            8921
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

    #[test]
    fn test_part_2_sample() {
        let solver = Day03 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_03/sample.txt"));
        assert_eq!(result, Outcome::U64(3_121_910_778_619));
    }
}
