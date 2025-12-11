use crate::shared::Solution;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::path::PathBuf;

pub struct Day01 {
    input_file: PathBuf,
}

impl Day01 {
    pub fn new(input_file: PathBuf) -> Self {
        Self { input_file }
    }

    /// Convert a piece of string like `L8` to `-8`
    fn step_to_number(line: &str) -> i16 {
        let mut chars = line.chars();
        let dir = chars.next().expect("Line is empty");
        let mut step: i16 = chars.as_str().parse().expect("Failed to parse to int");
        if dir == 'L' { step *= -1; }
        step
    }
}

impl Solution for Day01 {

    /// Main method to get the solution
    fn run(&self) -> Result<String, Box<dyn Error>> {
        let file = File::open(&self.input_file)?;
        let reader = BufReader::new(file);

        let mut zeros_count = 0;
        reader
            .lines()
            .try_fold(50, |acc,line| {
                let step = Self::step_to_number(&line?);
                let next = (acc + step).rem_euclid(100);
                if next == 0 { zeros_count += 1 }
                Ok::<i16, Box<dyn Error>>(next)
            })?;

        Ok(format!("{}", zeros_count))
    }
}
