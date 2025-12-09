advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let operations = get_operations_1(input);

    let sum = sum_operations(&operations);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let operations = get_operations_2(input);

    let sum = sum_operations(&operations);

    Some(sum)
}

#[derive(Debug)]
enum Operator {
    Addition,
    Multiplication,
}

#[derive(Debug)]
struct Operation {
    values: Vec<u64>,
    operator: Option<Operator>,
}

fn get_operations_1(input: &str) -> Vec<Operation> {
    let mut parsed_lines = parse_lines(input);

    let last_line = parsed_lines.next_back().unwrap();

    // The last line should be the operators. Parse them first to create the
    // operations vec.
    let mut operations: Vec<Operation> = parse_operators(&last_line);

    // Parse each row and add the values to the corresponding operation
    for line in parsed_lines {
        for (col, item) in line.iter().enumerate() {
            let value = item.parse::<u64>().unwrap();

            operations[col].values.push(value);
        }
    }

    operations
}

fn get_operations_2(input: &str) -> Vec<Operation> {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let line_length = lines[0].len();

    let mut operations = Vec::new();
    let mut operator = None;
    let mut values = vec![];

    // Iterate over each line one character at a time to build up the operation.
    // We know we have a complete operation when every character in the column
    // is a space.
    for i in 0..line_length {
        let mut current_val = String::new();

        for line in &lines {
            let current_char = line.get(i);

            if let Some(char) = current_char {
                match char {
                    '+' => {
                        operator = Some(Operator::Addition);
                    }
                    '*' => {
                        operator = Some(Operator::Multiplication);
                    }
                    ' ' => {} // do nothing
                    _ => current_val.push(*char),
                }
            }
        }

        // This means that it was a separate column (every char a space)
        if current_val.is_empty() {
            operations.push(Operation { values, operator });
            // Reset the values and operator
            values = vec![];
            operator = None;
        } else {
            let value = current_val.parse::<u64>().unwrap();
            values.push(value);
        }
    }

    // At the end of the loops we need to add the last operation
    if !values.is_empty() && operator.is_some() {
        operations.push(Operation { values, operator });
    }

    operations
}

fn parse_lines(input: &str) -> impl DoubleEndedIterator<Item = Vec<&str>> {
    input.lines().map(|line| {
        // Extract just the values and ignore empty strings
        line.split(" ")
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>()
    })
}

fn parse_operators(input: &[&str]) -> Vec<Operation> {
    input
        .iter()
        .map(|line| {
            let operator = match *line {
                "+" => Some(Operator::Addition),
                "*" => Some(Operator::Multiplication),
                _ => None,
            };

            Operation {
                values: vec![],
                operator,
            }
        })
        .collect()
}

fn sum_operations(operations: &[Operation]) -> u64 {
    operations
        .iter()
        .map(|op| match op.operator {
            Some(Operator::Addition) => add(&op.values),
            Some(Operator::Multiplication) => multiply(&op.values),
            None => 0,
        })
        .sum()
}

fn add(values: &[u64]) -> u64 {
    values.iter().sum()
}

fn multiply(values: &[u64]) -> u64 {
    values.iter().product()
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
}
