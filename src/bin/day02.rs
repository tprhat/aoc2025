fn part1(input: &str) -> u128 {
    let ranges = input.trim().split(",");
    let mut total: u128 = 0;
    for r in ranges {
        let (start, end) = r.split_once("-").unwrap();
        let mut start: u128 = start.parse().unwrap();
        let end: u128 = end.parse().unwrap();

        while start <= end {
            let cur_num: String = start.to_string();

            if !cur_num.len().is_multiple_of(2) {
                start += 1;
                continue;
            }

            if cur_num[..cur_num.len() / 2] == cur_num[cur_num.len() / 2..] {
                total += start;
            }

            start += 1;
        }
    }
    total
}

fn part2(input: &str) -> u128 {
    let ranges = input.trim().split(",");
    let mut total: u128 = 0;
    for r in ranges {
        let (start, end) = r.split_once("-").unwrap();
        let mut start: u128 = start.parse().unwrap();
        let end: u128 = end.parse().unwrap();

        while start <= end {
            let cur_num: String = start.to_string();

            'outer: for i in 1..=cur_num.len() / 2 {
                if !cur_num.len().is_multiple_of(i) {
                    continue;
                }
                let mut j = 0;
                while j < cur_num.len() - 1 {
                    // if the pattern does not match just continue to the next
                    if cur_num[j..j + i] != cur_num[j + i..j + i + i] {
                        continue 'outer;
                    }
                    // for exaplem: a pattern of size 3 can only happen
                    // for sequences of length 3 * n where n >= 2
                    //when we reach the end break the loop so it's not checking out of bounds
                    if j + i + i == cur_num.len() {
                        break;
                    }
                    j += 1;
                }
                // if we reach this point we found a good sequence
                total += start;
                break;
            }

            start += 1;
        }
    }
    total
}

fn main() {
    let input = aoc::read_input(2);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
