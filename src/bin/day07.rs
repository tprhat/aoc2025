use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let grid = aoc::parse_grid(input);
    let mut positions: Vec<(i32, i32)> = vec![];

    //find starting postion of S
    for (i, item) in grid[0].iter().enumerate() {
        if *item == 'S' {
            positions.push((0, i as i32));
            break;
        }
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut splits: HashSet<(i32, i32)> = HashSet::new();

    'outer: while let Some(mut cur_pos) = positions.pop() {
        // check end
        if cur_pos.0 + 1 == grid.len() as i32 {
            continue;
        }
        while grid[(cur_pos.0) as usize][cur_pos.1 as usize] != '^' {
            cur_pos.0 += 1;
            if cur_pos.0 == grid.len() as i32 - 1 {
                continue 'outer;
            }
        }
        splits.insert(cur_pos);
        //split left
        if cur_pos.1 > 0 && !visited.contains(&(cur_pos.0, cur_pos.1 - 1)) {
            visited.insert((cur_pos.0, cur_pos.1 - 1));
            positions.push((cur_pos.0, cur_pos.1 - 1));
        }
        //split right
        if cur_pos.1 < (grid[0].len() - 1) as i32 && !visited.contains(&(cur_pos.0, cur_pos.1 + 1))
        {
            visited.insert((cur_pos.0, cur_pos.1 + 1));
            positions.push((cur_pos.0, cur_pos.1 + 1));
        }
    }
    splits.len()
}

fn part2(input: &str) -> u128 {
    let grid = aoc::parse_grid(input);
    let mut starting: (i32, i32) = (0, 0);

    //find starting postion of S
    for (i, item) in grid[0].iter().enumerate() {
        if *item == 'S' {
            starting = (0, i as i32);
            break;
        }
    }

    //memoization
    let mut memo: HashMap<(i32, i32, char), u128> = HashMap::new();
    count_paths(&grid, starting.0, starting.1, 's', &mut memo)
}

fn count_paths(
    grid: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    dir: char,
    memo: &mut HashMap<(i32, i32, char), u128>,
) -> u128 {
    // out of bounds
    if col < 0 || col > grid[0].len() as i32 - 1 {
        return 0;
    }
    // reached the bottom
    if row == grid.len() as i32 - 1 {
        return 1;
    }

    if memo.contains_key(&(row, col, dir)) {
        return memo[&(row, col, dir)];
    }

    // move down until hitting bottom or a '^'
    let mut cur_pos = (row, col);
    while grid[(cur_pos.0) as usize][cur_pos.1 as usize] != '^' {
        cur_pos.0 += 1;
        // hit bottom
        if cur_pos.0 == grid.len() as i32 - 1 {
            return 1;
        }
    }
    // count paths left + right
    let result = count_paths(grid, cur_pos.0 + 1, col - 1, 'l', memo)
        + count_paths(grid, cur_pos.0 + 1, col + 1, 'r', memo);

    memo.insert((row, col, dir), result);

    result
}

fn main() {
    let input = aoc::read_input(7);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 40);
    }
}
