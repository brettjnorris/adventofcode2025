use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

#[derive(Debug)]
struct ServerRack {
    cable_map: HashMap<String, Vec<String>>,
}

impl ServerRack {
    fn from_input(input: &str) -> Self {
        let mut cable_map = HashMap::new();

        for line in input.lines() {
            if let [first, second] = line.split(':').collect::<Vec<&str>>()[..] {
                let outputs = second
                    .split(' ')
                    .map(|output| output.to_owned())
                    .filter(|input| !input.is_empty())
                    .collect::<Vec<String>>();

                cable_map.entry(first.to_owned()).or_insert(outputs);
            }
        }

        Self { cable_map }
    }

    fn find_path_count(&self, start: &str, check_required: bool) -> u64 {
        let mut cache = HashMap::new();
        // If not checking required nodes, pretend we've already seen them
        let (seen_fft, seen_dac) = if check_required {
            (false, false)
        } else {
            (true, true)
        };
        self.find_path_recursive(&mut cache, start, seen_fft, seen_dac)
    }

    fn find_path_recursive<'a>(
        &'a self,
        cache: &mut HashMap<(&'a str, bool, bool), u64>,
        node: &'a str,
        seen_fft: bool,
        seen_dac: bool,
    ) -> u64 {
        let seen_fft = seen_fft || node == "fft";
        let seen_dac = seen_dac || node == "dac";

        if node == "out" {
            return if seen_fft && seen_dac { 1 } else { 0 };
        }

        let cache_key = (node, seen_fft, seen_dac);
        if let Some(&count) = cache.get(&cache_key) {
            return count;
        }

        let result = match self.cable_map.get(node) {
            None => 0,
            Some(outputs) => outputs
                .iter()
                .map(|o| self.find_path_recursive(cache, o.as_str(), seen_fft, seen_dac))
                .sum(),
        };

        cache.insert(cache_key, result);
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(ServerRack::from_input(input).find_path_count("you", false))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(ServerRack::from_input(input).find_path_count("svr", true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
