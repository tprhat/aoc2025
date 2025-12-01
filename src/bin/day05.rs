fn part1(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    0
}

fn part2(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    0
}

fn main() {
    let input = aoc::read_input(5);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
EXAMPLE HERE :D";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
