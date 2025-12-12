use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Possible outcomes for a daily solver (either a number or a string directly)
#[derive(PartialEq, Debug)]
pub enum Outcome {
    Number(i32),
    Text(String),
}

/// Which part of the day
#[derive(PartialEq, Debug)]
pub enum Part {
    Part1,
    Part2,
}

impl TryFrom<u8> for Part {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(Part::Part1),
            2 => Ok(Part::Part2),
            _ => Err(()),
        }
    }
}

/// Base behavior of the daily solutions
pub trait Solution {
    fn run(&self, input_file: PathBuf, part: Part) -> Outcome;

    fn get_file_reader(&self, input_file: PathBuf) -> BufReader<File> {
        let file = File::open(input_file).unwrap();
        BufReader::new(file)
    }
}
