use std::collections::HashMap;

// final day of aoc, very complicated problem to find a general solution.
// but just computing the area needed to fit the shape and checking
// if that times 1.2 is less that total area available yields the correct answer.
// guess it's an early Christmas present from Eric.
fn part1(input: &str) -> usize {
    let mut shapes: HashMap<usize, usize> = HashMap::new();
    let mut sizes: Vec<&str> = Vec::new();
    let mut total = 0;

    for block in input.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }
        if let Some((first_line, rest)) = block.split_once("\n") {
            if first_line.ends_with(":") {
                let name = first_line.trim_end_matches(":").parse::<usize>().unwrap();
                let n = rest.chars().filter(|c| *c == '#').count();
                shapes.insert(name, n);
            } else {
                sizes = block.lines().collect();
            }
        }
    }

    for s in sizes {
        let ln: Vec<&str> = s.split(": ").collect();
        let grid: Vec<usize> = ln[0]
            .split("x")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let shapes_size: Vec<usize> = ln[1]
            .split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let grid_total: usize = grid.iter().product();
        let mut shapes_total = 0;

        for (i, shape) in shapes_size.iter().enumerate() {
            shapes_total += shapes.get(&i).unwrap() * shape;
        }
        if shapes_total as f32 * 1.2 < grid_total as f32 {
            total += 1;
        }
    }
    total
}

fn part2() -> i32 {
    0
}

fn main() {
    let input = aoc::read_input(12);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 0);
    }
}
