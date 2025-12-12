advent_of_code::solution!(9);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point(i64, i64);

#[derive(Debug, Copy, Clone)]
struct Bounds {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Bounds {
    fn from_points(a: Point, b: Point) -> Self {
        Self {
            min_x: a.0.min(b.0),
            max_x: a.0.max(b.0),
            min_y: a.1.min(b.1),
            max_y: a.1.max(b.1),
        }
    }

    fn area(&self) -> i64 {
        (self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)
    }

    fn center(&self) -> Point {
        Point(
            (self.min_x + self.max_x) / 2,
            (self.min_y + self.max_y) / 2,
        )
    }

    fn x_strictly_contains(&self, x: i64) -> bool {
        self.min_x < x && x < self.max_x
    }

    fn y_strictly_contains(&self, y: i64) -> bool {
        self.min_y < y && y < self.max_y
    }

    fn y_overlaps(&self, other: &Bounds) -> bool {
        self.min_y < other.max_y && self.max_y > other.min_y
    }

    fn x_overlaps(&self, other: &Bounds) -> bool {
        self.min_x < other.max_x && self.max_x > other.min_x
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    a: Point,
    b: Point,
}

impl Edge {
    fn from_points(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_vertical(&self) -> bool {
        self.a.0 == self.b.0
    }

    fn bounds(&self) -> Bounds {
        Bounds::from_points(self.a, self.b)
    }

    fn x(&self) -> i64 {
        self.a.0
    }
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    bounds: Bounds,
}

impl Rectangle {
    fn from_points(a: Point, b: Point) -> Self {
        Self {
            bounds: Bounds::from_points(a, b),
        }
    }

    fn area(&self) -> i64 {
        self.bounds.area()
    }
}

#[derive(Debug)]
struct TileFloor {
    tiles: Vec<Point>,
    bounding_lines: Vec<Edge>,
}

impl TileFloor {
    fn from_text(input: &str) -> Self {
        let tiles = input
            .lines()
            .filter_map(|line| {
                let parts = line
                    .split(",")
                    .map(|part| part.parse::<i64>().unwrap_or(0))
                    .collect::<Vec<i64>>();
                if parts.len() == 2 {
                    Some(Point(parts[0], parts[1]))
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();

        let bounding_lines = Self::parse_bounding_lines(&tiles);

        Self { tiles, bounding_lines }
    }

    fn parse_bounding_lines(tiles: &[Point]) -> Vec<Edge> {
        let mut bounding_lines = vec![];

        for i in 0..tiles.len() {
            match (tiles.first(), tiles.get(i), tiles.get(i + 1)) {
                (_, Some(&left), Some(&right)) => {
                    bounding_lines.push(Edge::from_points(left, right))
                }
                (Some(&first), Some(&left), None) => {
                    bounding_lines.push(Edge::from_points(left, first))
                }
                _ => {}
            }
        }

        bounding_lines
    }

    fn candidate_rectangles(&self) -> Vec<Rectangle> {
        let mut rectangles = vec![];

        for (index_a, &a) in self.tiles.iter().enumerate() {
            for &b in self.tiles.iter().skip(index_a) {
                rectangles.push(Rectangle::from_points(a, b))
            }
        }

        rectangles
    }

    fn find_largest_area(&self, rectangles: Vec<Rectangle>) -> i64 {
        rectangles
            .iter()
            .map(|r| r.area())
            .max()
            .unwrap_or(0)
    }

    fn find_bounded_rectangles(&self) -> Vec<Rectangle> {
        self.candidate_rectangles()
            .into_iter()
            .filter(|rect| {
                !self.rectangle_is_sliced(rect) && self.rectangle_is_bounded(rect)
            })
            .collect()
    }

    fn rectangle_is_sliced_by(&self, rect: &Rectangle, edge: &Edge) -> bool {
        let rb = &rect.bounds;
        let eb = edge.bounds();

        if edge.is_vertical() {
            rb.x_strictly_contains(eb.min_x) && rb.y_overlaps(&eb)
        } else {
            rb.y_strictly_contains(eb.min_y) && rb.x_overlaps(&eb)
        }
    }

    fn rectangle_is_sliced(&self, rect: &Rectangle) -> bool {
        self.bounding_lines
            .iter()
            .any(|edge| self.rectangle_is_sliced_by(rect, edge))
    }

    fn rectangle_is_bounded(&self, rect: &Rectangle) -> bool {
        let center = rect.bounds.center();

        let crossings = self
            .bounding_lines
            .iter()
            .filter(|edge| edge.is_vertical() && Self::crosses_edge(center, edge))
            .count();

        crossings % 2 == 1
    }

    fn crosses_edge(test_point: Point, edge: &Edge) -> bool {
        let eb = edge.bounds();
        edge.x() > test_point.0 && eb.min_y < test_point.1 && test_point.1 <= eb.max_y
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let tile_floor = TileFloor::from_text(input);
    let rectangles = tile_floor.candidate_rectangles();

    Some(tile_floor.find_largest_area(rectangles) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tile_floor = TileFloor::from_text(input);
    let rectangles = tile_floor.find_bounded_rectangles();

    Some(tile_floor.find_largest_area(rectangles) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_rectangle_area() {
        assert_eq!(Rectangle::from_points(Point(2, 5), Point(9, 7)).area(), 24);
        assert_eq!(Rectangle::from_points(Point(7, 1), Point(11, 7)).area(), 35);
        assert_eq!(Rectangle::from_points(Point(2, 5), Point(11, 1)).area(), 50);
    }

    #[test]
    fn test_bounds_strictly_contains() {
        let bounds = Bounds::from_points(Point(2, 2), Point(8, 6));

        assert_eq!(bounds.x_strictly_contains(5), true);
        assert_eq!(bounds.x_strictly_contains(1), false);
        assert_eq!(bounds.x_strictly_contains(9), false);
        assert_eq!(bounds.x_strictly_contains(2), false); // on boundary
        assert_eq!(bounds.x_strictly_contains(8), false); // on boundary

        assert_eq!(bounds.y_strictly_contains(4), true);
        assert_eq!(bounds.y_strictly_contains(1), false);
        assert_eq!(bounds.y_strictly_contains(7), false);
        assert_eq!(bounds.y_strictly_contains(2), false); // on boundary
        assert_eq!(bounds.y_strictly_contains(6), false); // on boundary
    }

    #[test]
    fn test_bounds_overlaps() {
        let bounds = Bounds::from_points(Point(2, 2), Point(8, 6));

        // Overlapping y-ranges
        let overlapping = Bounds::from_points(Point(0, 4), Point(0, 9));
        assert_eq!(bounds.y_overlaps(&overlapping), true);

        // Non-overlapping y-ranges (above)
        let above = Bounds::from_points(Point(0, 7), Point(0, 9));
        assert_eq!(bounds.y_overlaps(&above), false);

        // Non-overlapping y-ranges (below)
        let below = Bounds::from_points(Point(0, 0), Point(0, 1));
        assert_eq!(bounds.y_overlaps(&below), false);
    }

    #[test]
    fn test_rectangle_is_sliced_by() {
        let floor = TileFloor {
            tiles: vec![],
            bounding_lines: vec![],
        };

        let rect = Rectangle::from_points(Point(2, 2), Point(8, 6));

        // === VERTICAL EDGES ===

        // Vertical edge slices through middle (x=5, from y=0 to y=9)
        let v_slices = Edge::from_points(Point(5, 0), Point(5, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_slices), true);

        // Vertical edge to the left of rectangle (x=1)
        let v_left = Edge::from_points(Point(1, 0), Point(1, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_left), false);

        // Vertical edge to the right of rectangle (x=9)
        let v_right = Edge::from_points(Point(9, 0), Point(9, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_right), false);

        // Vertical edge ON the boundary (x=2) - touches but doesn't slice
        let v_on_left_boundary = Edge::from_points(Point(2, 0), Point(2, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_on_left_boundary), false);

        // Vertical edge ON the boundary (x=8)
        let v_on_right_boundary = Edge::from_points(Point(8, 0), Point(8, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_on_right_boundary), false);

        // Vertical edge in x-range but y-range doesn't overlap (x=5, y=7 to y=9)
        let v_no_y_overlap = Edge::from_points(Point(5, 7), Point(5, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_no_y_overlap), false);

        // Vertical edge in x-range, y-range partially overlaps
        let v_partial_y = Edge::from_points(Point(5, 4), Point(5, 9));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &v_partial_y), true);

        // === HORIZONTAL EDGES ===

        // Horizontal edge slices through middle (y=4, from x=0 to x=9)
        let h_slices = Edge::from_points(Point(0, 4), Point(9, 4));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &h_slices), true);

        // Horizontal edge above rectangle (y=1)
        let h_above = Edge::from_points(Point(0, 1), Point(9, 1));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &h_above), false);

        // Horizontal edge below rectangle (y=7)
        let h_below = Edge::from_points(Point(0, 7), Point(9, 7));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &h_below), false);

        // Horizontal edge ON the boundary (y=2)
        let h_on_top_boundary = Edge::from_points(Point(0, 2), Point(9, 2));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &h_on_top_boundary), false);

        // Horizontal edge in y-range but x-range doesn't overlap (y=4, x=9 to x=10)
        let h_no_x_overlap = Edge::from_points(Point(9, 4), Point(10, 4));
        assert_eq!(floor.rectangle_is_sliced_by(&rect, &h_no_x_overlap), false);
    }

    #[test]
    fn test_rectangle_is_bounded() {
        // Simple square polygon: vertices at (0,0), (10,0), (10,10), (0,10)
        //
        //   (0,10) ------- (10,10)
        //     |              |
        //     |              |
        //   (0,0) -------- (10,0)

        let floor = TileFloor {
            tiles: vec![Point(0, 0), Point(10, 0), Point(10, 10), Point(0, 10)],
            bounding_lines: vec![
                Edge::from_points(Point(0, 0), Point(10, 0)),   // bottom edge
                Edge::from_points(Point(10, 0), Point(10, 10)), // right edge
                Edge::from_points(Point(10, 10), Point(0, 10)), // top edge
                Edge::from_points(Point(0, 10), Point(0, 0)),   // left edge
            ],
        };

        // Rectangle fully inside: (2,2) to (5,5)
        let inside = Rectangle::from_points(Point(2, 2), Point(5, 5));
        assert_eq!(floor.rectangle_is_bounded(&inside), true);

        // Rectangle fully outside: (12,12) to (15,15)
        let outside = Rectangle::from_points(Point(12, 12), Point(15, 15));
        assert_eq!(floor.rectangle_is_bounded(&outside), false);

        // Rectangle with corners on boundary but inside: (0,0) to (5,5)
        let on_boundary = Rectangle::from_points(Point(0, 0), Point(5, 5));
        assert_eq!(floor.rectangle_is_bounded(&on_boundary), true);

        // Rectangle that extends outside: (5,5) to (15,15)
        let extends_outside = Rectangle::from_points(Point(5, 5), Point(15, 15));
        assert_eq!(floor.rectangle_is_bounded(&extends_outside), false);
    }

    #[test]
    fn test_crosses_edge() {
        // Vertical edge at x=10, from y=0 to y=10
        let edge = Edge::from_points(Point(10, 0), Point(10, 10));

        // Point to the left, y in range
        assert_eq!(TileFloor::crosses_edge(Point(5, 5), &edge), true);

        // Point to the right
        assert_eq!(TileFloor::crosses_edge(Point(15, 5), &edge), false);

        // Point at same x
        assert_eq!(TileFloor::crosses_edge(Point(10, 5), &edge), false);

        // Point y below edge
        assert_eq!(TileFloor::crosses_edge(Point(5, -1), &edge), false);

        // Point y above edge
        assert_eq!(TileFloor::crosses_edge(Point(5, 11), &edge), false);

        // Point y at min (exclusive, so false)
        assert_eq!(TileFloor::crosses_edge(Point(5, 0), &edge), false);

        // Point y at max (inclusive, so true)
        assert_eq!(TileFloor::crosses_edge(Point(5, 10), &edge), true);
    }
}