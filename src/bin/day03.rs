use std::collections::HashMap;

fn part1(input: &str) -> u32 {
    let lines = aoc::parse_lines(input);
    let mut total = 0;
    for l in lines {
        let mut map: HashMap<u32, Vec<usize>> = HashMap::new();

        for (pos, c) in l.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            map.entry(digit).or_default().push(pos);
        }

        let max_key = map.keys().max().unwrap();

        if map[max_key].len() > 1 {
            total += max_key * 10 + max_key;
            continue;
        }
        // if the largest element is at the end, it's is definitely the second number
        if map[max_key][0] != l.len() - 1 {
            let first = *max_key;
            let first_idx = map[max_key][0];
            let mut second_max = 0;

            for i in l[first_idx + 1..].chars() {
                let digit = i.to_digit(10).unwrap();
                if digit > second_max {
                    second_max = digit;
                }
            }
            total += first * 10 + second_max;
        } else {
            let second = *max_key;

            let second_idx = map[max_key].iter().max().unwrap();
            let mut first_max = 0;
            for i in l[..=second_idx - 1].chars() {
                let digit = i.to_digit(10).unwrap();
                if digit > first_max {
                    first_max = digit;
                }
            }
            total += first_max * 10 + second;
        }
    }
    total
}

fn part2(input: &str) -> i128 {
    let lines = aoc::parse_lines(input);
    let mut total = 0;
    for l in lines {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for (pos, c) in l.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            map.entry(digit.try_into().unwrap())
                .or_default()
                .push(pos as i32);
        }
        let mut largest: Vec<i32> = Vec::new();
        let mut cur_pos: i32 = -1;
        let mut i = 9;
        // we have a map where keys are number that appear in the line
        // and the values and all the positions for those keys
        // the idea is to go through all the keys, starting at the largest (9)
        // and trying to find the largest 12 digit number by putting the
        // largest in each position if there is enough space after it.
        // e.g. 9[9]0000000010 > 9[8]9999999999
        // if we look at the number in brackets it is ALWAYS more benefitial
        // to have larger numbers more to the left
        'outer: while i > 0 {
            if !map.contains_key(&i) {
                i -= 1;
                continue;
            }
            for pos in &map[&i] {
                // check if this position is more to the right than
                // the number we are trying to push
                if cur_pos >= *pos {
                    continue;
                }
                if l.len() as i32 - pos >= 12 - largest.len() as i32 {
                    largest.push(i);
                    cur_pos = *pos;
                    if largest.len() == 12 {
                        break 'outer;
                    }
                    i = 9;
                    continue 'outer;
                }
                if largest.len() == 12 {
                    break 'outer;
                }
            }
            i -= 1;
            if largest.len() == 12 {
                break;
            }
        }

        let mut t: i128 = 0;
        for n in largest {
            t = t * 10 + n as i128;
        }
        total += t;
    }
    total
}

fn main() {
    let input = aoc::read_input(3);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3121910778619);
    }
}
