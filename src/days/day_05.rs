use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::path::PathBuf;

type Range = RangeInclusive<u64>;

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

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let mut lines = self.get_file_reader(input_file).lines();
        let new_ranges = self.make_ranges(lines.by_ref());

        let mut combined_ranges: Vec<Range> = Vec::new();

        for range in new_ranges {
            combined_ranges = Self::combine_inclusive_range(combined_ranges, range);
        }

        Outcome::U64(Self::count_ranges_list(combined_ranges))
    }
}

impl Day05 {
    /// Get ranges from the first part of the input file
    fn make_ranges(
        &self,
        lines: &mut std::io::Lines<std::io::BufReader<std::fs::File>>,
    ) -> Vec<Range> {
        lines
            .take_while(|line| !line.as_ref().unwrap().is_empty())
            .map(|line| self.get_range_from_line(&line.unwrap()))
            .collect()
    }

    /// Merge a set of existing ranges with a new one
    fn combine_inclusive_range(mut ranges: Vec<Range>, new_range: Range) -> Vec<Range> {
        if new_range.end() < new_range.start() {
            return ranges; // Invalid or empty range
        }
        for (this_i, this_range) in ranges.iter().enumerate() {
            if this_range.contains(new_range.start()) && this_range.contains(new_range.end()) {
                // `new_range` falls entirely within `this_range`, so no need to do anything
                return ranges;
            } else if *new_range.end() >= this_range.start() - 1
                && *new_range.start() <= this_range.end() + 1
            {
                // We have any amount of overlap of `new_range` and `this_range`, but not full
                // Ranges that are directly adjacent will also be merged together
                // We replace `new_range` by a sum of `new_range` and `this_range` and remove
                // `this_range` from the list of already processed ranges.
                let new_new_range = *this_range.start().min(new_range.start())
                    ..=*this_range.end().max(new_range.end());
                // Our modified `new_range` could overlap with other existing ranges, hence we
                // put it though the loops again.
                ranges.remove(this_i);
                return Self::combine_inclusive_range(ranges, new_new_range);
            }
        }
        ranges.push(new_range);
        ranges
    }

    /// Count all the possible values in a list of ranges
    fn count_ranges_list(ranges: Vec<Range>) -> u64 {
        ranges.into_iter().map(|r| r.end() - r.start() + 1).sum()
        // Avoid `count()` as it will perform the actual iteration, which we don't need
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let solver = Day05 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_05/sample.txt"));
        assert_eq!(result, Outcome::I32(3));
    }

    #[allow(clippy::reversed_empty_ranges)]
    #[test]
    fn test_combine_inclusive_range() {
        let mut ranges = Vec::new();
        ranges = Day05::combine_inclusive_range(ranges, 3..=8); // First
        assert_eq!(ranges, vec![3..=8]);

        ranges = Day05::combine_inclusive_range(ranges, 1..=10); // Full upgrade
        assert_eq!(ranges, vec![1..=10]);

        ranges = Day05::combine_inclusive_range(ranges, 16..=18); // No overlap
        assert_eq!(ranges, vec![1..=10, 16..=18]);

        ranges = Day05::combine_inclusive_range(ranges, 16..=18); // Exact duplicate
        assert_eq!(ranges, vec![1..=10, 16..=18]);

        ranges = Day05::combine_inclusive_range(ranges, 3..=5); // Full overlap
        assert_eq!(ranges, vec![1..=10, 16..=18]);

        ranges = Day05::combine_inclusive_range(ranges, 17..=20); // Partial overlap
        assert_eq!(ranges, vec![1..=10, 16..=20]);

        ranges = Day05::combine_inclusive_range(ranges, 5..=19); // Dual overlap
        assert_eq!(ranges, vec![1..=20]);

        // Tiple overlap:
        ranges = Day05::combine_inclusive_range(ranges, 25..=30);
        ranges = Day05::combine_inclusive_range(ranges, 35..=40);
        assert_eq!(ranges, vec![1..=20, 25..=30, 35..=40]);
        ranges = Day05::combine_inclusive_range(ranges, 18..=35);
        assert_eq!(ranges, vec![1..=40]);

        ranges = Day05::combine_inclusive_range(ranges, 41..=41); // Adjacent
        assert_eq!(ranges, vec![1..=41]);

        ranges = Day05::combine_inclusive_range(ranges, 100..=90); // Negative range
        assert_eq!(ranges, vec![1..=41]);
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day05 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_05/sample.txt"));
        assert_eq!(result, Outcome::U64(14));
    }

    #[test]
    fn test_part_2_custom() {
        let solver = Day05 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_05/custom.txt"));
        // In our files, the optimized ranges are:
        // 100-170
        // 5-35
        // 990-1100
        assert_eq!(
            result,
            Outcome::U64(170 - 100 + 1 + 35 - 5 + 1 + 1100 - 990 + 1)
        );
    }

    #[allow(clippy::identity_op)]
    #[test]
    fn test_part_2_big() {
        let solver = Day05 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_05/big.txt"));
        assert_eq!(
            result,
            Outcome::U64(
                213743501161200 - 213743501161200 + 1 + 219471291456959 - 213743501161201 + 1
            )
        );
    }
}
