use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// Base behavior of the daily solutions
pub trait Solution {
    fn run(&self, input_file: PathBuf) -> String;

    fn get_file_reader(&self, input_file: PathBuf) -> BufReader<File> {
        let file = File::open(input_file).unwrap();
        BufReader::new(file)
    }
}
