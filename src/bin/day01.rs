fn part1(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    let mut curr_dial: i32 = 50;
    let mut cnt = 0;

    for l in lines {
        let side = &l[0..1];
        let mut steps: i32 = match l[1..].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!(),
        };
        steps %= 100;
        if side == "L" {
            curr_dial -= steps;
        } else {
            curr_dial += steps;
        }

        if curr_dial < 0 {
            curr_dial += 100;
        }
        if curr_dial > 99 {
            curr_dial -= 100;
        }

        if curr_dial == 0 {
            cnt += 1;
        }
    }
    cnt
}

fn part2(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    let mut curr_dial: i32 = 50;
    let mut cnt = 0;

    for l in lines {
        let side = &l[0..1];
        let mut steps: i32 = match l[1..].trim().parse() {
            Ok(num) => num,
            Err(_) => panic!(),
        };
        cnt += steps / 100;
        steps %= 100;

        // if the dial is currently at zero and moves left (to negative values)
        // we reduce cnt by one temporarily since the code below increments it regardless
        // of the fact that it is not going "over" 0 again.
        if curr_dial == 0 && side == "L" {
            cnt -= 1;
        }

        if side == "L" {
            curr_dial -= steps;
        } else {
            curr_dial += steps;
        }

        if curr_dial < 0 {
            curr_dial += 100;
            cnt += 1;
        }

        if curr_dial > 99 {
            curr_dial -= 100;
            // don't increment the value if it points to 0 here,
            // this is only for passing "over" 0.
            if curr_dial != 0 {
                cnt += 1;
            }
        }

        if curr_dial == 0 {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let input = aoc::read_input(1);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 6);
    }
}
