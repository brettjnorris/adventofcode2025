use std::thread::current;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    left: usize,
    right: usize,
    up: usize,
    down: usize,
    column: usize,
}

#[derive(Debug)]
pub struct Arena {
    pub nodes: Vec<Node>,
    pub primary_columns: usize,
}

#[derive(Debug)]
enum TraversalDirection {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

impl Arena {
    // BUILDING
    pub fn new() -> Self {
        Arena {
            nodes: vec![Node { left: 0, right: 0, up: 0, down: 0, column: 0}],
            primary_columns: 0
        }
    }

    pub fn add_column(&mut self, primary: bool) -> usize {
        let new_index = self.nodes.len();
        let last_column = new_index - 1;

        let new_node = Node { left: last_column, right: 0, up: new_index, down: new_index, column: new_index};
        self.nodes.push(new_node);
        self.nodes[last_column].right = new_index;
        self.nodes[0].left = new_index;

        if primary {
            self.primary_columns += 1;
        }

        new_index
    }

    pub fn add_row(&mut self, columns: Vec<usize>) {
        let row_start = self.nodes.len();
        let mut row_indices: Vec<usize> = vec![];

        // First, we create the new node and create vertical links
        for &column in columns.iter() {
            let last_column_index = self.get_column_nodes(column).last().unwrap_or(&column).clone();
            let new_index = self.nodes.len();
            row_indices.push(new_index);

            let new_node = Node { left: 0, right: 0, up: last_column_index, down: column, column };
            self.nodes.push(new_node);

            self.nodes[last_column_index].down = new_index;
            self.nodes[column].up = new_index;
        }

        // Next, we set the horizontal links
        for (i, &node_index) in row_indices.iter().enumerate() {
            let left = row_indices[(i + row_indices.len() - 1) % row_indices.len()];
            let right = row_indices[(i + 1) % row_indices.len()];
            self.nodes[node_index].left = left;
            self.nodes[node_index].right = right;
        }
    }

    // TRAVERSAL
    fn traverse(&self, start_index: usize, direction: TraversalDirection) -> Vec<usize> {
        let mut nodes = vec![];
        let mut current_index = self.get_next_index(start_index, &direction);

        while current_index != start_index {
            nodes.push(current_index);
            current_index = self.get_next_index(current_index, &direction);
        }

        nodes
    }

    fn get_next_index(&self, start_index: usize, direction: &TraversalDirection) -> usize {
        match direction {
            TraversalDirection::DOWN => self.nodes[start_index].down,
            TraversalDirection::UP => self.nodes[start_index].up,
            TraversalDirection::LEFT => self.nodes[start_index].left,
            TraversalDirection::RIGHT => self.nodes[start_index].right,
        }
    }

    fn get_headers(&self, primary: bool) -> Vec<usize> {
        let mut headers = vec![];
        let mut current_index = self.nodes[0].right;

        while current_index != 0 && (!primary || (primary && current_index <= self.primary_columns)) {
            headers.push(current_index);
            current_index = self.nodes[current_index].right;
        }

        headers
    }

    fn get_column_nodes(&self, column_index: usize) -> Vec<usize> {
        self.traverse(column_index, TraversalDirection::DOWN)
    }

    fn reverse_column_nodes(&self, column_index: usize) -> Vec<usize> {
        self.traverse(column_index, TraversalDirection::UP)
    }

    fn get_row_nodes(&self, index: usize) -> Vec<usize> {
        self.traverse(index, TraversalDirection::RIGHT)
    }

    fn reverse_row_nodes(&self, index: usize) -> Vec<usize> {
        self.traverse(index, TraversalDirection::LEFT)
    }

    // RESTORE/REMOVE
    fn remove_horizontal(&mut self, index: usize) {
        let left = self.nodes[index].left;
        let right = self.nodes[index].right;

        self.nodes[left].right = right;
        self.nodes[right].left = left;
    }

    fn restore_horizontal(&mut self, index: usize) {
        let left = self.nodes[index].left;
        let right = self.nodes[index].right;

        self.nodes[left].right = index;
        self.nodes[right].left = index;
    }

    fn remove_vertical(&mut self, index: usize) {
        let up = self.nodes[index].up;
        let down = self.nodes[index].down;

        self.nodes[up].down = down;
        self.nodes[down].up = up;
    }

    fn restore_vertical(&mut self, index: usize) {
        let up = self.nodes[index].up;
        let down = self.nodes[index].down;

        self.nodes[up].down = index;
        self.nodes[down].up = index;
    }

    fn cover_column(&mut self, index: usize) {
        let column = &self.nodes[index];
        self.remove_horizontal(index);

        for col_node in self.get_column_nodes(index) {
            for row_node in self.get_row_nodes(col_node) {
                if (row_node != index) {
                    self.remove_vertical(row_node);
                }
            }
        }
    }

    fn uncover_column(&mut self, index: usize) {
        let column = &self.nodes[index];

        for col_node in self.reverse_column_nodes(index) {
            for row_node in self.reverse_row_nodes(col_node) {
                if (row_node != index) {
                    self.restore_vertical(row_node);
                }
            }
        }

        self.restore_horizontal(index);
    }

    pub fn solve(&mut self, depth: usize) -> Option<Vec<usize>> {
        self.solve_with_limit(depth, &mut None)
    }

    /// Solve with an optional call limit. Returns None if limit exceeded or no solution.
    pub fn solve_with_limit(&mut self, depth: usize, calls: &mut Option<usize>) -> Option<Vec<usize>> {
        // Check call limit
        if let Some(remaining) = calls {
            if *remaining == 0 {
                return None; // Limit exceeded
            }
            *remaining -= 1;
        }

        let headers = self.get_headers(true);

        if headers.is_empty() {
            return Some(vec![])
        }

        let column = headers
            .iter()
            .min_by_key(|&&col| self.get_column_nodes(col).len())
            .copied()
            .unwrap();

        let rows = self.get_column_nodes(column);

        if rows.is_empty() {
            return None
        }

        self.cover_column(column);

        for row_index in rows {
            let row_nodes = self.get_row_nodes(row_index);

            for &node_index in &row_nodes {
                let column_header = self.nodes[node_index].column;
                self.cover_column(column_header);
            }

            if let Some(solution) = self.solve_with_limit(depth + 1, calls) {
                return Some([vec![row_index], solution].concat())
            }

            // Uncover in reverse order
            for &node_index in row_nodes.iter().rev() {
                let column_header = self.nodes[node_index].column;
                self.uncover_column(column_header);
            }
        }

        self.uncover_column(column);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal() {
        // H        C1  C2  C3
        //     R1:   1   0   1
        //     R2:   0   1   1
        let nodes = vec![
            Node {left: 3, right: 1, up: 0, down: 0, column: 0}, // H

            Node {left: 0, right: 2, up: 4, down: 4, column: 1}, // C1
            Node {left: 1, right: 3, up: 6, down: 6, column: 2}, // C2
            Node {left: 2, right: 0, up: 7, down: 5, column: 3}, // C3

            Node {left: 5, right: 5, up: 1, down: 1, column: 1}, // R1-C1
            Node {left: 4, right: 4, up: 3, down: 7, column: 3}, // R1-C3

            Node {left: 7, right: 7, up: 2, down: 2, column: 2}, // R2-C2
            Node {left: 6, right: 6, up: 5, down: 3, column: 3}, // R2-C3
        ];
        let mut arena = Arena { nodes, primary_columns: 3 };

        println!("Arena: {:?}", arena);
        assert_eq!(arena.get_headers(true), vec![1, 2, 3]);
        arena.remove_horizontal(2);
        assert_eq!(arena.get_headers(true), vec![1, 3]);
        arena.restore_horizontal(2);
        assert_eq!(arena.get_headers(true), vec![1, 2, 3]);

        assert_eq!(arena.get_row_nodes(4), vec![5]);
        arena.remove_horizontal(5);
        assert_eq!(arena.get_row_nodes(4), vec![]);
        arena.restore_horizontal(5);
        assert_eq!(arena.get_row_nodes(4), vec![5]);
    }

    #[test]
    fn test_vertical() {
        // H        C1  C2  C3
        //     R1:   1   0   1
        //     R2:   0   1   1
        let nodes = vec![
            Node {left: 3, right: 1, up: 0, down: 0, column: 0}, // H

            Node {left: 0, right: 2, up: 4, down: 4, column: 1}, // C1
            Node {left: 1, right: 3, up: 6, down: 6, column: 2}, // C2
            Node {left: 2, right: 0, up: 7, down: 5, column: 3}, // C3

            Node {left: 5, right: 5, up: 1, down: 1, column: 1}, // R1-C1
            Node {left: 4, right: 4, up: 3, down: 7, column: 3}, // R1-C3

            Node {left: 7, right: 7, up: 2, down: 2, column: 2}, // R2-C2
            Node {left: 6, right: 6, up: 5, down: 3, column: 3}, // R2-C3
        ];
        let mut arena = Arena { nodes, primary_columns: 3 };

        assert_eq!(arena.get_column_nodes(3), vec![5, 7]);
        arena.remove_vertical(5);
        assert_eq!(arena.get_column_nodes(3), vec![7]);
        arena.restore_vertical(5);
        assert_eq!(arena.get_column_nodes(3), vec![5, 7]);
    }

    #[test]
    fn test_cover_uncover() {
        // H        C1  C2  C3
        //     R1:   1   0   1
        //     R2:   0   1   1
        let nodes = vec![
            Node {left: 3, right: 1, up: 0, down: 0, column: 0}, // H

            Node {left: 0, right: 2, up: 4, down: 4, column: 1}, // C1
            Node {left: 1, right: 3, up: 6, down: 6, column: 2}, // C2
            Node {left: 2, right: 0, up: 7, down: 5, column: 3}, // C3

            Node {left: 5, right: 5, up: 1, down: 1, column: 1}, // R1-C1
            Node {left: 4, right: 4, up: 3, down: 7, column: 3}, // R1-C3

            Node {left: 7, right: 7, up: 2, down: 2, column: 2}, // R2-C2
            Node {left: 6, right: 6, up: 5, down: 3, column: 3}, // R2-C3
        ];
        let mut arena = Arena { nodes, primary_columns: 3 };

        arena.cover_column(1);

        assert_eq!(arena.get_headers(true), vec![2, 3]);
        assert_eq!(arena.get_column_nodes(3), vec![7]);

        arena.uncover_column(1);

        assert_eq!(arena.get_headers(true), vec![1, 2, 3]);
        assert_eq!(arena.get_column_nodes(3), vec![5, 7]);
    }

    #[test]
    fn test_solve() {
        //      C1  C2  C3
        // R1:   1   0   0
        // R2:   1   1   0
        // R3:   0   1   1
        let nodes = vec![
            Node {left: 3, right: 1, up: 0, down: 0, column: 0}, // 0 H

            Node {left: 0, right: 2, up: 5, down: 4, column: 1}, // 1 C1
            Node {left: 1, right: 3, up: 7, down: 6, column: 2}, // 2 C2
            Node {left: 2, right: 0, up: 8, down: 8, column: 3}, // 3 C3

            Node {left: 4, right: 4, up: 1, down: 5, column: 1}, // 4 R1-C1

            Node {left: 6, right: 6, up: 4, down: 1, column: 1}, // 5 R2-C1
            Node {left: 5, right: 5, up: 2, down: 7, column: 2}, // 6 R2-C2

            Node {left: 8, right: 8, up: 6, down: 2, column: 2}, // 7 R3-C2
            Node {left: 7, right: 7, up: 3, down: 3, column: 3}, // 8 R3-C3
        ];
        let mut arena = Arena { nodes, primary_columns: 3 };

        assert_eq!(arena.solve(0), Some(vec![4, 7]));
    }

    #[test]
    fn test_solve_with_secondary() {
        //      C1  C2  C3   O1  O2
        // R1:   1   0   0    0   1
        // R2:   1   1   0    0   0
        // R3:   0   1   1    0   0
        let nodes = vec![
            Node {left: 3, right: 1, up: 0, down: 0, column: 0}, // 0 H

            Node {left: 0, right: 2, up: 5, down: 6, column: 1}, // 1 C1
            Node {left: 1, right: 3, up: 7, down: 9, column: 2}, // 2 C2
            Node {left: 2, right: 4, up: 8, down: 11, column: 3}, // 3 C3
            Node {left: 3, right: 5, up: 4, down: 4, column: 4}, // 4 O1
            Node {left: 4, right: 0, up: 7, down: 5, column: 5}, // 5 O2

            Node {left: 7, right: 7, up: 1, down: 8, column: 1}, // 6 R1-C1
            Node {left: 6, right: 6, up: 5, down: 5, column: 5}, // 7 R1-O1

            Node {left: 9, right: 9, up: 6, down: 1, column: 1}, // 8 R2-C1
            Node {left: 8, right: 8, up: 2, down: 10, column: 2}, // 9 R2-C2

            Node {left: 11, right: 11, up: 9, down: 2, column: 2}, // 10 R3-C2
            Node {left: 10, right: 10, up: 3, down: 3, column: 3}, // 11 R3-C3
        ];

        let mut arena = Arena { nodes, primary_columns: 3 };

        assert_eq!(arena.solve(0), Some(vec![6, 10]));
    }

    #[test]
    fn test_building() {
        //      C1  C2  C3   O1  O2
        // R1:   1   0   0    0   1
        // R2:   1   1   0    0   0
        // R3:   0   1   1    0   0
        let mut arena = Arena::new();

        let c1 = arena.add_column(true);
        let c2 = arena.add_column(true);
        let c3 = arena.add_column(true);
        let o1 = arena.add_column(false);
        let o2 = arena.add_column(false);

        arena.add_row(vec![c1, o2]);
        arena.add_row(vec![c1, c2]);
        arena.add_row(vec![c2, c3]);

        assert_eq!(arena.solve(0), Some(vec![6, 10]));
        println!("primary_columns: {}, total nodes: {}", arena.primary_columns, arena.nodes.len());
        assert_eq!(arena.primary_columns, 3);
        assert_eq!(arena.nodes.len(), 12);
    }

}
