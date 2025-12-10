use crate::shared::Solution;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub struct Day01 {
    input_file: PathBuf,
}

impl Day01 {
    pub fn new(input_file: PathBuf) -> Self {
        Self { input_file }
    }
}

impl Solution for Day01 {
    fn run(&self) -> String {
        let file = File::open(&self.input_file).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut code: i16 = 50;
        let mut zeros = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let mut chars = line.chars();
            let dir = chars.next().unwrap();

            let mut step: i16 = chars.as_str().parse().unwrap();
            if dir == 'L' {
                step *= -1;
            }

            code = (code + step).rem_euclid(100);

            if code == 0 {
                zeros += 1;
            }
        }

        format!("{}", zeros)
    }
}
