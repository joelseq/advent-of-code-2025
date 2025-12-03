advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let num_batteries = 2;
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>());

    let sum = lines
        .map(|line| find_max_joltage(&line[..], num_batteries))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_batteries = 12;
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>());

    let sum = lines
        .map(|line| find_max_joltage(&line[..], num_batteries))
        .sum();

    Some(sum)
}

fn find_max_joltage(batteries: &[char], num_batteries: usize) -> u64 {
    if num_batteries < 1 {
        return 0;
    }

    // We need at least num_batteries worth of digits so we need to find the max
    // until batteries.len() - (num_batteries - 1)
    let end_index = batteries.len() - (num_batteries - 1);

    let (max, max_index) = find_max_battery(&batteries[0..end_index]);

    let jolts = (max as u64) * 10_u64.pow((num_batteries as u32) - 1);

    jolts + find_max_joltage(&batteries[max_index + 1..], num_batteries - 1)
}

fn find_max_battery(batteries: &[char]) -> (u32, usize) {
    let mut max = 0;
    let mut max_index = 0;

    for (i, battery) in batteries.iter().enumerate() {
        let jolts = battery
            .to_digit(10)
            .expect("Expected battery to be integer");

        if jolts > max {
            max = jolts;
            max_index = i;
        }
    }

    (max, max_index)
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
}
