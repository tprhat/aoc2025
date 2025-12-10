use good_lp::{
    Expression, Solution, SolverModel, Variable, constraint, microlp, variable, variables,
};
use itertools::Itertools;
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    // button pressed 2 times== button pressed 0 times
    // optimize for fewest buttons pressed once
    let lines = aoc::parse_lines(input);
    let mut total = 0;

    for line in lines {
        let pieces: Vec<&str> = line.split_whitespace().collect();
        let toggles: Vec<Vec<i32>> = pieces[1..&pieces.len() - 1]
            .iter()
            .map(|nums| {
                nums[1..nums.len() - 1]
                    .split(",")
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        let diagram: Vec<i32> = pieces[0][1..pieces[0].len() - 1]
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '#' { Some(i as i32) } else { None })
            .collect();

        'outer: for size in 1..=toggles.len() {
            for combo in toggles.iter().combinations(size) {
                let flattened: Vec<i32> = combo.into_iter().flatten().copied().collect();
                let mut hashmap: HashMap<i32, usize> = HashMap::new();

                for f in flattened {
                    hashmap.entry(f).and_modify(|e| *e += 1).or_insert(1);
                }
                let mut lst: Vec<i32> = vec![];
                for (k, v) in hashmap.iter() {
                    // must not be even since that turns off the light
                    if v % 2 == 1 {
                        lst.push(*k);
                    }
                }
                lst.sort();
                if lst == diagram {
                    total += size;
                    break 'outer;
                }
            }
        }
    }

    total
}

// ilp problem
fn part2(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    let mut total = 0;
    for line in lines {
        let pieces: Vec<&str> = line.split_whitespace().collect();

        let toggles: Vec<Vec<usize>> = pieces[1..&pieces.len() - 1]
            .iter()
            .map(|nums| {
                nums[1..nums.len() - 1]
                    .split(",")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let joltage: Vec<i32> = pieces[pieces.len() - 1][1..pieces[pieces.len() - 1].len() - 1]
            .split(",")
            .map(|c| c.parse::<i32>().unwrap())
            .collect();

        total += solve_ilp(&joltage, &toggles);
    }
    total
}

fn solve_ilp(targets: &[i32], buttons: &[Vec<usize>]) -> i32 {
    variables! {
        problem:
    }

    let button_vars: Vec<Variable> = buttons
        .iter()
        .map(|_| problem.add(variable().integer().min(0)))
        .collect();

    let objective: Expression = button_vars.iter().copied().sum();

    let mut constraints = Vec::new();
    for (i, &target) in targets.iter().enumerate() {
        let lhs: Expression = buttons
            .iter()
            .enumerate()
            .filter(|(_, btn)| btn.contains(&i))
            .map(|(b, _)| button_vars[b])
            .sum();

        constraints.push(constraint!(lhs == target));
    }

    let mut model = problem.minimise(objective).using(microlp);

    for c in constraints {
        model.add_constraint(c);
    }
    let solution = model.solve().unwrap();

    button_vars
        .iter()
        // solution value returns stupid shit since it's a LP solver
        // MOST ROUND() values so it gets the correct values
        // using "as i32" rounds down to values --> 2.999999999 is 2 instead of 3
        .map(|&v| solution.value(v).round() as i32)
        .sum()
}

fn main() {
    let input = aoc::read_input(10);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 33);
    }
}
