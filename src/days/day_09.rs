use crate::shared::{Outcome, RowCol, Solution};
use std::cmp::{max, min};
use std::io::BufRead;
use std::path::PathBuf;

/// Abstraction for a straight line (horizontal or vertical)
#[derive(Copy, Clone)]
pub struct Line {
    p_nw: RowCol, // Top or left
    p_se: RowCol, // Bottom or right
    vertical: bool,
}

impl Line {
    /// Create new line
    ///
    /// Order of points provided doesn't matter
    pub fn new(a: RowCol, b: RowCol) -> Self {
        Self {
            p_nw: RowCol::new(min(a.row, b.row), min(a.col, b.col)),
            p_se: RowCol::new(max(a.row, b.row), max(a.col, b.col)),
            vertical: if a.row == b.row {
                false
            } else if a.col == b.col {
                true
            } else {
                panic!("Only straight lines allowed");
            },
        }
    }

    /// Create new from integers directly
    #[allow(dead_code)]
    pub fn new_by_coords(a_row: i32, a_col: i32, b_row: i32, b_col: i32) -> Self {
        Self::new(RowCol::new(a_row, a_col), RowCol::new(b_row, b_col))
    }
}

/// Rectangle abstraction, spanned by two points
pub struct Rectangle {
    corner_nw: RowCol, // Top-left
    corner_se: RowCol, // Bottom-right
}

impl Rectangle {
    /// Create a new rectangle (point order doesn't matter)
    pub fn new(p1: RowCol, p2: RowCol) -> Self {
        Self {
            corner_nw: RowCol::new(min(p1.row, p2.row), min(p1.col, p2.col)),
            corner_se: RowCol::new(max(p1.row, p2.row), max(p1.col, p2.col)),
        }
    }

    /// Return rectangle size (including corners)
    pub fn size(&self) -> u64 {
        u64::try_from(self.corner_se.row - self.corner_nw.row + 1).unwrap()
            * u64::try_from(self.corner_se.col - self.corner_nw.col + 1).unwrap()
    }

    /// Return true if rectangle is crossed by the given line
    ///
    /// The line is expected to be either vertical or horizontal!
    pub fn crossed_by_line(&self, line: &Line) -> bool {
        if line.vertical {
            (line.p_nw.col > self.corner_nw.col && line.p_nw.col < self.corner_se.col)
                && (line.p_nw.row < self.corner_se.row && line.p_se.row > self.corner_nw.row)
        } else {
            (line.p_nw.row > self.corner_nw.row && line.p_nw.row < self.corner_se.row)
                && (line.p_se.col > self.corner_nw.col && line.p_nw.col < self.corner_se.col)
        }
    }
}

pub struct Day09;

impl Solution for Day09 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let points = self.build_points(input_file);
        let mut biggest: u64 = 0;

        // Check all combinations of points:
        for (i1, p1) in points.iter().enumerate() {
            for p2 in points.iter().skip(i1 + 1) {
                let rect = Rectangle::new(*p1, *p2);
                biggest = max(biggest, rect.size());
            }
        }

        Outcome::U64(biggest)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let points = self.build_points(input_file);

        // Lists of all lines and the vertical ones in particular
        let mut lines: Vec<Line> = Vec::new();
        let mut lines_vertical: Vec<Line> = Vec::new();

        for (i1, &p1) in points.iter().enumerate() {
            let i2 = (i1 + 1) % points.len();
            let p2 = points[i2];
            let line = Line::new(p1, p2);
            lines.push(line);
            if line.vertical {
                lines_vertical.push(line);
            }
        }

        // Try gather all pairs of points we could make in the whole set, for trial rectangles:
        let mut biggest = 0;
        for (i1, &p1) in points.iter().enumerate() {
            for &p2 in points.iter().skip(i1) {
                let rect = Rectangle::new(p1, p2);
                let size = rect.size();
                if size <= biggest {
                    continue; // Don't bother checking anything else, this pair is useless
                }

                // To check if this rectangle falls within the perimeter made by `points`, we do
                // two checks:
                // i)  Ray test - determine if this rect is not outside the shape entirely
                // ii) Cross test - check if no lines of the perimeter cut through it

                if Self::count_ray_intersections(&rect, &lines_vertical).is_power_of_two() {
                    continue; // Skip this `rect`, it's outside the perimeter
                }

                if lines.iter().any(|line| rect.crossed_by_line(line)) {
                    continue; // At least one perimeter cuts though it, invalid
                }

                biggest = size;
            }
        }

        Outcome::U64(biggest)
    }
}

impl Day09 {
    /// Build the grid from the input file
    fn build_points(&self, input_file: PathBuf) -> Vec<RowCol> {
        self.get_file_reader(input_file)
            .lines()
            .map(|line| RowCol::from(line.unwrap().as_str()))
            .collect()
    }

    /// Count number of ray intersections from this rectangle to a set of perimeter lines
    fn count_ray_intersections(rect: &Rectangle, lines: &[Line]) -> u32 {
        // Our ray origin will effectively be `rect.corner_nw + RowCol(0.5, 0.5)`
        // And it will point to the right
        let origin = rect.corner_nw;

        let mut intersects: u32 = 0;

        for line in lines {
            if line.p_nw.col <= rect.corner_nw.col {
                continue; // Entirely to the left of the square
            }

            if origin.row >= line.p_nw.row && origin.row < line.p_se.row {
                intersects += 1;
            }
        }

        intersects
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

    #[test]
    fn test_rectangle_crossed_by_line() {
        let rect = Rectangle::new(RowCol::new(3, 1), RowCol::new(7, 9));

        // Horizontal:
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 0, 5, 4)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 2, 5, 7)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 8, 5, 11)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 0, 5, 11)));

        // Vertical:
        assert!(rect.crossed_by_line(&Line::new_by_coords(1, 3, 4, 3)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 3, 6, 3)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(5, 3, 12, 3)));
        assert!(rect.crossed_by_line(&Line::new_by_coords(1, 3, 12, 3)));
    }

    #[test]
    fn test_part_2_sample() {
        let solver = Day09 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_09/sample.txt"));
        assert_eq!(result, Outcome::U64(24));
    }
}
