use crate::shared::{Grid, Outcome, RowCol, Solution};
use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day07;

impl Solution for Day07 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let grid = Grid::from(self.get_file_reader(input_file).lines());

        let loc_start = grid.get_item_by_symbol('S').unwrap();

        // Keep a set of laser beam heads (a `set` to avoid having duplicates):
        let mut tips: HashSet<RowCol> = HashSet::from([loc_start]);

        let mut splits = 0;

        // Walk through the next rows in the grid:
        for row in loc_start.row..grid.rows {
            let mut next_tips: HashSet<RowCol> = HashSet::new();

            for tip in tips {
                let next = RowCol::new(row, tip.col);
                if grid.items.contains_key(&next) {
                    next_tips.insert(next + RowCol::new(0, -1));
                    next_tips.insert(next + RowCol::new(0, 1));
                    splits += 1;
                } else {
                    next_tips.insert(next);
                }
            }
            tips = next_tips; // Replace tips with the new list
        }

        Outcome::U64(splits)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let grid = Grid::from(self.get_file_reader(input_file).lines());

        let loc_start = grid.get_item_by_symbol('S').unwrap();

        // Keep a set of laser beam x-positions together with how many paths lead there, so far
        let mut tips: HashMap<i16, u64> = HashMap::from([(loc_start.col, 1)]);

        // Walk through the next rows in the grid:
        for row in (loc_start.row + 1)..grid.rows {
            let mut next_tips: HashMap<i16, u64> = HashMap::new();

            for (tip_col, path_count) in tips {
                let next = RowCol::new(row, tip_col);
                let steps = if grid.items.contains_key(&next) {
                    vec![-1, 1]
                } else {
                    vec![0]
                };
                for step in steps {
                    *next_tips.entry(tip_col + step).or_insert(0) += path_count;
                }
            }
            tips = next_tips; // Replace tips with the new list
        }

        Outcome::U64(tips.values().sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let solver = Day07 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_07/sample.txt"));
        assert_eq!(result, Outcome::U64(21));
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day07 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_07/sample.txt"));
        assert_eq!(result, Outcome::U64(40));
    }
}
