mod days;
mod shared;

use crate::shared::{Outcome, Part};
use clap::Parser;
use days::get_solver;
use std::{path, path::PathBuf};

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
    #[arg(short, long, default_value = "default")]
    input: PathBuf,

    /// Whether to run part 1 or part 2
    #[arg(short, long, default_value = "1", value_parser=clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Enable printing of the timing statistic
    #[arg(short, long, default_value = "false")]
    timing: bool,

    /// Enable to print intermediate debugging info
    #[arg(short, long, default_value = "false")]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // Resolve input path
    let input_file: PathBuf = if args.input.to_str() == Some("default") {
        PathBuf::from(format!("./inputs/day_{:02}.txt", args.day))
    } else {
        args.input
    };
    let input_file = path::absolute(&input_file).unwrap();

    // Instantiate the solver for the selected day
    let solver = get_solver(args.day);

    let part = Part::try_from(args.part).unwrap();
    
    let result = solver.run(input_file, part);

    match result {
        Outcome::Number(n) => println!("{}", n),
        Outcome::Text(t) => println!("{}", t),
    }
}
