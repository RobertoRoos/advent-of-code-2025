/// Base behavior of the daily solutions
pub trait Solution {
    fn run(&self) -> String;
}
