use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day02;

impl Day02 {}

impl Day02 {
    /// Shared solution logic for both part 1 and 2
    fn sum_invalid_ids_in_ranges(&self, input_file: PathBuf, only_two: bool) -> Outcome {
        let first_line = self
            .get_file_reader(input_file)
            .lines()
            .next()
            .unwrap()
            .unwrap();

        let func = if only_two {
            |(from, to)| Self::sum_invalid_ids_doubles(from, to)
        } else {
            panic!("Yet to be implemented!")
        };

        let sum: u64 = first_line
            .split(",")
            .map(Self::parse_id_range)
            .map(func)
            .sum();

        Outcome::U64(sum)
    }

    /// Parse an ID range, e.g. "11-22" becomes (11, 22)
    fn parse_id_range(segment: &str) -> (u64, u64) {
        let mut parts = segment.split("-");
        (
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        )
    }

    /// Get the next invalid id (or just the given one, if already invalid)
    fn get_next_or_current_invalid_id_doubles(num: u64) -> u64 {
        let mut num = num;

        loop {
            let num_mag = num.ilog10(); // Magnitude of the number, e.g. 3 (= 1000) for 4567
            if num_mag.is_multiple_of(2) {
                // If the code doesn't have an even number of digits it's never invalid
                // So skip to the next power of 10 (e.g. turn `123` into `1000`)
                num = 10_u64.pow(num_mag + 1);
                continue;
            }

            let factor = 10_u64.pow(num_mag / 2 + 1);
            let (mut left, right) = (num / factor, num % factor); // Turn e.g. 1234 into (12, 34)

            if left == right {
                return num;
            }
            if left > right {
                return left * factor + left; // E.g. (12, 01) --> 1212
            }

            left += 1; // E.g. (12, 23) --> 1313
            return left * factor + left;
        }
    }

    /// In a provided product range, sum the invalid ids (doubled digit group)
    fn sum_invalid_ids_doubles(from: u64, to: u64) -> u64 {
        let mut num = from;
        let mut sum = 0;
        loop {
            num = Self::get_next_or_current_invalid_id_doubles(num);
            if num > to {
                break;
            }
            sum += num;

            num += 1; // Continue
        }
        sum
    }
}

impl Solution for Day02 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        self.sum_invalid_ids_in_ranges(input_file, true)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        self.sum_invalid_ids_in_ranges(input_file, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_or_current_invalid_id_doubles() {
        assert_eq!(Day02::get_next_or_current_invalid_id_doubles(11), 11);
        assert_eq!(Day02::get_next_or_current_invalid_id_doubles(12), 22);
        assert_eq!(Day02::get_next_or_current_invalid_id_doubles(111), 1010);
        assert_eq!(
            Day02::get_next_or_current_invalid_id_doubles(123000),
            123123
        );
        assert_eq!(
            Day02::get_next_or_current_invalid_id_doubles(123124),
            124124
        );
        assert_eq!(
            Day02::get_next_or_current_invalid_id_doubles(5555555),
            10001000
        );
    }

    #[test]
    fn part_1_sample() {
        let solver = Day02 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_02/sample.txt"));
        assert_eq!(result, Outcome::U64(1227775554));
    }

    // #[test]
    // fn part_2_sample() {
    //     let solver = Day02 {};
    //     let result = solver.run_part_2(PathBuf::from("tests/day_02/sample.txt"));
    //     assert_eq!(result, Outcome::U64(4174379265));
    // }
}
