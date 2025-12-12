advent_of_code::solution!(6);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operator {
    Multiply, Add
}

fn calculate_columns_from_input(text: &str) -> Option<Vec<u64>> {
    let lines: Vec<&str> = text.lines().filter(|line| !line.is_empty()).rev().collect();

    if let Some((head, tail)) = lines.split_first() {
        let operators = parse_operators(head);
        let totals = process_rows(operators, tail);
        Some(totals)
    } else {
        None
    }
}

fn parse_operators(line: &str) -> Vec<Operator> {
    line.split_whitespace().map(|operator| {
        match operator {
            "*" => Operator::Multiply,
            "+" => Operator::Add,
            _ => panic!("Unsupported operator")
        }
    }).collect()
}

fn process_rows(operators: Vec<Operator>, rows: &[&str]) -> Vec<u64> {
    rows.iter().fold(initialize_accumulator(&operators), |mut acc, &line| {
        for (index, val) in line.split_whitespace().enumerate() {
            let num = val.parse::<u64>().unwrap_or(0);

            match operators[index] {
                Operator::Multiply => {
                    acc[index] *= num
                },
                Operator::Add => {
                    acc[index] += num
                }
            }
        }

        acc
    })
}

fn parse_operators_by_index(line: &str) -> Vec<Operator> {
    let mut current_operator: Option<Operator> = None;
    line.chars().map(|char| {
        match char {
            '*' => current_operator = Some(Operator::Multiply),
            '+' => current_operator = Some(Operator::Add),
            _ => {}
        }

        current_operator.unwrap_or(Operator::Add)
    }).collect()
}

fn parse_columnar_input(text: &str) -> Option<Vec<u64>> {
    let lines: Vec<&str> = text.lines().filter(|line| !line.is_empty()).rev().collect();

    if let Some((head, tail)) = lines.split_first() {
        let operators = parse_operators_by_index(head);
        let reversed: Vec<&str> = tail.iter().cloned().rev().collect();
        let totals = process_columns(&reversed);

        Some(calculate_columnar_values(operators, totals))
    } else {
        None
    }
}

fn calculate_columnar_values(operators: Vec<Operator>, numbers: Vec<u64>) -> Vec<u64> {
    let mut totals: Vec<u64> = vec![];
    let mut sum_index = 0;
    let mut last_operator = Operator::Add;

    for (index, number) in numbers.into_iter().enumerate() {
        if number == 0 {
            sum_index += 1;
        } else {
            let operator = operators.get(index).copied().unwrap_or(last_operator);
            last_operator = operator;

            let new_value = match last_operator {
                Operator::Add => {
                    let current = totals.get(sum_index).unwrap_or(&0);
                    current + number
                }
                Operator::Multiply => {
                    let current = totals.get(sum_index).unwrap_or(&1);
                    current * number
                }
            };

            if sum_index >= totals.len() {
                totals.resize(sum_index + 1, new_value);
            } else {
               totals[sum_index] = new_value
            }
        }
    }

    totals
}

fn process_columns(lines: &[&str]) -> Vec<u64> {
    let mut parsed_values: Vec<Vec<char>> = Vec::new();

    for line in lines.iter() {
        for (index, val) in line.chars().enumerate() {
            if val != ' ' {
                if index >= parsed_values.len() {
                    parsed_values.resize_with(index + 1, Vec::new);
                }
                parsed_values[index].push(val);
            }
        }
    }

    parsed_values.iter().map(|vals| {
        if !vals.is_empty() {
            vals.iter().collect::<String>().parse::<u64>().unwrap_or(0)
        } else {
            0
        }
    }).collect()
}

fn initialize_accumulator(operators: &Vec<Operator>) -> Vec<u64> {
    operators.iter().map(|operator| {
        match operator {
            Operator::Multiply => 1,
            Operator::Add => 0
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    calculate_columns_from_input(input).map(|result| result.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_columnar_input(input).map(|result| result.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }

    #[test]
    fn test_parse_operators() {
        let input = String::from("*   +  *   *  *  +");
        let result = parse_operators(&input);
        assert_eq!(result, vec![Operator::Multiply, Operator::Add, Operator::Multiply, Operator::Multiply, Operator::Multiply, Operator::Add])
    }
}
