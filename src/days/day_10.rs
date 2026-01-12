use crate::shared::{Outcome, Solution};
use std::collections::HashSet;
use std::io::BufRead;
use std::path::PathBuf;

type Lights = Vec<bool>;
type Button = Vec<usize>;

/// Machine abstraction (one input line)
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
}

impl From<&str> for Machine {
    /// Constructor from a line
    fn from(value: &str) -> Self {
        let mut it = value.split_whitespace();
        let lights = it
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect();

        let mut buttons = Vec::new();
        for part in it {
            let mut part_chars = part.chars();
            if part_chars.next().unwrap() == '{' {
                break;
            }
            part_chars.next_back();

            buttons.push(
                part_chars
                    .as_str()
                    .split(',')
                    .map(|c| c.parse().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }

        Self { lights, buttons }
    }
}

impl Machine {
    /// Switch lights based on button wiring
    pub fn switch_lights(lights: &Lights, button: &Button) -> Lights {
        let mut lights = lights.clone();
        for idx in button {
            lights[*idx] = !lights[*idx];
        }
        lights
    }

    /// Return minimal number of button presses
    pub fn minimum_button_presses(&self) -> u64 {
        // Sort of brute force all button options
        // One big optimization is to merge button-paths with identical outcomes
        // We don't need the actual button sequence, so just track a set of light-states, together
        // with a count of the total number of button presses.

        let mut states: HashSet<Lights> = HashSet::from([vec![false; self.lights.len()]]);
        let mut count = 0;
        loop {
            let mut new_states: HashSet<Lights> = HashSet::new();
            count += 1;
            for state in states.drain() {
                for button in &self.buttons {
                    let new_state = Self::switch_lights(&state, button);
                    new_states.insert(new_state);
                }
            }
            if new_states.contains(&self.lights) {
                return count;
            }
            states = new_states;
        }
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let machines: Vec<Machine> = self
            .get_file_reader(input_file)
            .lines()
            .map(|line| Machine::from(&line.unwrap()[..]))
            .collect();

        let result: u64 = machines.iter().map(Machine::minimum_button_presses).sum();

        Outcome::U64(result)
    }

    fn run_part_2(&self, _input_file: PathBuf) -> Outcome {
        Outcome::Text(String::from("d10p2"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_sample() {
        let solver = Day10 {};
        let result = solver.run_part_1(PathBuf::from("tests/day_10/sample.txt"));
        assert_eq!(result, Outcome::U64(7));
    }
}
