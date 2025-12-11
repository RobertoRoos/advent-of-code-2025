use std::error::Error;

/// Base behavior of the daily solutions
pub trait Solution {
    fn run(&self) -> Result<String, Box<dyn Error>>;
}
