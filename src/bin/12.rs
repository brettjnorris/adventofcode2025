use advent_of_code::dlx::Arena;
use good_lp::{constraint, default_solver, variable, Expression, Solution, SolverModel, Variable};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

advent_of_code::solution!(12);

#[derive(Debug, Clone)]
struct Shape {
    width: usize,
    height: usize,
    points: Vec<Point>,
    permutations: Vec<Vec<Point>>,
}

impl Shape {
    fn from_input(input: &str) -> Self {
        let mut points = vec![];
        let mut width = 0;
        let mut height = 0;

        for (row, line) in input.lines().enumerate() {
            height = max(height, row);

            for (col, char) in line.chars().into_iter().enumerate() {
                width = max(width, col);

                match char {
                    '#' => points.push(Point(row as isize, col as isize)),
                    _ => {}
                }
            }
        }

        let permutations = Self::generate_permutations(&points);

        Self {
            points,
            permutations,
            width,
            height,
        }
    }

    fn apply_transform(points: &[Point], transform: fn(&Point) -> Point) -> Vec<Point> {
        points.iter().map(|p| transform(p)).collect()
    }

    fn normalize(points: Vec<Point>) -> Vec<Point> {
        let min_row = points.iter().map(|p| p.0).min().unwrap();
        let min_col = points.iter().map(|p| p.1).min().unwrap();

        points
            .iter()
            .map(|p| Point(p.0 - min_row, p.1 - min_col))
            .collect()
    }

    fn generate_permutations(points: &Vec<Point>) -> Vec<Vec<Point>> {
        let transforms: Vec<fn(&Point) -> Point> = vec![
            |p| Point(p.0, p.1),   // identity
            |p| Point(p.1, -p.0),  // 90° CW
            |p| Point(-p.0, -p.1), // 180°
            |p| Point(-p.1, p.0),  // 270° CW
            |p| Point(-p.0, p.1),  // flip
            |p| Point(p.1, p.0),   // flip + 90°
            |p| Point(p.0, -p.1),  // flip + 180°
            |p| Point(-p.1, -p.0), // flip + 270°
        ];

        transforms
            .iter()
            .map(|&transform| {
                let transformed = Self::apply_transform(points, transform);
                let mut normalized = Self::normalize(transformed);
                normalized.sort(); // Sort points for proper deduplication
                normalized
            })
            .unique()
            .collect::<Vec<Vec<Point>>>()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point(isize, isize);

#[derive(Debug)]
struct Puzzle {
    width: usize,
    height: usize,
    requirements: HashMap<usize, usize>,
}

impl Puzzle {
    fn find_solution_via_dlx(&self, shapes: &HashMap<usize, Shape>) -> Option<Vec<usize>> {
        // Early check: total cells needed must fit in grid
        let total_cells_needed: usize = self.requirements.iter()
            .map(|(&shape_idx, &count)| {
                shapes.get(&shape_idx).map(|s| s.permutations[0].len() * count).unwrap_or(0)
            })
            .sum();

        if total_cells_needed > self.width * self.height {
            return None; // Impossible - not enough space
        }

        let mut arena = self.build_arena(shapes);

        // No call limit - need correct answer
        arena.solve(0)
    }

    fn build_arena(&self, shapes: &HashMap<usize, Shape>) -> Arena {
        let mut arena = Arena::new();

        let num_cells = self.width * self.height;
        let piece_indices = self.necessary_piece_indices();

        // Build primary columns - one for each piece instance needed
        let mut piece_columns = vec![];
        for _ in &piece_indices {
            let column_index = arena.add_column(true);
            piece_columns.push(column_index);
        }

        // Build secondary columns - one for each grid cell
        let cell_column_start = arena.nodes.len();
        for _ in 0..num_cells {
            arena.add_column(false);
        }

        // Add rows for each possible piece placement
        for (piece_index, &piece_column) in piece_columns.iter().enumerate() {
            let shape_index = piece_indices[piece_index];
            let shape = shapes.get(&shape_index).unwrap();

            for permutation in &shape.permutations {
                for (start_row, start_col) in self.valid_positions_for_permutation(permutation) {
                    let mut row_columns = vec![piece_column];

                    for point in permutation {
                        let cell_row = start_row + point.0 as usize;
                        let cell_col = start_col + point.1 as usize;

                        let cell_index = cell_row * self.width + cell_col;
                        let cell_column = cell_column_start + cell_index;
                        row_columns.push(cell_column);
                    }

                    arena.add_row(row_columns);
                }
            }
        }

        arena
    }

    fn necessary_piece_indices(&self) -> Vec<usize> {
        let mut pieces = vec![];
        for (&index, &count) in self.requirements.iter() {
            for _ in 0..count {
                pieces.push(index)
            }
        }
        pieces
    }

    fn valid_positions_for_permutation(&self, permutation: &[Point]) -> Vec<(usize, usize)> {
        let mut positions = vec![];

        for start_row in 0..self.height {
            for start_col in 0..self.width {
                let fits = permutation.iter().all(|p| {
                    let final_row = start_row as isize + p.0;
                    let final_col = start_col as isize + p.1;
                    final_row >= 0
                        && final_row < self.height as isize
                        && final_col >= 0
                        && final_col < self.width as isize
                });

                if fits {
                    positions.push((start_row, start_col));
                }
            }
        }

        positions
    }

    /// Solve using Integer Linear Programming
    fn find_solution_via_ilp(&self, shapes: &HashMap<usize, Shape>) -> Option<bool> {
        // Early check: pieces must fit in grid (can't need more cells than available)
        let total_cells_needed: usize = self
            .requirements
            .iter()
            .map(|(&shape_idx, &count)| {
                shapes
                    .get(&shape_idx)
                    .map(|s| s.permutations[0].len() * count)
                    .unwrap_or(0)
            })
            .sum();

        let grid_size = self.width * self.height;
        if total_cells_needed > grid_size {
            // Can't fit all pieces - too many cells needed
            return Some(false);
        }

        // Generate all possible placements
        // Each placement is (piece_instance_index, cells_covered)
        let piece_indices = self.necessary_piece_indices();
        let mut placements: Vec<(usize, Vec<usize>)> = vec![];

        for (piece_instance, &shape_idx) in piece_indices.iter().enumerate() {
            let shape = shapes.get(&shape_idx)?;

            for permutation in &shape.permutations {
                for (start_row, start_col) in self.valid_positions_for_permutation(permutation) {
                    let cells: Vec<usize> = permutation
                        .iter()
                        .map(|p| {
                            let row = start_row + p.0 as usize;
                            let col = start_col + p.1 as usize;
                            row * self.width + col
                        })
                        .collect();

                    placements.push((piece_instance, cells));
                }
            }
        }

        if placements.is_empty() {
            return Some(false);
        }

        // Create ILP problem
        use good_lp::ProblemVariables;
        let mut problem = ProblemVariables::new();

        // Create binary variable for each placement
        let vars: Vec<Variable> = placements
            .iter()
            .map(|_| problem.add(variable().binary()))
            .collect();

        // Objective: just find feasibility (minimize 0)
        let objective: Expression = vars.iter().map(|&v| v * 0.0).sum();
        let mut model = problem.minimise(objective).using(default_solver);

        // Constraint: each piece instance placed exactly once
        for piece_instance in 0..piece_indices.len() {
            let piece_vars: Expression = placements
                .iter()
                .enumerate()
                .filter(|(_, (pi, _))| *pi == piece_instance)
                .map(|(i, _)| vars[i])
                .sum();

            model = model.with(constraint!(piece_vars == 1));
        }

        // Constraint: each cell covered at most once
        for cell in 0..grid_size {
            let cell_vars: Expression = placements
                .iter()
                .enumerate()
                .filter(|(_, (_, cells))| cells.contains(&cell))
                .map(|(i, _)| vars[i])
                .sum();

            model = model.with(constraint!(cell_vars <= 1));
        }

        // Solve
        match model.solve() {
            Ok(_solution) => Some(true),
            Err(_) => Some(false),
        }
    }
}

#[derive(Debug)]
struct PuzzleInput {
    shapes: HashMap<usize, Shape>,
    puzzles: Vec<Puzzle>,
}

impl PuzzleInput {
    fn from_input(input: &str) -> Self {
        let mut shapes = HashMap::new();
        let mut puzzles = vec![];

        for group in input.split("\n\n") {
            let re = Regex::new(r"^(\d+):\n([\s\S]+)$").unwrap();

            if let Some(caps) = re.captures(group) {
                let index: usize = caps[1].parse().unwrap();
                let shape_str = &caps[2];

                shapes.entry(index).or_insert(Shape::from_input(shape_str));
            } else {
                for line in group.lines() {
                    if let Some(puzzle) = Self::parse_puzzle(line) {
                        puzzles.push(puzzle);
                    }
                }
            }
        }

        Self { shapes, puzzles }
    }

    fn parse_shape(shape_input: &str) -> Vec<Point> {
        let mut points = vec![];

        for (row, line) in shape_input.lines().enumerate() {
            for (col, char) in line.chars().into_iter().enumerate() {
                match char {
                    '#' => points.push(Point(row as isize, col as isize)),
                    _ => {}
                }
            }
        }

        points
    }

    fn parse_puzzle(puzzle_input: &str) -> Option<Puzzle> {
        let re = Regex::new(r"^(\d+)x(\d+): (.+)$").unwrap();

        if let Some(caps) = re.captures(puzzle_input) {
            let width: usize = caps[1].parse().unwrap();
            let height: usize = caps[2].parse().unwrap();
            let mut requirements: HashMap<usize, usize> = HashMap::new();

            for (i, count) in caps[3]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .enumerate()
            {
                requirements.entry(i).or_insert(count);
            }

            Some(Puzzle {
                width,
                height,
                requirements,
            })
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let puzzle_input = PuzzleInput::from_input(input);
    let total = puzzle_input.puzzles.len();

    let solved = AtomicUsize::new(0);
    let completed = AtomicUsize::new(0);

    puzzle_input
        .puzzles
        .par_iter()
        .for_each(|puzzle| {
            // Early check: cells needed must fit in grid
            let cells_needed: usize = puzzle
                .requirements
                .iter()
                .map(|(&idx, &count)| {
                    puzzle_input
                        .shapes
                        .get(&idx)
                        .map(|s| s.permutations[0].len() * count)
                        .unwrap_or(0)
                })
                .sum();

            if cells_needed <= puzzle.width * puzzle.height {
                // Use ILP to solve
                if puzzle.find_solution_via_ilp(&puzzle_input.shapes) == Some(true) {
                    solved.fetch_add(1, Ordering::Relaxed);
                }
            }

            let done = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if done % 50 == 0 || done == total {
                eprintln!(
                    "Progress: {}/{} puzzles, {} solvable",
                    done,
                    total,
                    solved.load(Ordering::Relaxed)
                );
            }
        });

    Some(solved.load(Ordering::Relaxed) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_shapes() {
        let shape = Shape::from_input("\n###\n##.\n##.");
        eprintln!("shape: {:?}", shape);
        eprintln!("permutations: {:?}", shape.permutations.len());

        let shape = Shape::from_input("0:\n...\n.#.\n...");
        assert_eq!(shape.permutations.len(), 1);
    }

    #[test]
    fn test_puzzle_arena() {
        let mut requirements = HashMap::new();
        requirements.insert(0, 1);

        let puzzle = Puzzle {
            width: 4,
            height: 4,
            requirements,
        };

        let mut shapes = HashMap::new();
        shapes.entry(0).or_insert(Shape::from_input("\n###\n##.\n##."));

        eprintln!("result: {:?}", puzzle.find_solution_via_dlx(&shapes));
    }

    #[test]
    #[should_panic]
    fn test_shape_index_mapping() {
        // Puzzle requires shape index 4 (two copies)
        let mut requirements = HashMap::new();
        requirements.insert(4, 2); // shape 4, count 2

        let puzzle = Puzzle {
            width: 4,
            height: 4,
            requirements,
        };

        // If shapes HashMap doesn't have shape 4, unwrap will panic
        let mut shapes = HashMap::new();
        shapes.entry(0).or_insert(Shape::from_input("###\n#.."));

        // This should panic because shape 4 doesn't exist
        puzzle.find_solution_via_dlx(&shapes);
    }

    #[test]
    fn test_requirements_parsing() {
        let puzzle = PuzzleInput::parse_puzzle("4x4: 0 0 0 0 2 0").unwrap();

        // Should only require shape index 4 with count 2
        assert_eq!(puzzle.requirements.get(&4), Some(&2));

        // Verify necessary_piece_indices only includes shape 4 twice
        let indices = puzzle.necessary_piece_indices();
        assert_eq!(indices, vec![4, 4]);
    }

    #[test]
    fn test_unsolvable_example_input() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

12x5: 1 0 1 0 3 2
";

        let result = part_one(input);
        assert_eq!(result, Some(0));

    }

    #[test]
    fn test_shape_ordering() {
        let input = "0:\n#\n\n1:\n##\n\n2:\n###";
        let puzzle_input = PuzzleInput::from_input(input);

        let shapes: Vec<Shape> = puzzle_input.shapes.values().cloned().collect();

        // Print what's at each Vec index
        for (i, shape) in shapes.iter().enumerate() {
            eprintln!("shapes[{}] has {} points", i, shape.points.len());
        }
    }

    #[test]
    fn test_minimal_solve() {
        // Single piece, single cell
        let mut requirements = HashMap::new();
        requirements.insert(0, 1);

        let puzzle = Puzzle {
            width: 1,
            height: 1,
            requirements,
        };

        let mut shapes = HashMap::new();
        shapes.insert(0, Shape::from_input("#"));

        eprintln!("Calling find_solution_via_dlx...");
        let result = puzzle.find_solution_via_dlx(&shapes);
        eprintln!("Result: {:?}", result);

        assert!(result.is_some());
    }

    #[test]
    fn test_first_solvable_example() {
        // From problem: 4x4 grid with two shape-4 pieces (should be solvable)
        // Shape 4: ###
        //          #..
        //          ###
        let input = "4:
###
#..
###

4x4: 0 0 0 0 2 0
";
        let result = part_one(input);
        eprintln!("Result: {:?}", result);
        assert_eq!(result, Some(1)); // Should be solvable
    }

    #[test]
    fn test_second_solvable_example() {
        // From problem: 12x5 grid (should be solvable)
        let input = "0:
###
##.
##.

2:
.##
###
##.

4:
###
#..
###

5:
###
.#.
###

12x5: 1 0 1 0 2 2
";
        let result = part_one(input);
        eprintln!("Result: {:?}", result);
        assert_eq!(result, Some(1)); // Should be solvable
    }
}
