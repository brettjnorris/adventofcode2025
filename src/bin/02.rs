use itertools::Itertools;

advent_of_code::solution!(2);

struct ProductRanges {
    ranges: Vec<(u64, u64)>,
}

impl ProductRanges {
    fn from_text(text: &str) -> Self {
        let ranges = text.lines().map(|line| {
            line.split(',').map(|range| {
                let parts = range.split('-').collect::<Vec<&str>>();
                (
                    parts[0].parse::<u64>().unwrap_or(0),
                    parts[1].parse::<u64>().unwrap_or(0),
                )
            }).collect::<Vec<(u64, u64)>>()
        }).flatten().collect();

        ProductRanges { ranges }
    }

    fn is_doubled(text: &str) -> bool {
        let len = text.len();
        let parts = text.split_at(len / 2);

        parts.0 == parts.1
    }

    fn is_repeated(text: &str) -> bool {
        (1..=text.len() / 2).any(|chunk_size| {
            text.len() % chunk_size == 0 && {
                let pattern = &text[..chunk_size];
                text.chars()
                    .chunks(chunk_size)
                    .into_iter()
                    .all(|chunk| chunk.collect::<String>() == pattern)
            }
        })
    }

    fn find_matching_ids<F>(&self, predicate: F) -> Vec<u64>
    where
        F: Fn(&str) -> bool,
    {
        self.ranges
            .iter()
            .flat_map(|(start, end)| *start..=*end)
            .filter(|id| predicate(&id.to_string()))
            .collect()
    }

    fn find_invalid_ids(&self) -> Vec<u64> {
        self.find_matching_ids(Self::is_doubled)
    }

    fn find_repeats(&self) -> Vec<u64> {
        self.find_matching_ids(Self::is_repeated)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = ProductRanges::from_text(input).find_invalid_ids().iter().sum::<u64>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = ProductRanges::from_text(input).find_repeats().iter().sum::<u64>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_is_doubled() {
        assert_eq!(ProductRanges::is_doubled("11"), true);
        assert_eq!(ProductRanges::is_doubled("22"), true);
        assert_eq!(ProductRanges::is_doubled("12"), false);
        assert_eq!(ProductRanges::is_doubled("1010"), true);
        assert_eq!(ProductRanges::is_doubled("1188511885"), true);
        assert_eq!(ProductRanges::is_doubled("222222"), true);
        assert_eq!(ProductRanges::is_doubled("446446"), true);
        assert_eq!(ProductRanges::is_doubled("38593859"), true);
    }

    #[test]
    fn test_is_repeated() {
        assert_eq!(ProductRanges::is_repeated("11"), true);
        assert_eq!(ProductRanges::is_repeated("22"), true);
        assert_eq!(ProductRanges::is_repeated("99"), true);
        assert_eq!(ProductRanges::is_repeated("111"), true);
        assert_eq!(ProductRanges::is_repeated("999"), true);
        assert_eq!(ProductRanges::is_repeated("1010"), true);
        assert_eq!(ProductRanges::is_repeated("1188511885"), true);
        assert_eq!(ProductRanges::is_repeated("222222"), true);
        assert_eq!(ProductRanges::is_repeated("446446"), true);
        assert_eq!(ProductRanges::is_repeated("38593859"), true);
        assert_eq!(ProductRanges::is_repeated("565656"), true);
        assert_eq!(ProductRanges::is_repeated("824824824"), true);
        assert_eq!(ProductRanges::is_repeated("2121212121"), true);
    }
}
