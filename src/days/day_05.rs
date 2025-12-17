use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::path::PathBuf;

pub struct Day05;

impl Solution for Day05 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let mut lines = self.get_file_reader(input_file).lines();
        let ranges = self.make_ranges(lines.by_ref());
        let count = lines
            .map(|line| line.unwrap().parse::<u64>().unwrap())
            .filter(|number| ranges.iter().any(|range| range.contains(number)))
            .count();

        Outcome::I32(count as i32)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d5p2"))
    }
}

impl Day05 {
    /// Get ranges from the first part of the input file
    fn make_ranges(
        &self,
        lines: &mut std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Vec<RangeInclusive<u64>> {
        lines
            .take_while(|line| !line.as_ref().unwrap().is_empty())
            .map(|line| self.get_range_from_line(&line.unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let solver = Day05 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_05/sample.txt"));
        assert_eq!(result, Outcome::I32(3));
    }

    #[test]
    fn part_2_sample() {
        let solver = Day05 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_05/sample.txt"));
        assert_eq!(result, Outcome::I32(14));
    }
}
