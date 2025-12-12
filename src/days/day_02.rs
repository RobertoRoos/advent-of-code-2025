use crate::shared::{Outcome, Solution, Part};
use std::path::PathBuf;

pub struct Day02;

impl Day02 {}

impl Solution for Day02 {
    fn run(&self, _input_file: PathBuf, _part: Part) -> Outcome {
        Outcome::Text(String::from("day 2"))
    }
}
