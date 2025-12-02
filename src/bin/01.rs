use std::num::ParseIntError;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = Dial::new();

    let mut zero_count = 0;

    input.lines().for_each(|line| {
        dial.rotate(line).unwrap();

        if dial.0 == 0 {
            zero_count += 1;
        }
    });

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = Dial::new();

    let mut zero_count = 0;
    let mut clicks = 0;

    input.lines().for_each(|line| {
        clicks += dial.rotate(line).unwrap();

        if dial.0 == 0 {
            zero_count += 1;
        }
    });

    Some(clicks + zero_count)
}

struct Dial(i16);

impl Dial {
    fn new() -> Self {
        Self(50)
    }

    fn rotate(&mut self, rotation: &str) -> Result<u64, ParseIntError> {
        let (direction, amount_str) = rotation.split_at(1);

        let mut clicks = 0;
        let amount = amount_str.parse::<i16>()?;

        clicks += (amount / 100) as u64;

        let rotate_amount = amount % 100;
        let start_position = self.0;

        match direction {
            "L" => {
                self.0 -= rotate_amount;

                if self.0 < 0 {
                    self.0 += 100;
                    // Only increment clicks if we crossed 0 during the rotation
                    // and not if it was already on 0.
                    if start_position != 0 {
                        clicks += 1;
                    }
                }
            }
            "R" => {
                self.0 += rotate_amount;

                if self.0 > 99 {
                    self.0 -= 100;
                    // Only increment clicks if we crossed 0 during the rotation
                    // and not if it was already on 0 or we ended on zero.
                    if start_position != 0 && self.0 != 0 {
                        clicks += 1;
                    }
                }
            }
            _ => panic!("Invalid direction"),
        }

        Ok(clicks)
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
        assert_eq!(result, Some(6));
    }
}
