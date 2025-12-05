use itertools::Itertools;
use std::cmp::max;

advent_of_code::solution!(5);

#[derive(Debug)]
struct IngredientList {
    fresh_ingredients: Vec<(u64, u64)>,
    available_ingredients: Vec<u64>,
}

fn consolidate_ranges(range_list: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = vec![];

    let mut current_range: Option<(u64, u64)> = None;

    for candidate in range_list {
        match current_range {
            None => current_range = Some(candidate),
            Some((start, end)) => {
                if candidate.0 <= end {
                    current_range = Some((start, max(end, candidate.1)))
                } else {
                    ranges.push(current_range.unwrap());
                    current_range = Some((candidate.0, candidate.1));
                }
            }
        }
    }

    ranges.push(current_range.unwrap());

    ranges
}

fn check_ingredients(
    fresh_ingredients: Vec<(u64, u64)>,
    available_ingredients: Vec<u64>,
) -> Vec<u64> {
    let mut matches: Vec<u64> = vec![];

    for ingredient in available_ingredients {
        for (range_start, range_end) in &fresh_ingredients {
            if ingredient <= *range_end && ingredient >= *range_start {
                matches.push(ingredient);
            }
        }
    }

    matches
}

fn count_fresh_ingredients(ranges: Vec<(u64, u64)>) -> u64 {
    let mut count = 0;

    for (start, end) in ranges {
        count += end - (start - 1)
    }

    count
}

impl IngredientList {
    fn from_text(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n\n").collect();

        let fresh_ingredients = IngredientList::parse_ranges(parts[0]);
        let available_ingredients = IngredientList::parse_available(parts[1]);

        Self {
            fresh_ingredients,
            available_ingredients,
        }
    }

    fn parse_ranges(text: &str) -> Vec<(u64, u64)> {
        let ranges = text
            .lines()
            .flat_map(|line| {
                line.split(',')
                    .map(|entry| {
                        let parts = entry.split('-').collect::<Vec<&str>>();
                        (
                            parts[0].parse::<u64>().unwrap_or(0),
                            parts[1].parse::<u64>().unwrap_or(0),
                        )
                    })
                    .collect::<Vec<(u64, u64)>>()
            })
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect::<Vec<(u64, u64)>>();

        consolidate_ranges(ranges)
    }

    fn parse_available(text: &str) -> Vec<u64> {
        let mut ingredients = vec![];

        for line in text.lines() {
            ingredients.push(line.parse::<u64>().unwrap_or(0));
        }

        ingredients
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let list = IngredientList::from_text(input);

    Some(
        check_ingredients(list.fresh_ingredients, list.available_ingredients)
            .iter()
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let list = IngredientList::from_text(input);
    Some(count_fresh_ingredients(list.fresh_ingredients))
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_consolidate_ranges() {
        let list = vec![(1, 1), (3, 5), (4, 6), (5, 10)];
        assert_eq!(consolidate_ranges(list), vec![(1, 1), (3, 10)])
    }

    #[test]
    fn test_check_ingredients() {
        let fresh_ingredients = vec![(1, 5), (7, 10)];
        let available_ingredients = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            check_ingredients(fresh_ingredients, available_ingredients),
            vec![1, 2, 3, 4, 5, 7, 8, 9]
        );
    }
}
