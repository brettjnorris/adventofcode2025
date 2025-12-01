advent_of_code::solution!(1);

struct Safe {
    position: usize,
    instructions: Vec<isize>,
}

fn wrap_with_counts(position: i64, input: i64, min: i64, max: i64) -> (i64, i64) {
    let modulus = max - min;
    let new_position = position + input;

    let final_position = ((new_position % modulus + modulus) % modulus) + min;
    let overwraps = if input > 0 {
        new_position / modulus - position / modulus
    } else {
        (position - 1).div_euclid(modulus) - (new_position - 1).div_euclid(modulus)
    };

    (final_position, overwraps)
}

impl Safe {
    fn from_text(starting_position: usize, text: &str) -> Self {
        let instructions = text.lines().map(|line| {
            let (dir, amount) = line.split_at(1);
            let multiplier = if dir == "L" { -1 } else { 1 };
            amount.parse::<isize>().unwrap_or(0) * multiplier
        }).collect();

        Safe { position: starting_position, instructions }
    }

    fn count_ending_positions(&mut self, target: usize) -> Option<u64> {
        let mut matches = 0;
        for &amount in &self.instructions {
            let (new_position, _) = wrap_with_counts(self.position as i64, amount as i64, 0, 100);
            self.position = new_position as usize;
            if self.position == target {
                matches += 1;
            }
        }
        Some(matches)
    }

    fn count_overflows(&mut self) -> Option<u64> {
        let mut visits = 0;
        for &amount in &self.instructions {
            let (new_position, overwraps) = wrap_with_counts(self.position as i64, amount as i64, 0, 100);
            self.position = new_position as usize;
            visits += overwraps as u64;
        }
        Some(visits)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Safe::from_text(50, input).count_ending_positions(0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Safe::from_text(50, input).count_overflows()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_overwrap_with_counts() {
        assert_eq!(wrap_with_counts(14, -82, 0, 100), (32, 1));
        assert_eq!(wrap_with_counts(80, -687, 0, 100), (93, 7));
    }
}
