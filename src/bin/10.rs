use good_lp::{coin_cbc, constraint, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    expected_output: isize,
    button_bitmasks: Vec<isize>,
    button_vectors: Vec<Vec<u8>>,
    joltages: Vec<isize>,
}

impl Machine {
    fn from_input(text: &str) -> Option<Self> {
        let main_re = Regex::new(r"\[([^\]]+)\]\s+(.+?)\s+\{([^\}]+)\}").unwrap();
        let paren_re = Regex::new(r"\(([^\)]+)\)").unwrap();

        let caps = main_re.captures(text)?;

        let indicator_lights = caps.get(1)?.as_str();
        let buttons_section = caps.get(2)?.as_str();
        let joltages_str = caps.get(3)?.as_str();

        let buttons: Vec<&str> = paren_re
            .captures_iter(buttons_section)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect();

        let joltages = parse_joltages(joltages_str);

        Some(Self {
            expected_output: indicator_as_bitmask(indicator_lights),
            button_bitmasks: buttons.iter().map(|b| button_as_bitmask(b)).collect(),
            button_vectors: buttons.iter().map(|b| button_as_vector(b, joltages.len())).collect(),
            joltages,
        })
    }

    fn find_solution_for_lights(&self) -> Option<usize> {
        (0..=self.button_bitmasks.len())
            .find_map(|count| {
                self.button_bitmasks
                    .iter()
                    .combinations(count)
                    .find(|combo| combo.iter().copied().fold(0, |acc, x| acc ^ x) == self.expected_output)
                    .map(|_| count)
            })
    }

    fn find_solution_for_joltages(&self) -> Option<usize> {
        let num_buttons = self.button_vectors.len();
        let num_counters = self.joltages.len();

        let mut vars = variables!();
        let button_presses: Vec<_> = (0..num_buttons)
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();

        let objective: good_lp::Expression = button_presses.iter().sum();
        let mut problem = vars.minimise(objective).using(coin_cbc);

        for counter_idx in 0..num_counters {
            let expr: good_lp::Expression = self.button_vectors
                .iter()
                .enumerate()
                .filter(|(_, button)| button[counter_idx] == 1)
                .map(|(button_idx, _)| button_presses[button_idx])
                .sum();

            problem = problem.with(constraint!(expr == self.joltages[counter_idx] as i32));
        }

        problem.solve().ok().map(|solution| {
            button_presses
                .iter()
                .map(|&v| solution.value(v).round() as usize)
                .sum()
        })
    }
}

fn indicator_as_bitmask(input: &str) -> isize {
    let binary_string: String = input
        .chars()
        .rev()
        .map(|c| if c == '#' { '1' } else { '0' })
        .collect();

    isize::from_str_radix(&binary_string, 2).unwrap_or(0)
}

fn button_as_bitmask(input: &str) -> isize {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .fold(0, |acc, pos| acc | (1 << pos))
}

fn button_as_vector(input: &str, num_counters: usize) -> Vec<u8> {
    let mut result = vec![0u8; num_counters];
    for pos in input.split(',').filter_map(|s| s.trim().parse::<usize>().ok()) {
        if pos < num_counters {
            result[pos] = 1;
        }
    }
    result
}

fn parse_joltages(input: &str) -> Vec<isize> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum: usize = input
        .lines()
        .filter_map(|line| Machine::from_input(line)?.find_solution_for_lights())
        .sum();

    Some(sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: usize = input
        .lines()
        .filter_map(|line| Machine::from_input(line)?.find_solution_for_joltages())
        .sum();

    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_parse_buttons() {
        assert_eq!(button_as_bitmask("1,3"), 0b1010);
        assert_eq!(button_as_bitmask("3,5,4,7"), 0b10111000);
    }

    #[test]
    fn test_parse_indicator() {
        assert_eq!(indicator_as_bitmask(".##."), 0b0110);
        assert_eq!(indicator_as_bitmask("...#."), 0b01000);
        assert_eq!(indicator_as_bitmask(".###.#"), 0b101110);
    }

    #[test]
    fn test_find_solution_for_lights() {
        let example1 = Machine::from_input("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .unwrap().find_solution_for_lights();
        assert_eq!(example1, Some(2));

        let example2 = Machine::from_input("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
            .unwrap().find_solution_for_lights();
        assert_eq!(example2, Some(3));

        let example3 = Machine::from_input("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
            .unwrap().find_solution_for_lights();
        assert_eq!(example3, Some(2));
    }

    #[test]
    fn test_find_solution_for_joltages() {
        let example1 = Machine::from_input("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .unwrap().find_solution_for_joltages();
        assert_eq!(example1, Some(10));

        let example2 = Machine::from_input("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}")
            .unwrap().find_solution_for_joltages();
        assert_eq!(example2, Some(12));

        let example3 = Machine::from_input("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")
            .unwrap().find_solution_for_joltages();
        assert_eq!(example3, Some(11));
    }
}