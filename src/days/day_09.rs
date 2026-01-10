use crate::shared::{Grid, Outcome, RowCol, Solution};
use std::cmp::max;
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day09;

impl Solution for Day09 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let mut grid = Grid::default();

        for line in self.get_file_reader(input_file).lines() {
            let coord = RowCol::from(line.unwrap().as_str());
            grid.add_item(coord, 'x');
        }

        let mut biggest: u64 = 0;

        // Check all combinations of points:
        for (i1, p1) in grid.items.keys().enumerate() {
            for p2 in grid.items.keys().skip(i1 + 1) {
                let this_size = u64::try_from((p2.row - p1.row).abs() + 1).unwrap()
                    * u64::try_from((p2.col - p1.col).abs() + 1).unwrap();
                biggest = max(biggest, this_size);
            }
        }

        Outcome::U64(biggest)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d9p2"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let solver = Day09 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_09/sample.txt"));
        assert_eq!(result, Outcome::U64(50));
    }
}
