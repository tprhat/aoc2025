use std::collections::HashMap;

/// calculate n of different paths from starting postion
fn part1(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);

    let mut total = 0;
    let mut hashmap: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in lines {
        let paths: Vec<&str> = l.split(": ").collect::<Vec<&str>>();
        let key = paths[0];
        for path in paths[1].split_whitespace() {
            hashmap
                .entry(key)
                .and_modify(|v| v.push(path))
                .or_insert(vec![path]);
        }
    }
    let mut queue = vec![];
    //starting position
    for v in hashmap.get("you").unwrap() {
        queue.push(v);
    }

    'outer: while let Some(q) = queue.pop() {
        for p in hashmap.get(q).unwrap() {
            if *p == "out" {
                total += 1;
                continue 'outer;
            }
            queue.push(p);
        }
    }
    total
}

/// there were too many paths from "svr" to "out" to improve the part 1 solution
/// so idea was to recursively check paths and cache the results for each branch
fn part2(input: &str) -> u128 {
    let lines = aoc::parse_lines(input);
    let mut hashmap: HashMap<&str, Vec<&str>> = HashMap::new();

    for l in lines {
        let paths: Vec<&str> = l.split(": ").collect::<Vec<&str>>();
        let key = paths[0];
        for path in paths[1].split_whitespace() {
            hashmap
                .entry(key)
                .and_modify(|v| v.push(path))
                .or_insert(vec![path]);
        }
    }
    let mut cache: HashMap<(&str, bool, bool), u128> = HashMap::new();
    solve(&hashmap, &mut cache, "svr", false, false)
}

fn solve<'a>(
    map: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<(&'a str, bool, bool), u128>,
    x: &'a str,
    fft: bool,
    dac: bool,
) -> u128 {
    // if in cache return value;
    if let Some(s) = cache.get(&(x, fft, dac)) {
        return *s;
    }
    // base case
    if x == "out" {
        if fft && dac {
            return 1;
        } else {
            return 0;
        };
    }
    let mut ans = 0;
    // go over all brances and sum their results
    for y in map.get(x).unwrap() {
        let new_dac = dac || *y == "dac";
        let new_fft = fft || *y == "fft";
        ans += solve(map, cache, y, new_fft, new_dac);
    }
    cache.insert((x, fft, dac), ans);
    ans
}

fn main() {
    let input = aoc::read_input(11);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE_2), 2);
    }
}
