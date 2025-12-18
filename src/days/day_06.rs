use crate::shared::{Outcome, Solution};
use std::io::BufRead;
use std::path::PathBuf;

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

        let sum = numbers
            .iter()
            .enumerate()
            .map(|(column_idx, column)| match operators[column_idx] {
                '+' => column.iter().sum::<u64>(),
                '*' => column.iter().product::<u64>(),
                _ => panic!("Unrecognized character"),
            })
            .sum();

        Outcome::U64(sum)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d6p2"))
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
}
