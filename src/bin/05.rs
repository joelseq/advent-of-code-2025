use std::cmp;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut checker = FreshnessChecker::default();

    let mut fresh_count = 0;

    input.lines().for_each(|line| {
        if line.contains("-") {
            checker.add_range(get_range(line));
        } else if !line.is_empty() {
            let value = line.parse::<u64>().unwrap();
            if checker.is_fresh(value) {
                fresh_count += 1;
            }
        }
    });

    Some(fresh_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut checker = FreshnessChecker::default();

    input.lines().for_each(|line| {
        if line.contains("-") {
            checker.add_range(get_range(line));
        }
    });

    checker.merge_ranges();

    Some(checker.fresh_count())
}

fn get_range(line: &str) -> (u64, u64) {
    let mut parts = line.split("-");
    let start = parts.next().unwrap().parse::<u64>().unwrap();
    let end = parts.next().unwrap().parse::<u64>().unwrap();

    (start, end)
}

#[derive(Debug, Default)]
struct FreshnessChecker {
    ranges: Vec<(u64, u64)>,
}

impl FreshnessChecker {
    fn add_range(&mut self, range: (u64, u64)) {
        self.ranges.push(range)
    }

    fn merge_ranges(&mut self) {
        let mut merged_ranges = vec![];

        self.ranges.sort_unstable();
        let first = self.ranges.first().expect("No ranges found");

        merged_ranges.push((first.0, first.1));

        for (start, end) in &self.ranges[1..] {
            let last_merged_range = merged_ranges.last_mut().expect("No merged ranges found");

            // If the start is LTE the end of the last merged range, then merge
            if *start <= last_merged_range.1 {
                last_merged_range.1 = cmp::max(last_merged_range.1, *end);
            } else {
                merged_ranges.push((*start, *end));
            }
        }

        self.ranges = merged_ranges;
    }

    fn is_fresh(&self, value: u64) -> bool {
        for (start, end) in &self.ranges {
            if value >= *start && value <= *end {
                return true;
            }
        }

        false
    }

    fn fresh_count(&self) -> u64 {
        let mut count = 0;

        for (start, end) in &self.ranges {
            count += end + 1 - start
        }

        count
    }
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
}
