use crate::shared::{Grid, Outcome, RowCol, Solution};
use std::collections::HashMap;
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
        Outcome::I32(count as i32)
    }

    // fn run_part_2(&self, input_file: PathBuf) -> Outcome {
    //     let mut grid = Grid::from(self.get_file_reader(input_file).lines());
    //     let mut removed: i32 = 0;
    //
    //     loop {
    //         let to_be_removed: Vec<RowCol> = self.find_accessible_locations(&grid).collect();
    //         if to_be_removed.is_empty() {
    //             break;
    //         }
    //         for loc in to_be_removed.into_iter() {
    //             grid.remove_item(&loc);
    //             removed += 1;
    //         }
    //     }
    //     Outcome::I32(removed)
    // }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let grid = Grid::from(self.get_file_reader(input_file).lines());
        let mut removed: i32 = 0;

        // Look-up table of each item and the items it's adjacent too (i.e. the other items
        // it might block from being accessed.
        let mut obstacle_for: HashMap<RowCol, Vec<RowCol>> = HashMap::new();

        // Count how many obstacles (0 to 8) a location currently has around it
        let mut obstacle_count: HashMap<RowCol, u8> = HashMap::new();

        // For each obstacle, loop over the 8 adjacent spots
        for (&obs_loc, _) in grid.items.iter() {
            for &nb_step in SURROUNDING.iter() {
                let nb_loc = obs_loc + nb_step;
                if grid.items.contains_key(&nb_loc) {
                    obstacle_for.entry(obs_loc).or_default().push(nb_loc);
                    *obstacle_count.entry(nb_loc).or_insert(0) += 1;
                }
            }
        }

        // Now we start removing the items that have less than 4 obstacles around them
        // With the map we just made we can quickly find which other items might now have freed up
        loop {
            let next_removed = obstacle_count
                .iter()
                .filter_map(|(loc, cnt)| {
                    if *cnt < 4 {
                        Some(*loc)
                        // Force a clone here such that no borrow to `obstacle_count` persists
                    } else {
                        None
                    }
                })
                .next();

            if let Some(loc) = next_removed {
                for nb_loc in obstacle_for[&loc].iter() {
                    // With `loc` removed, some other items now have one less obstacle
                    if let Some(cnt) = obstacle_count.get_mut(nb_loc) {
                        *cnt -= 1;
                    }
                }
                obstacle_count.remove(&loc);
                obstacle_for.remove(&loc);
                removed += 1;
            } else {
                break; // Cannot remove any others
            }
        }

        Outcome::I32(removed)
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
        assert_eq!(result, Outcome::I32(13));
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day04 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_04/sample.txt"));
        assert_eq!(result, Outcome::I32(43));
    }
}
