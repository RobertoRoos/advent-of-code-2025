use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Lines;
use std::ops::{Add, Range};

/// Directions between 2D coordinates
#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "UP"),
            Direction::Right => write!(f, "RIGHT"),
            Direction::Down => write!(f, "DOWN"),
            Direction::Left => write!(f, "LEFT"),
        }
    }
}

static DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

type I = i32;

/// 2D coordinate through row and column
///
/// (0,0) is always the top-left.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct RowCol {
    pub row: I,
    pub col: I,
    // `row` and `col` can only be positive, but making them signed integers prevents
    // doing a lot of casting.
}

impl RowCol {
    pub fn new(row: I, col: I) -> Self {
        Self { row, col }
    }

    pub fn default() -> Self {
        Self { row: 0, col: 0 }
    }

    /// Get a new coordinate of a neighboring location
    pub fn step(self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self::new(self.row - 1, self.col),
            Direction::Right => Self::new(self.row, self.col + 1),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col - 1),
        }
    }

    /// Loop over the 4 neighboring locations
    pub fn neighbours(&self) -> impl Iterator<Item = RowCol> {
        DIRECTIONS.iter().map(|dir| self.step(dir))
    }
}

impl From<&str> for RowCol {
    fn from(line: &str) -> Self {
        let mut it = line.split(',');

        let part_to_num = |txt: Option<&str>| txt.unwrap().parse::<I>().unwrap();

        Self {
            row: part_to_num(it.next()),
            col: part_to_num(it.next()),
        }
    }
}

impl Display for RowCol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row: {}, col: {}", self.row, self.col)
    }
}

impl Add for RowCol {
    type Output = RowCol;

    fn add(self, other: Self) -> Self {
        Self::new(self.row + other.row, self.col + other.col)
    }
}

/// Helper object to make grid positions iterable
#[derive(Debug)]
pub struct GridIterator<'a> {
    loc: Option<RowCol>,
    grid: &'a Grid,
}

impl Iterator for GridIterator<'_> {
    type Item = RowCol;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.loc {
            None => self.loc = Some(RowCol::default()),
            Some(loc) => {
                if loc.col < self.grid.cols - 1 {
                    loc.col += 1;
                } else if loc.row < self.grid.rows - 1 {
                    loc.col = 0;
                    loc.row += 1;
                } else {
                    return None;
                }
            }
        }

        self.loc
    }
}

/// Range of rows and columns in 2D, with or without items
#[derive(Debug)]
pub struct Grid {
    pub rows: i32,
    pub cols: i32,
    pub items: HashMap<RowCol, char>, // Grid items are marked only with a single character
}

#[allow(dead_code)]
impl Grid {
    pub fn new(rows: I, cols: I) -> Self {
        Self {
            rows,
            cols,
            items: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }

    pub fn range_rows(&self) -> Range<I> {
        0..self.rows
    }

    pub fn range_cols(&self) -> Range<I> {
        0..self.cols
    }

    /// Iterable over all coordinates in this grid, left-to-right and then top-to-bottom
    pub fn range(&self) -> GridIterator<'_> {
        GridIterator {
            loc: None,
            grid: self,
        }
    }

    /// Insert a new item into the grid
    pub fn add_item(&mut self, loc: RowCol, symbol: char) {
        assert!(
            !self.items.contains_key(&loc),
            "Item {loc} is already filled in the grid"
        );
        if loc.row >= self.rows {
            self.rows = loc.row + 1;
        }
        if loc.col >= self.cols {
            self.cols = loc.col + 1;
        }
        self.items.insert(loc, symbol);
    }

    /// Remove an item from this grid
    pub fn remove_item(&mut self, loc: RowCol) {
        self.items.remove(&loc);
    }

    /// Return the first item with a given symbol
    pub fn get_item_by_symbol(&self, symbol: char) -> Result<RowCol, String> {
        for (&loc, &c) in &self.items {
            if c == symbol {
                return Ok(loc);
            }
        }
        Err(format!("Failed to find {symbol}"))
    }

    /// Get neighboring items in the grid to a given position
    pub fn neighbouring_items(&self, loc: &RowCol) -> impl Iterator<Item = (RowCol, char)> {
        loc.neighbours().filter_map(|n_loc| {
            let item = self.items.get(&n_loc);
            match item {
                Some(&symbol) => Some((n_loc, symbol)),
                _ => None,
            }
        })
    }

    /// Parse a line into this grid, typically from an input.txt
    fn add_row_from_text(&mut self, line: &str) {
        let row = self.rows;
        self.rows += 1;
        for (col, symbol) in line.chars().enumerate() {
            let col: I = col.try_into().unwrap();
            if col >= self.cols {
                self.cols += 1; // Also expand the column range for white-space
            }
            if symbol == '.' {
                continue; // Dot implies empty space
            }
            let loc = RowCol::new(row, col);
            self.add_item(loc, symbol);
        }
    }

    /// Print the grid to the command line
    pub fn print(&self) {
        for row in self.range_rows() {
            for col in self.range_cols() {
                let loc = RowCol::new(row, col);
                let c = self.items.get(&loc).unwrap_or(&'.');
                print!("{c}");
            }
            println!();
        }
    }
}

impl<T: std::io::BufRead> From<Lines<T>> for Grid {
    fn from(lines: Lines<T>) -> Self {
        let mut grid = Grid::default();

        for line in lines {
            grid.add_row_from_text(&line.unwrap());
        }

        grid
    }
}

#[cfg(test)]
mod tests_row_col {
    use super::*;

    #[test]
    fn test_row_col() {
        let p1 = RowCol { row: 1, col: 5 };
        let p2 = RowCol::new(1, 5);
        let mut p3 = RowCol::default();
        p3.row = 1;
        p3.col = 5;
        assert_eq!(p1, p2);
        assert_eq!(p1, p3);
    }

    #[test]
    fn test_row_col_neighbours() {
        let loc = RowCol::new(3, 1);
        let mut list: Vec<RowCol> = Vec::new();
        for next in loc.neighbours() {
            list.push(next);
        }
        assert_eq!(
            list,
            vec![
                RowCol::new(2, 1),
                RowCol::new(3, 2),
                RowCol::new(4, 1),
                RowCol::new(3, 0),
            ]
        );
    }

    #[test]
    fn test_row_col_sum() {
        assert_eq!(RowCol::new(1, 2) + RowCol::new(3, 4), RowCol::new(4, 6));
    }
}

#[cfg(test)]
mod tests_grid {
    use super::*;

    #[test]
    fn test_grid_iterator() {
        let grid = Grid::new(2, 3);

        let expected = vec![
            RowCol::new(0, 0),
            RowCol::new(0, 1),
            RowCol::new(0, 2),
            RowCol::new(1, 0),
            RowCol::new(1, 1),
            RowCol::new(1, 2),
        ];
        let mut list: Vec<RowCol> = Vec::new();

        for loc in grid.range() {
            list.push(loc);
        }
        assert_eq!(list, expected);
    }

    #[test]
    fn test_grid_neighbours() {
        let mut grid = Grid::default();
        grid.add_item(RowCol::new(3, 2), 'x');
        grid.add_item(RowCol::new(3, 3), 'y');
        grid.add_item(RowCol::new(4, 2), 'z');
        grid.add_item(RowCol::new(5, 2), 'w');

        let mut result: Vec<(RowCol, char)> = Vec::new();
        for other in grid.neighbouring_items(&RowCol::new(3, 2)) {
            result.push(other);
        }
        assert_eq!(
            result,
            vec![(RowCol::new(3, 3), 'y'), (RowCol::new(4, 2), 'z')]
        );
    }
}
