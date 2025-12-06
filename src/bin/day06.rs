fn part1(input: &str) -> i128 {
    let lines = aoc::parse_lines(input);
    let mut total = 0;
    let grid: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    for j in 0..grid[0].len() {
        let mut res: i128;
        let mut is_add = true;

        // prep the res
        if grid[grid.len() - 1][j] == "+" {
            res = 0;
        } else {
            res = 1;
            is_add = false;
        }

        for row in grid.iter().take(grid.len() - 1) {
            if is_add {
                res += row[j].parse::<i128>().unwrap();
            } else {
                res *= row[j].parse::<i128>().unwrap();
            }
        }
        total += res;
    }

    total
}

fn part2(input: &str) -> i128 {
    let grid = aoc::parse_grid(input);
    let mut total = 0;
    let mut nums: Vec<i128> = Vec::new();

    // the idea is to create numbers by checking columns from right to left
    // and either multiply or product based on the sign
    for j in (0..grid[0].len()).rev() {
        let mut num = 0;
        for row in grid.iter().take(grid.len() - 1) {
            if row[j] != ' ' {
                num = num * 10 + row[j].to_digit(10).unwrap();
            }
        }
        if num != 0 {
            nums.push(num as i128);
            // edge case: reaching the leftmost position -> take the j = 0 sign
            if j == 0 {
                if grid[grid.len() - 1][j] == '+' {
                    total += nums.iter().sum::<i128>();
                } else {
                    total += nums.iter().product::<i128>();
                }
            }
            continue;
        }
        if grid[grid.len() - 1][j + 1] == '+' {
            total += nums.iter().sum::<i128>();
        } else {
            total += nums.iter().product::<i128>();
        }
        nums.clear();
    }

    total
}

fn main() {
    let input = aoc::read_input(6);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
