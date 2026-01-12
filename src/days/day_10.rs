use crate::shared::{Outcome, Solution};
use std::collections::HashSet;
use std::io::BufRead;
use std::path::PathBuf;

type Lights = Vec<bool>;
type Button = Vec<usize>;
type Joltages = Vec<usize>;

/// Machine abstraction (one input line)
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltages: Joltages,
}

impl From<&str> for Machine {
    /// Constructor from a line
    fn from(line: &str) -> Self {
        let mut lights = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        for block in line.split_whitespace() {
            let mut block_chars = block.chars();
            match block_chars.next().unwrap() {
                '[' => {
                    lights = block
                        .chars()
                        .filter_map(|c| match c {
                            '.' => Some(false),
                            '#' => Some(true),
                            _ => None,
                        })
                        .collect();
                }
                '(' => buttons.push(Machine::line_to_array(block)),
                '{' => joltages = Machine::line_to_array(block),
                _ => panic!("Unrecognized field"),
            }
        }

        Self {
            lights,
            buttons,
            joltages,
        }
    }
}

impl Machine {
    /// Helper to contains a string like {3, 5, 4} into a vector
    fn line_to_array(block: &str) -> Vec<usize> {
        let mut block_chars = block.chars();
        block_chars.next();
        block_chars.next_back();

        block_chars
            .as_str()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect()
    }

    /// Switch lights based on button wiring
    pub fn switch_lights(lights: &Lights, button: &Button) -> Lights {
        let mut lights = lights.clone();
        for idx in button {
            lights[*idx] = !lights[*idx];
        }
        lights
    }

    /// Increment joltages based on button wiring
    pub fn add_joltages(joltages: &Joltages, button: &Button) -> Joltages {
        let mut joltages = joltages.clone();
        for idx in button {
            joltages[*idx] += 1;
        }
        joltages
    }

    /// Return minimal number of button presses to find the lights state
    pub fn minimum_button_presses_lights(&self) -> u64 {
        // Cleverly brute force all button options
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

    /// Return minimal number of button presses to find the joltages state
    pub fn minimum_button_presses_joltages(&self) -> u64 {
        // Similar to the other method

        let mut states: HashSet<Joltages> = HashSet::from([vec![0; self.joltages.len()]]);
        let mut count = 0;
        loop {
            let mut new_states: HashSet<Joltages> = HashSet::new();
            count += 1;
            for state in states.drain() {
                if state
                    .iter()
                    .enumerate()
                    .any(|(idx, &val)| val > self.joltages[idx])
                {
                    continue;
                    // Any of the joltage number has surpassed the target, this branch is useless
                }

                for button in &self.buttons {
                    let new_state = Self::add_joltages(&state, button);
                    new_states.insert(new_state);
                }
            }
            if new_states.contains(&self.joltages) {
                return count;
            }
            states = new_states;
        }
    }
}

pub struct Day10;

impl Solution for Day10 {
    fn run_part_1(&self, input_file: PathBuf) -> Outcome {
        let machines = self.make_machines(input_file);

        let result: u64 = machines
            .iter()
            .map(Machine::minimum_button_presses_lights)
            .sum();

        Outcome::U64(result)
    }

    fn run_part_2(&self, input_file: PathBuf) -> Outcome {
        let machines = self.make_machines(input_file);

        let result: u64 = machines
            .iter()
            .map(Machine::minimum_button_presses_joltages)
            .sum();

        Outcome::U64(result)
    }
}

impl Day10 {
    fn make_machines(&self, input_file: PathBuf) -> Vec<Machine> {
        self.get_file_reader(input_file)
            .lines()
            .map(|line| Machine::from(line.unwrap().as_str()))
            .collect()
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

    #[test]
    fn part_2_sample() {
        let solver = Day10 {};
        let result = solver.run_part_2(PathBuf::from("tests/day_10/sample.txt"));
        assert_eq!(result, Outcome::U64(33));
    }
}
