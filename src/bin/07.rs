use std::cmp::max;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

const BEAM_SPLIT_VECTORS: [(i32, i32); 2] = [(0, -1), (0, 1)];

struct BeamMap {
    splitter_map: HashMap<usize, Vec<usize>>,
    start_position: (usize, usize),
    row_size: usize,
    col_size: usize,
}

impl BeamMap {
    fn from_text(input: &str) -> Self {
        let mut splitter_map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut start_position = (0, 0);
        let mut row_size = 0;
        let mut col_size = 0;

        for (row_index, line) in input.lines().enumerate() {
            row_size = max(row_size, row_index);
            for (col_index, char) in line.chars().enumerate() {
                col_size = max(col_size, col_index + 1);
                match char {
                    'S' => start_position = (row_index, col_index),
                    '^' => splitter_map
                        .entry(col_index)
                        .or_default()
                        .push(row_index),
                    _ => {}
                }
            }
        }

        Self {
            splitter_map,
            start_position,
            row_size,
            col_size,
        }
    }

    fn count_visited_splitters(&self) -> Option<u64> {
        let mut count: u64 = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut visited_splitters: HashSet<(usize, usize)> = HashSet::new();
        let mut stack: Vec<(usize, usize)> = vec![];

        if let Some(new_row) =
            self.find_next_splitter_row(self.start_position.1, self.start_position.0)
        {
            let splitter_position = (new_row, self.start_position.1);
            count += 1;
            for beam_position in self.get_valid_split_positions(splitter_position) {
                stack.push(beam_position);
                visited.insert(beam_position);
            }
        }

        while let Some((row, col)) = stack.pop() {
            if let Some(splitter_row) = self.find_next_splitter_row(col, row) {
                let split_position = (splitter_row, col);

                if visited_splitters.contains(&split_position) {
                    continue;
                } else {
                    visited_splitters.insert(split_position);
                }

                count += 1;
                for new_position in self.get_valid_split_positions(split_position) {
                    if !visited.contains(&new_position) {
                        stack.push(new_position);
                        visited.insert(new_position);
                    }
                }
            }
        }

        Some(count)
    }

    fn find_next_splitter_row(&self, col_index: usize, current_row: usize) -> Option<usize> {
        if let Some(col_splitters) = self.splitter_map.get(&col_index) {
            col_splitters
                .iter()
                .find(|&elem| elem > &current_row).copied()
        } else {
            None
        }
    }

    fn get_valid_split_positions(&self, start_position: (usize, usize)) -> Vec<(usize, usize)> {
        BEAM_SPLIT_VECTORS
            .iter()
            .filter_map(|(row_delta, col_delta)| {
                let (new_row, new_col) = (
                    start_position.0.wrapping_add(*row_delta as usize),
                    start_position.1.wrapping_add(*col_delta as usize),
                );
                if new_col <= self.col_size && new_row <= self.row_size {
                    Some((new_row, new_col))
                } else {
                    None
                }
            })
            .collect()
    }

    fn count_paths(&self) -> u64 {
        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
        self.count_paths_recursive(&mut cache, self.start_position)
    }

    fn count_paths_recursive(
        &self,
        cache: &mut HashMap<(usize, usize), u64>,
        start: (usize, usize),
    ) -> u64 {
        if let Some(cached_count) = cache.get(&start) {
            return *cached_count;
        }

        let mut count = 0;

        if let Some(position) = self.find_next_splitter_row(start.1, start.0) {
            for new_position in self.get_valid_split_positions((position, start.1)) {
                let sub_count = self.count_paths_recursive(cache, new_position);
                cache.entry(new_position).or_insert(sub_count);
                count += sub_count;
            }

            count
        } else {
            1
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    BeamMap::from_text(input).count_visited_splitters()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(BeamMap::from_text(input).count_paths())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
