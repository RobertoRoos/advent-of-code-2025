use crate::shared::{Outcome, Solution};
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
        let (_junctions, distances, mut circuits) = self.find_distances_and_circuits(input_file);

        // Get the shortest distances (up to limit):
        for (pair, _dist) in distances.iter().take(self.limit) {
            Self::merge_circuits(pair, &mut circuits);
        }

        // Find the largest three circuits to multiply the size of:
        let mut circuit_sizes: Vec<u64> = circuits.iter().map(|c| c.len() as u64).collect();
        circuit_sizes.sort_by(|a, b| b.cmp(a));

        let result = circuit_sizes.iter().take(3).product();
        Outcome::U64(result)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let (junctions, distances, mut circuits) = self.find_distances_and_circuits(input_file);

        // Get the shortest distances (up to limit):
        for (pair, _dist) in distances {
            Self::merge_circuits(&pair, &mut circuits);

            if circuits.len() == 1 {
                // Everything just became a single circuit!
                return Outcome::U64(
                    u64::try_from(junctions[pair.0][0]).unwrap()
                        * u64::try_from(junctions[pair.1][0]).unwrap(),
                );
            }
        }

        panic!("Failed to find solution");
    }
}

impl Day08 {
    fn find_distances_and_circuits(
        &self,
        input_file: PathBuf,
    ) -> (Points, Vec<(Pair, f64)>, Vec<HashSet<usize>>) {
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
        let circuits: Vec<HashSet<usize>> = junctions
            .iter()
            .enumerate()
            .map(|(idx, _)| HashSet::from([idx]))
            .collect();

        let distances = Self::find_closest_pairs(&junctions);

        (junctions, distances, circuits)
    }

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

    /// From a pair, combine the two corresponding circuits
    fn merge_circuits(pair: &Pair, circuits: &mut Vec<HashSet<usize>>) {
        let mut circuit_ids: Vec<usize> = [pair.0, pair.1]
            .iter()
            .map(|idx| Self::find_in_sets(idx, circuits).unwrap())
            .collect();

        if circuit_ids[0] == circuit_ids[1] {
            return; // Nothing changes
        }

        circuit_ids.sort_unstable(); // Sort, such that the lowest index won't be affected by the removal
        // of the higher index.

        let circuit_old = circuits.swap_remove(circuit_ids[1]); // Move out of the vector

        circuits[circuit_ids[0]].extend(circuit_old);
        // Move the elements into the first circuit, inside the vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_sample() {
        let solver = Day08 { limit: 10 };
        let result = solver.run_part_1(PathBuf::from("tests/day_08/sample.txt"));
        assert_eq!(result, Outcome::U64(40));
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day08 { limit: 10 };
        let result = solver.run_part_2(PathBuf::from("tests/day_08/sample.txt"));
        assert_eq!(result, Outcome::U64(25272));
    }
}
