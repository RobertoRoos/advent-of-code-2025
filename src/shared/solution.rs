use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::path::PathBuf;

/// Possible outcomes for a daily solver (either a number or a string directly)
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum Outcome {
    I32(i32),
    U64(u64),
    Text(String),
}

/// Implement string conversion for our general `Outcome` enum
impl Display for Outcome {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Outcome::I32(n) => write!(f, "{}", n),
            Outcome::U64(n) => write!(f, "{}", n),
            Outcome::Text(txt) => write!(f, "{}", txt),
        }
    }
}

/// Base behavior of the daily solutions
pub trait Solution {
    /// Helper initialize method - default implementation is empty
    fn init(&self) {}

    /// Main run method, it just picks the right solution method
    fn run(&self, input_file: PathBuf, part: u8) -> Outcome {
        self.init(); // Helper for future implementations for shared logic across parts

        match part {
            1 => self.run_part_1(input_file),
            2 => self.run_part_2(input_file),
            _ => panic!("Cannot do anything with part {}", part), // Also validated by CLI
        }
    }

    /// Solution for part 1 (must be implemented)
    fn run_part_1(&self, input_file: PathBuf) -> Outcome;

    /// Solution for part 2 (must be implemented)
    fn run_part_2(&self, input_file: PathBuf) -> Outcome;

    /// Create a reader object for the input file
    fn get_file_reader(&self, input_file: PathBuf) -> BufReader<File> {
        let file = File::open(input_file).unwrap();
        BufReader::new(file)
    }

    /// Make a Range object from a string like "11-22" (inclusive start and end)
    fn get_range_from_line(&self, line: &str) -> RangeInclusive<u64> {
        let mut split = line.split("-");
        split.next().unwrap().parse().unwrap()..=split.next().unwrap().parse().unwrap()
    }
}
