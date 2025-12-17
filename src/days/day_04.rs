use crate::shared::{Grid, Outcome, RowCol, Solution};
use std::io::BufRead;
use std::path::PathBuf;

pub struct Day04;

/// 8 possible steps for surrounding positions
static SURROUNDING: [RowCol; 8] = [
    RowCol { row: -1, col: 0 },
    RowCol { row: -1, col: 1 },
    RowCol { row: 0, col: 1 },
    RowCol { row: 1, col: 1 },
    RowCol { row: 1, col: 0 },
    RowCol { row: 1, col: -1 },
    RowCol { row: 0, col: -1 },
    RowCol { row: -1, col: -1 },
];

impl Solution for Day04 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let grid = Grid::from(self.get_file_reader(input_file).lines());
        let count = self.find_accessible_locations(&grid).count();
        Outcome::U64(count.try_into().unwrap())
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let mut grid = Grid::from(self.get_file_reader(input_file).lines());
        let mut removed = 0;

        loop {
            let to_be_removed: Vec<RowCol> = self.find_accessible_locations(&grid).collect();
            if to_be_removed.is_empty() {
                break;
            }
            for loc in to_be_removed.into_iter() {
                grid.remove_item(&loc);
                removed += 1;
            }
        }
        Outcome::U64(removed)
    }
}

impl Day04 {
    fn find_accessible_locations(&self, grid: &Grid) -> impl Iterator<Item = RowCol> {
        // Count number of items in the grid that have less than 4 surrounding items
        grid.items
            .iter()
            .filter(|&(&loc, _)| {
                SURROUNDING
                    .iter()
                    .filter(|&&step| grid.items.contains_key(&(loc + step)))
                    .count()
                    < 4
            })
            .map(|(&loc, _)| loc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let solver = Day04 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_04/sample.txt"));
        assert_eq!(result, Outcome::U64(13));
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day04 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_04/sample.txt"));
        assert_eq!(result, Outcome::U64(43));
    }
}
