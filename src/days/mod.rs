mod day_01;
mod day_02;

pub use day_01::Day01;
pub use day_02::Day02;

use crate::shared::Solution;

/// Return an instance of a solver based on the day number
pub fn get_solver(number: u8) -> Box<dyn Solution> {
    match number {
        1 => Box::new(Day01 {}),
        2 => Box::new(Day02 {}),
        _ => panic!("Invalid number for <day>"), // Also covered by CLI validator
    }
}
