use std::iter::FromIterator;

advent_of_code::solution!(3);

struct BatteryArray {
    banks: Vec<BatteryBank>
}

struct BatteryBank(Vec<u8>);

impl BatteryArray {
    fn from_text(text: &str) -> Self {
       let banks = text.lines().map(|line| {
            BatteryBank::from_text(line)
       }).collect::<Vec<BatteryBank>>();

       Self { banks }
    }

    fn total_voltage(&self, max_depth: usize) -> u64 {
        self.banks.iter().map(|bank| bank.highest_voltage(max_depth).unwrap_or(0)).sum()
    }
}

impl FromIterator<u8> for BatteryBank {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        BatteryBank(iter.into_iter().collect())
    }
}

impl BatteryBank {
    fn from_text(text: &str) -> Self {
        text.chars().map(|battery| {
            battery.to_string().parse::<u8>().unwrap_or(0)
        }).collect::<BatteryBank>()
    }

    fn highest_voltage(&self, max_depth: usize) -> Option<u64> {
        BatteryBank::highest_voltage_recursive(max_depth, &mut vec![], &self.0)
    }

    fn highest_voltage_recursive(max_depth: usize, current_stack: &mut Vec<u8>, candidates: &[u8]) -> Option<u64> {
        if current_stack.len() >= max_depth {
            let voltage = current_stack.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("").parse::<u64>().unwrap_or(0);
            return Some(voltage);
        }

        if candidates.is_empty() || (candidates.len() + current_stack.len() < max_depth) {
            return None
        }

        let remaining_picks = max_depth - current_stack.len();
        let last_valid_index = candidates.len() - remaining_picks;
        let valid_candidates = &candidates[0..=last_valid_index];

        let max_value = valid_candidates.iter().max().unwrap();
        let max_index = valid_candidates.iter().position(|x| x == max_value).unwrap();

        current_stack.push(candidates[max_index]);
        let result = Self::highest_voltage_recursive(max_depth, current_stack, &candidates[(max_index + 1)..]);
        current_stack.pop();

        result
    }

}
pub fn part_one(input: &str) -> Option<u64> {
    let total_voltage = BatteryArray::from_text(input).total_voltage(2);

    Some(total_voltage)
}

pub fn part_two(input: &str) -> Option<u64> {
    let total_voltage = BatteryArray::from_text(input).total_voltage(12);

    Some(total_voltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_highest_voltage_recursive() {
        let bank = BatteryBank::from_text("987654321111111");
        let result = BatteryBank::highest_voltage_recursive(2, &mut vec![], &bank.0);
        assert_eq!(result, Some(98));

        let result = BatteryBank::highest_voltage_recursive(12, &mut vec![], &bank.0);
        assert_eq!(result, Some(987654321111));

        let bank = BatteryBank::from_text("811111111111119");
        let result = BatteryBank::highest_voltage_recursive(2, &mut vec![], &bank.0);
        assert_eq!(result, Some(89));

        let result = BatteryBank::highest_voltage_recursive(12, &mut vec![], &bank.0);
        assert_eq!(result, Some(811111111119))
    }
}
