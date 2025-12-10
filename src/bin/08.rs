advent_of_code::solution!(8);

fn calculate_distance(a: (isize, isize, isize), b: (isize, isize, isize)) -> isize {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    let dz = b.2 - a.2;
    (dx * dx + dy * dy + dz * dz).abs()
}
#[derive(Debug)]
struct JunctionBoxes {
    boxes: Vec<(isize, isize, isize)>,
    pairs: Vec<(isize, usize, usize)>,
}

impl JunctionBoxes {
    fn from_text(input: &str) -> Self {
        let boxes = input
            .lines()
            .filter_map(|line| {
                let parts = line
                    .split(",")
                    .map(|part| part.parse::<isize>().unwrap_or(0))
                    .collect::<Vec<isize>>();

                if parts.len() >= 3 {
                    Some((parts[0], parts[1], parts[2]))
                } else {
                    None
                }
            })
            .collect::<Vec<(isize, isize, isize)>>();

        let mut pairs = vec![];

        for (a_index, a) in boxes.iter().enumerate() {
            for (b_index, b) in boxes.iter().enumerate().skip(a_index + 1) {
                pairs.push((calculate_distance(*a, *b), a_index, b_index));
            }
        }

        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        Self { boxes, pairs }
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>, // parent[i] = parent of element i
    size: Vec<usize>,   // size[i] = size of set (only valid when i is a root)
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // each element is its own parent initially
            size: vec![1; n],         // each set starts with size 1
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.find(self.parent[x])
        }
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.size[root_x] >= self.size[root_y] {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            } else {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            }
            true
        } else {
            false
        }
    }

    fn get_circuit_sizes(&self) -> Vec<usize> {
        self.parent
            .iter()
            .enumerate()
            .filter(|(i, p)| *i == **p) // only roots
            .map(|(i, _)| self.size[i])
            .collect()
    }
}

pub fn solve(input: &str, take_count: usize) -> Option<u64> {
    let junction_boxes = JunctionBoxes::from_text(input);
    let mut uf = UnionFind::new(junction_boxes.boxes.len());

    for (_, a, b) in junction_boxes.pairs.iter().take(take_count) {
        uf.union(*a, *b);
    }

    let mut sizes = uf.get_circuit_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // sort descending

    let result = sizes[0] * sizes[1] * sizes[2];
    Some(result as u64)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes = JunctionBoxes::from_text(input);
    let mut uf = UnionFind::new(junction_boxes.boxes.len());
    let mut last_connection: Option<(usize, usize)> = None;

    for (dist, a, b) in junction_boxes.pairs.iter() {
        if uf.union(*a, *b) {
            last_connection = Some((*a, *b));
        }
    }

    if let Some((index_a, index_b)) = last_connection {
        let a = junction_boxes.boxes[index_a];
        let b = junction_boxes.boxes[index_b];
        let result = a.0 * b.0;

        Some(result as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
