use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Possible outcomes for a daily solver (either a number or a string directly)
#[derive(PartialEq, Debug)]
pub enum Outcome {
    Number(i32),
    Text(String),
}

/// Base behavior of the daily solutions
pub trait Solution {
    fn run(&self, input_file: PathBuf) -> Outcome;

    fn get_file_reader(&self, input_file: PathBuf) -> BufReader<File> {
        let file = File::open(input_file).unwrap();
        BufReader::new(file)
    }
}
