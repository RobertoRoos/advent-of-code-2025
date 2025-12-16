mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;

pub use day_01::Day01;
pub use day_02::Day02;
pub use day_03::Day03;
pub use day_04::Day04;
pub use day_05::Day05;
pub use day_06::Day06;
pub use day_07::Day07;
pub use day_08::Day08;
pub use day_09::Day09;
pub use day_10::Day10;
pub use day_11::Day11;
pub use day_12::Day12;

use crate::shared::Solution;

/// Return an instance of a solver based on the day number
pub fn get_solver(number: u8) -> Box<dyn Solution> {
    match number {
        1 => Box::new(Day01 {}),
        2 => Box::new(Day02 {}),
        3 => Box::new(Day03 {}),
        4 => Box::new(Day04 {}),
        5 => Box::new(Day05 {}),
        6 => Box::new(Day06 {}),
        7 => Box::new(Day07 {}),
        8 => Box::new(Day08 {}),
        9 => Box::new(Day09 {}),
        10 => Box::new(Day10 {}),
        11 => Box::new(Day11 {}),
        12 => Box::new(Day12 {}),
        _ => panic!("Invalid number for <day>"), // Also covered by CLI validator
    }
}
