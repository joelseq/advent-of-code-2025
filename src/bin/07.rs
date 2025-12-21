use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = get_grid(input);

    let (split_count, _) = split_beams(&mut grid);

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = get_grid(input);
    let (_, timelines) = split_beams(&mut grid);

    Some(timelines)
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn split_beams(grid: &mut [Vec<char>]) -> (u64, u64) {
    let mut split_count = 0;
    let mut timeline_counts = HashMap::new();
    let split_col_offsets = [-1, 1];

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cell = grid[row][col];

            match cell {
                // starting point or beam, mark row below as beam
                'S' => {
                    if row + 1 < grid.len() {
                        grid[row + 1][col] = '|';
                        timeline_counts.insert((row + 1, col), 1u64);
                    }
                }
                '^' => {
                    // split the beam if the cell above is a beam
                    if row > 0 && grid[row - 1][col] == '|' {
                        let prev_beam = timeline_counts.get(&(row - 1, col)).unwrap();
                        let prev_beam_val = *prev_beam;
                        split_count += 1;
                        for dx in &split_col_offsets {
                            let new_col = col as isize + dx;
                            if new_col < grid[row].len() as isize
                                && new_col >= 0
                                && grid[row][new_col as usize] != '^'
                            {
                                let key = (row, new_col as usize);
                                let current_beam_val = timeline_counts.get(&key).unwrap_or(&0);
                                timeline_counts.insert(
                                    (row, new_col as usize),
                                    current_beam_val + prev_beam_val,
                                );
                                grid[row][new_col as usize] = '|';
                            }
                        }
                    }
                }
                '.' => {
                    // check if cell above is a beam and if so continue it
                    if row > 0 && grid[row - 1][col] == '|' {
                        grid[row][col] = '|';
                        let prev_beam_val = timeline_counts.get(&(row - 1, col)).unwrap();
                        timeline_counts.insert((row, col), *prev_beam_val);
                    }
                }
                '|' => {
                    // tally timelines from above beam
                    if row > 0 && grid[row - 1][col] == '|' {
                        let current_beam_val = timeline_counts.get(&(row, col)).unwrap_or(&0);
                        let prev_beam_val = timeline_counts.get(&(row - 1, col)).unwrap();
                        timeline_counts.insert((row, col), current_beam_val + *prev_beam_val);
                    }
                }
                _ => {} // do nothing for other cells
            }
        }
    }

    let mut total_timelines = 0;

    let last_row = grid.len() - 1;

    for col in 0..grid[last_row].len() {
        if let Some(&count) = timeline_counts.get(&(last_row, col)) {
            total_timelines += count;
        }
    }

    (split_count, total_timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
