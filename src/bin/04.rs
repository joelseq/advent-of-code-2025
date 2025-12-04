advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = create_grid(input);

    Some(count_rolls(&mut grid, false))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = create_grid(input);

    let mut total_count = 0;
    let mut count = count_rolls(&mut grid, true);

    while count > 0 {
        total_count += count;
        count = count_rolls(&mut grid, true);
    }

    Some(total_count)
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_rolls(grid: &mut [Vec<char>], remove: bool) -> u64 {
    let cols = grid[0].len();
    let rows = grid.len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            let cell = grid[r][c];

            if cell == '@' && check_surroundings(grid, r, c) {
                count += 1;
                if remove {
                    grid[r][c] = 'x';
                }
            }
        }
    }

    count
}

fn check_surroundings(grid: &[Vec<char>], r: usize, c: usize) -> bool {
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut rolls = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for offset in offsets.iter() {
        let new_r = r as isize + offset.0;
        let new_c = c as isize + offset.1;

        if new_r >= 0 && new_r < rows && new_c >= 0 && new_c < cols {
            let neighbor = grid[new_r as usize][new_c as usize];
            if neighbor == '@' {
                rolls += 1;
            }
        }
    }

    rolls < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
