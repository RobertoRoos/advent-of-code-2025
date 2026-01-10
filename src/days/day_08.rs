use crate::shared::{Outcome, Solution};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;
use std::io::BufRead;
use std::path::PathBuf;

type Point = [i32; 3];
type Points = Vec<Point>;
type Pair = (usize, usize);

pub struct Day08 {
    pub limit: usize,
}

impl Solution for Day08 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let junctions: Points = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.split(',')
                    .map(|part| part.parse().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into()
                    .unwrap()
            })
            .collect();

        // Track circuits (= groups of connected junctions):
        let mut circuits: Vec<HashSet<usize>> = junctions
            .iter()
            .enumerate()
            .map(|(idx, _)| HashSet::from([idx]))
            .collect();

        let distances = Self::find_closest_pairs(&junctions);

        // Get the shortest distances:
        for (pair, _dist) in distances.iter().take(self.limit) {
            let idx_1 = Self::find_in_sets(max(&pair.0, &pair.1), &circuits).unwrap();
            let idx_2 = Self::find_in_sets(min(&pair.0, &pair.1), &circuits).unwrap();

            let moving: Vec<usize> = circuits[idx_2].drain().collect();
            circuits[idx_1].extend(moving);
        }

        // Find the largest three circuits to multiply the size of:
        let mut circuit_sizes: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
        circuit_sizes.sort_by(|a, b| b.cmp(a));

        let result = circuit_sizes.iter().take(3).product();
        Outcome::U64(result)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d8p2"))
    }
}

impl Day08 {
    /// Make a list of pairs and their distances and sort them
    fn find_closest_pairs(points: &[Point]) -> Vec<(Pair, f64)> {
        let mut distances: Vec<(Pair, f64)> = Vec::new();

        for (i_a, point_a) in points.iter().enumerate() {
            for (i_b, point_b) in points[(i_a + 1)..].iter().enumerate() {
                let i_b = i_a + 1 + i_b;

                distances.push(((i_a, i_b), Self::distance(point_a, point_b)));
            }
        }

        // Sort distances:
        distances.sort_by(|(_, dist_a), (_, dist_b)| dist_a.partial_cmp(dist_b).unwrap());
        distances
    }

    /// Get distance between 2 points in 3D space
    fn distance(a: &Point, b: &Point) -> f64 {
        (f64::from(b[0] - a[0]).powi(2)
            + f64::from(b[1] - a[1]).powi(2)
            + f64::from(b[2] - a[2]).powi(2))
        .sqrt()
    }

    /// Find the index of the first hashset that contains the needle
    fn find_in_sets<T: Eq + Hash>(needle: &T, haystack: &[HashSet<T>]) -> Option<usize> {
        for (i, set) in haystack.iter().enumerate() {
            if set.contains(needle) {
                return Some(i);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_find_closest_pair() {
    //     let points = [
    //         [5, 5, 5],
    //         [1, 1, 1],
    //         [25, 25, 25],
    //         [45, 45, 45],
    //         [15, 15, 15],
    //         [2, 2, 2],
    //         [90, 90, 90],
    //         [190, 190, 190],
    //         [290, 290, 290],
    //         [390, 390, 390],
    //     ];
    //     let (pair, dist) = Day08::find_closest_pair(&points);
    //
    //     assert_eq!(pair, (1, 5));
    //     assert_eq!(dist, 3_f64.sqrt());
    // }

    // #[test]
    // fn test_find_closest_pair_from_sample() {
    //     let points = [
    //         [162, 817, 812],
    //         [57, 618, 57],
    //         [906, 360, 560],
    //         [592, 479, 940],
    //         [352, 342, 300],
    //         [466, 668, 158],
    //         [542, 29, 236],
    //         [431, 825, 988],
    //         [739, 650, 466],
    //         [52, 470, 668],
    //         [216, 146, 977],
    //         [819, 987, 18],
    //         [117, 168, 530],
    //         [805, 96, 715],
    //         [346, 949, 466],
    //         [970, 615, 88],
    //         [941, 993, 340],
    //         [862, 61, 35],
    //         [984, 92, 344],
    //         [425, 690, 689],
    //     ];
    //     let (pair, _) = Day08::find_closest_pair(&points);
    //     assert_eq!(pair, (0, 19));
    // }

    // #[test]
    // fn test_find_closest_pair_across() {
    //     let points_left = [(10, [1, 1, 1]), (11, [2, 2, 2]), (12, [3, 3, 3])];
    //     let points_right = [
    //         (13, [4, 4, 4]),
    //         (14, [5, 5, 5]),
    //         (15, [6, 6, 6]),
    //         (16, [7, 7, 7]),
    //     ];
    //     let result = Day08::find_closest_pair_across(&points_left, &points_right, 100_f64);
    //     assert!(result.is_some());
    //     assert_eq!(result.unwrap().0, (12, 13));
    // }

    #[test]
    fn test_part_1_sample() {
        let solver = Day08 { limit: 10 };
        let result = solver.run_part_1(PathBuf::from("tests/day_08/sample.txt"));
        assert_eq!(result, Outcome::U64(40));
    }
}
