advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let trimmed = input.replace("\n", "");
    let ranges = get_ranges(&trimmed);
    let mut invalid_ids = Vec::new();

    ranges.for_each(|(start, end)| {
        for id in start..=end {
            if is_invalid_id_one(id) {
                invalid_ids.push(id);
            }
        }
    });

    // Sum up the invalid IDs
    Some(invalid_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let trimmed = input.replace("\n", "");
    let ranges = get_ranges(&trimmed);
    let mut invalid_ids = Vec::new();

    ranges.for_each(|(start, end)| {
        for id in start..=end {
            if is_invalid_id_two(id) {
                invalid_ids.push(id);
            }
        }
    });

    // Sum up the invalid IDs
    Some(invalid_ids.iter().sum())
}

fn get_ranges(input: &str) -> impl Iterator<Item = (usize, usize)> {
    input.split(",").map(parse_range)
}

fn parse_range(range_str: &str) -> (usize, usize) {
    let elems = range_str.split("-").collect::<Vec<&str>>();
    let start = elems[0].parse::<usize>().unwrap();
    let end = elems[1].parse::<usize>().unwrap();
    (start, end)
}

fn is_invalid_id_one(id: usize) -> bool {
    let id_chars = id.to_string().chars().collect::<Vec<char>>();

    // If we don't have an even number of digits, it cannot be invalid
    if id_chars.len() % 2 != 0 {
        return false;
    }

    let mut first_ptr = 0;
    let mut second_ptr = id_chars.len() / 2;

    while second_ptr < id_chars.len() {
        if id_chars[first_ptr] != id_chars[second_ptr] {
            return false;
        }
        first_ptr += 1;
        second_ptr += 1;
    }

    true
}

fn is_invalid_id_two(id: usize) -> bool {
    let id_chars = id.to_string().chars().collect::<Vec<char>>();

    // Need at least 2 characters to have a repeating sequence
    if id_chars.len() < 2 {
        return false;
    }

    let factors = get_factors(id_chars.len());

    for factor in factors {
        // Get target sequence to match
        // e.g. for factor 2 and id "121212", target_sequence = "12"
        let target_sequence = &id_chars[0..factor];

        // Check each character sequence of the same length
        let mut i = factor;

        while i < id_chars.len() {
            let current_sequence = &id_chars[i..i + factor];
            if current_sequence != target_sequence {
                break;
            }
            i += factor;
        }

        if i == id_chars.len() {
            // This means we matched the entire string with the sequence
            return true;
        }
    }

    false
}

// Only gets the factors up to the square root of n (excluding n itself)
fn get_factors(n: usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut divisor = 1;

    while divisor * divisor <= n {
        if n.is_multiple_of(divisor) {
            factors.push(divisor);
            if divisor != 1 && (divisor != n / divisor) {
                factors.push(n / divisor);
            }
        }
        divisor += 1;
    }

    factors
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
}
