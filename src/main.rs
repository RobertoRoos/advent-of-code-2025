mod days;
mod shared;

use clap::Parser;
use std::path::PathBuf;

use days::{Day01, Day02};
use shared::Solution;

/// Advent of code 2025 solutions
///
/// This single executable can be run for each day
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day of the challenge
    #[arg(value_parser=clap::value_parser!(u8).range(1..=2))]
    day: u8,

    /// Path to the input.txt file
    input: PathBuf,

    /// Enable printing of the timing statistic
    #[arg(short, long, default_value = "false")]
    timing: bool,

    /// Enable to print intermediate debugging info
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Instantiate the solver for the selected day
    let solver: Box<dyn Solution>;
    solver = match args.day {
        1 => Box::new(Day01 {}),
        2 => Box::new(Day02 {}),
        _ => panic!("Invalid number for <day>"), // Also covered by CLI validator
    };

    let result = solver.run();

    println!("{}", result);
}
