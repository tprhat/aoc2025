use std::collections::HashMap;

fn part1(input: &str) -> usize {
    let mut total = 0;

    let mut ranges: Vec<&str> = Vec::new();
    let mut products: Vec<&str> = Vec::new();
    let mut is_prod = false;

    for l in aoc::parse_lines(input) {
        if l.is_empty() {
            is_prod = true;
            continue;
        }
        if is_prod {
            products.push(l);
        } else {
            ranges.push(l);
        }
    }

    for p in products {
        let prod: i128 = p.parse().unwrap();
        for r in &ranges {
            let (s, e) = r.split_once("-").unwrap();
            let s: i128 = s.parse().unwrap();
            let e: i128 = e.parse().unwrap();

            //if (s..=e).contains(&prod) {
            if s <= prod && prod <= e {
                total += 1;
                break;
            }
        }
    }
    total
}

fn part2(input: &str) -> i128 {
    let mut total = 0;
    let mut ranges: Vec<&str> = Vec::new();

    for l in aoc::parse_lines(input) {
        if l.is_empty() {
            break;
        }
        ranges.push(l);
    }

    let mut r1: HashMap<i128, i128> = HashMap::new();

    for r in &ranges {
        let (start, end) = r.split_once("-").unwrap();
        let start: i128 = start.parse().unwrap();
        let end: i128 = end.parse().unwrap();

        // in case of 2 identical keys insert the value of the larger one
        r1.entry(start)
            .and_modify(|v| *v = (*v).max(end))
            .or_insert(end);
    }

    let mut change = true;
    'outer: while change {
        change = false;
        let mut keys: Vec<_> = r1.keys().copied().collect();
        keys.sort();

        // keys are sorted in the ascending order so
        // we know that the next key is always larger
        for i in 0..keys.len() {
            for j in i + 1..keys.len() {
                if r1[&keys[i]] >= r1[&keys[j]] {
                    // drop keys[j] since it's
                    // fully inside of keys[i]-r1[keys[i]]
                    r1.remove(&keys[j]);
                    change = true;
                    continue 'outer;
                }
                if r1[&keys[i]] >= keys[j] {
                    // update keys[i] since the keys[i] falls
                    // in the range keys[j]-r1[keys[j]]
                    r1.insert(keys[i], r1[&keys[j]]);
                    change = true;
                    continue 'outer;
                }
            }
        }
    }

    for (k, v) in &r1 {
        total += v - k + 1;
    }

    total
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 14);
    }
}
