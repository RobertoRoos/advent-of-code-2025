use std::collections::{HashMap, HashSet};
use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::path::PathBuf;

type Point = [i16; 3];
type IdxPoint = (usize, Point);
type Points = Vec<Point>;
type Pair = (usize, usize);

pub struct Day08;

impl Solution for Day08 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let junctions: Points = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| {
                let line = line.unwrap();
                line.split(',')
                    .map(|part| part.parse().unwrap())
                    .collect::<Vec<i16>>()
                    .try_into()
                    .unwrap()
            })
            .collect();

        // Track circuits (= groups of connected junctions):
        let mut circuits: Vec<HashSet<usize>> = Vec::new();

        // Now connect the shortest junctions a bunch of times:
        for _ in 0..Self::LIMIT {
            let (pair, _) = Self::find_closest_pair(&junctions);

            let mut circuit = None;

            for this_circuit in circuits.iter_mut() {
                if this_circuit.contains(&pair.0) || this_circuit.contains(&pair.1) {
                    circuit = Some(this_circuit)
                }
            }
            if circuit.is_none() {
                let new_circuit = HashSet::new();
                circuits.push(new_circuit);
                circuit = Some(new_circuit);
            }
            let Some(circuit) = circuit;

            circuit.insert(pair.0);
            circuit.insert(pair.1); // Make sure both IDs are in there
        }

        let (pair, _dist) = Self::find_closest_pair(&junctions);

        dbg!(junctions[pair.0], junctions[pair.1]);

        Outcome::Text(String::from("d8p1"))
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d8p2"))
    }
}

impl Day08 {

    const LIMIT: u16 = 1_000;

    /// Find the pair of points in a cloud that are closest
    ///
    /// Returns the two indices, as well as the final distance.
    ///
    /// See https://en.wikipedia.org/wiki/Closest_pair_of_points_problem
    /// See https://codelucky.com/closest-pair-problem
    fn find_closest_pair(points: &[Point]) -> (Pair, f32) {
        let mut points_with_id: Vec<IdxPoint> = points.iter().copied().enumerate().collect();

        // Sort all points, by X, Y and Z coordinates respectively
        points_with_id.sort_by(|a, b| a.1.cmp(&b.1));

        Self::recurse_find_closest_pair(&points_with_id)
    }

    /// Private method for recursive part of `find_closest_pair`
    ///
    /// The points provided must be sorted!
    fn recurse_find_closest_pair(points_sorted: &[IdxPoint]) -> (Pair, f32) {
        assert!(points_sorted.len() >= 2, "Must have at least two points");
        if points_sorted.len() == 2 {
            return (
                (points_sorted[0].0, points_sorted[1].0),
                Self::distance(points_sorted[0].1, points_sorted[1].1),
            );
        } else if points_sorted.len() == 3 {
            // Ugly but mega-fast, hardcoded minimum distance of three points:
            let set = [
                (
                    (points_sorted[0].0, points_sorted[1].0),
                    Self::distance(points_sorted[0].1, points_sorted[1].1),
                ),
                (
                    (points_sorted[0].0, points_sorted[2].0),
                    Self::distance(points_sorted[0].1, points_sorted[2].1),
                ),
                (
                    (points_sorted[1].0, points_sorted[2].0),
                    Self::distance(points_sorted[1].1, points_sorted[2].1),
                ),
            ];
            return Self::min_with_tuple(set[0], Self::min_with_tuple(set[1], set[2]));
        }

        // Still 4 or more points left, do recursion:

        let m = points_sorted.len() / 2; // Split points in two halves

        // In each half, find their best distance and continue with the best of those results:
        let points_left = &points_sorted[..m];
        let points_right = &points_sorted[m..];
        let (best_pair, best_dist) = Self::min_with_tuple(
            Self::recurse_find_closest_pair(points_left),
            Self::recurse_find_closest_pair(points_right),
        );

        if let Some((best_pair_across, best_dist_across)) =
            Self::find_closest_pair_across(points_left, points_right, best_dist)
        {
            if best_dist_across < best_dist {
                return (best_pair_across, best_dist_across);
            }
        }

        (best_pair, best_dist)
    }

    /// Find the closest pair, each from two sets.
    ///
    /// This is aided a lot by the fact that we already know points are sorted over the X-axis.
    fn find_closest_pair_across(
        points_left: &[IdxPoint],
        points_right: &[IdxPoint],
        minimum_distance: f32,
    ) -> Option<(Pair, f32)> {
        let x_border = points_left.last().unwrap().1[0]; // X-coordinate of the furthest points on the left

        let minimum_distance = minimum_distance as i16;

        let mut distances: Vec<(Pair, f32)> = Vec::new();
        for point_left in points_left {
            if x_border - point_left.1[0] > minimum_distance {
                continue; // This point is so far on the left, the distance wouldn't be relevant
            }
            for point_right in points_right {
                if point_right.1[0] - x_border > minimum_distance {
                    break; // Similarly, this point (and others) are too far to the right anyway
                }

                distances.push((
                    (point_left.0, point_right.0),
                    Self::distance(point_left.1, point_right.1),
                ));
            }
        }

        if distances.is_empty() {
            None
        } else {
            Some(*distances.iter().min_by(|a, b| a.1.total_cmp(&b.1)).unwrap())
        }
    }

    /// Get distance between 2 points in 3D space
    fn distance(a: Point, b: Point) -> f32 {
        (f32::from(b[0] - a[0]).powi(2)
            + f32::from(b[1] - a[1]).powi(2)
            + f32::from(b[2] - a[2]).powi(2))
        .sqrt()
    }

    /// Helper to get a minimum with associated variable(s)
    fn min_with_tuple<T>(a: (T, f32), b: (T, f32)) -> (T, f32) {
        if a.1 < b.1 { a } else { b }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_closest_pair() {
        let points = [
            [5, 5, 5],
            [1, 1, 1],
            [25, 25, 25],
            [45, 45, 45],
            [15, 15, 15],
            [2, 2, 2],
            [90, 90, 90],
            [190, 190, 190],
            [290, 290, 290],
            [390, 390, 390],
        ];
        let (pair, dist) = Day08::find_closest_pair(&points);

        assert_eq!(pair, (1, 5));
        assert_eq!(dist, 3_f32.sqrt());
    }

    #[test]
    fn test_find_closest_pair_from_sample() {
        let points = [
            [162, 817, 812],
            [57, 618, 57],
            [906, 360, 560],
            [592, 479, 940],
            [352, 342, 300],
            [466, 668, 158],
            [542, 29, 236],
            [431, 825, 988],
            [739, 650, 466],
            [52, 470, 668],
            [216, 146, 977],
            [819, 987, 18],
            [117, 168, 530],
            [805, 96, 715],
            [346, 949, 466],
            [970, 615, 88],
            [941, 993, 340],
            [862, 61, 35],
            [984, 92, 344],
            [425, 690, 689],
        ];
        let (pair, _) = Day08::find_closest_pair(&points);
        assert_eq!(pair, (0, 19));
    }

    #[test]
    fn test_find_closest_pair_across() {
        let points_left = [(10, [1, 1, 1]), (11, [2, 2, 2]), (12, [3, 3, 3])];
        let points_right = [
            (13, [4, 4, 4]),
            (14, [5, 5, 5]),
            (15, [6, 6, 6]),
            (16, [7, 7, 7]),
        ];
        let result = Day08::find_closest_pair_across(&points_left, &points_right, 100_f32);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, (12, 13));
    }

    #[test]
    fn test_part_1_sample() {
        let solver = Day08 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_08/sample.txt"));
        assert_eq!(result, Outcome::U64(40));
    }
}
