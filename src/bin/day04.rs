fn part1(input: &str) -> i32 {
    let grid = aoc::parse_grid(input);
    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' {
                continue;
            }
            let mut cnt = 0;
            for d in aoc::DIRECTIONS_8 {
                if aoc::in_bounds(i as i32 + d.0, j as i32 + d.1, grid.len(), grid[0].len())
                    && grid[(i as i32 + d.0) as usize][(j as i32 + d.1) as usize] == '@'
                {
                    cnt += 1;
                }
            }
            if cnt < 4 {
                total += 1;
            }
        }
    }
    total
}

fn part2(input: &str) -> i32 {
    let mut grid = aoc::parse_grid(input);
    let mut total = 0;
    let mut change = true;

    while change {
        change = false;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '.' {
                    continue;
                }
                let mut cnt = 0;
                for d in aoc::DIRECTIONS_8 {
                    if aoc::in_bounds(i as i32 + d.0, j as i32 + d.1, grid.len(), grid[0].len())
                        && grid[(i as i32 + d.0) as usize][(j as i32 + d.1) as usize] == '@'
                    {
                        cnt += 1;
                    }
                }
                if cnt < 4 {
                    grid[i][j] = '.';
                    total += 1;
                    change = true;
                }
            }
        }
    }
    total
}

fn main() {
    let input = aoc::read_input(4);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 43);
    }
}
