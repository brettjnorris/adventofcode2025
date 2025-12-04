use std::collections::HashSet;

advent_of_code::solution!(4);

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Grid {
    items: HashSet<(isize, isize)>,
}

impl Grid {
    fn from_text(text: &str) -> Self {
        let mut points: HashSet<(isize, isize)> = HashSet::new();
        text.lines().enumerate().for_each(|(x, line)| {
            line.chars().enumerate().for_each(|(y, char)| match char {
                '.' => {}
                _ => {
                    let point = (x as isize, y as isize);
                    points.insert(point);
                }
            })
        });

        Self { items: points }
    }

    fn count_neighbors(&self, x: isize, y: isize) -> usize {
        NEIGHBOR_OFFSETS
            .iter()
            .filter(|(offset_x, offset_y)| {
                self.items.contains(&(x + offset_x, y + offset_y))
            })
            .count()
    }

    fn get_neighbor_positions(&self, x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
        NEIGHBOR_OFFSETS
            .iter()
            .map(move |(offset_x, offset_y)| (x + offset_x, y + offset_y))
            .filter(|pos| self.items.contains(pos))
    }

    fn find_reachable_items(&self, max_occupied_neighbors: usize) -> Vec<(isize, isize)> {
        self.items
            .iter()
            .filter(|(x, y)| self.count_neighbors(*x, *y) < max_occupied_neighbors)
            .copied()
            .collect()
    }

    fn find_reachable_in_candidates(
        &self,
        candidates: &HashSet<(isize, isize)>,
        max_occupied_neighbors: usize,
    ) -> Vec<(isize, isize)> {
        candidates
            .iter()
            .filter(|(x, y)| {
                self.items.contains(&(*x, *y))
                    && self.count_neighbors(*x, *y) < max_occupied_neighbors
            })
            .copied()
            .collect()
    }

    fn remove_iteratively(&mut self) -> Vec<(isize, isize)> {
        let mut removed_items: Vec<(isize, isize)> = vec![];
        let mut candidates: HashSet<(isize, isize)> = self.items.clone();

        while !candidates.is_empty() {
            let reachable = self.find_reachable_in_candidates(&candidates, 4);

            if reachable.is_empty() {
                break;
            }

            let mut next_candidates: HashSet<(isize, isize)> = HashSet::new();
            for (x, y) in &reachable {
                for neighbor in self.get_neighbor_positions(*x, *y) {
                    next_candidates.insert(neighbor);
                }
            }

            for item in reachable {
                self.items.remove(&item);
                removed_items.push(item);
            }

            next_candidates.retain(|pos| self.items.contains(pos));

            candidates = next_candidates;
        }

        removed_items
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_text(input);
    let reachable_items = grid.find_reachable_items(4);

    Some(reachable_items.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_text(input);
    let removed_items = grid.remove_iteratively();

    Some(removed_items.len() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}