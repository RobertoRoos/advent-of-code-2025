use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::ops::Range;
use std::path::PathBuf;
use std::str::Chars;

pub struct Day06;

impl Solution for Day06 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        // Matrix of numbers, organized like numbers[<column>][<row>]
        let mut numbers: Vec<Vec<u64>> = Vec::new();

        // Operator for a column:
        let mut operators: Vec<char> = Vec::new();

        // Do old-fashioned loop because the if-else is hard to manage in an expression
        for line in self.get_file_reader(input_file).lines() {
            let line = line.unwrap();
            let row: Result<Vec<_>, _> = line.split_whitespace().map(str::parse::<u64>).collect();
            if let Ok(row) = row {
                if numbers.is_empty() {
                    numbers.resize(row.len(), Vec::new());
                }
                for (column_idx, value) in row.into_iter().enumerate() {
                    numbers[column_idx].push(value);
                }
            } else {
                operators = line
                    .split_whitespace()
                    .map(|bit| bit.chars().next().unwrap())
                    .collect();
            }
        }

        Outcome::U64(Self::sum_column_operations(&numbers, &operators))
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let mut lines: Vec<String> = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| line.unwrap())
            .collect();
        let max_line_length: usize = lines.iter().map(|line| line.chars().count()).max().unwrap();

        let operators_line = lines.pop().unwrap();

        // Build a list of the character ranges for colum
        let mut column_ranges: Vec<Range<usize>> = Vec::new(); // Note: characters, not bytes!!!
        let mut operators: Vec<char> = Vec::new(); // List of operators per column
        let mut last_idx = None;
        for (column_idx, c) in operators_line.chars().enumerate() {
            if c != ' ' {
                operators.push(c);
                if let Some(last_idx) = last_idx {
                    column_ranges.push(last_idx..(column_idx - 1));
                }
                last_idx = Some(column_idx);
            }
        }
        if let Some(last_idx) = last_idx {
            column_ranges.push(last_idx..max_line_length); // Last range will be missing
        }

        // Now build the matrix of numbers:
        let mut numbers: Vec<Vec<u64>> = vec![Vec::new(); operators.len()];

        // Track (mutable!) iterators to each of the lines.
        // We will now advance through all lines, character by character.
        let mut line_iters: Vec<Chars> = lines.iter().map(|line| line.chars()).collect();

        for (column_idx, slice) in column_ranges.into_iter().enumerate() {
            // Foreach character belonging to the next column, take the next character of each of
            // the lines and combine them into strings:
            for _ in slice {
                let num_str: String = line_iters
                    .iter_mut()
                    .map(|it| it.next().unwrap_or(' '))
                    .collect();
                let num: u64 = num_str.trim().parse().unwrap();
                numbers[column_idx].push(num);
            }
            // Advance iterators past empty spaces:
            for it in &mut line_iters {
                it.next();
            }
        }

        Outcome::U64(Self::sum_column_operations(&numbers, &operators))
    }
}

impl Day06 {
    /// Perform a repeated math operation on a column
    fn column_operation(column: &[u64], operator: char) -> u64 {
        match operator {
            '+' => column.iter().sum::<u64>(),
            '*' => column.iter().product::<u64>(),
            _ => panic!("Unrecognized character"),
        }
    }

    /// Sum operations on all columns at once
    fn sum_column_operations(columns: &[Vec<u64>], operators: &[char]) -> u64 {
        columns
            .iter()
            .enumerate()
            .map(|(column_idx, column)| Self::column_operation(column, operators[column_idx]))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let solver = Day06 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_06/sample.txt"));
        assert_eq!(result, Outcome::U64(4_277_556));
    }

    #[test]
    fn part_2_sample() {
        let solver = Day06 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_06/sample.txt"));
        assert_eq!(result, Outcome::U64(3_263_827));
    }
}
