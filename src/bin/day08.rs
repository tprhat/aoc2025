use std::collections::{BinaryHeap, HashMap};

#[cfg(test)]
const NUM_CONNECTIONS: usize = 10;

#[cfg(not(test))]
const NUM_CONNECTIONS: usize = 1000;

type Point = (i32, i32, i32);

fn calc_distance(a: Point, b: Point) -> u128 {
    let dx = (a.0 - b.0) as f32;
    let dy = (a.1 - b.1) as f32;
    let dz = (a.2 - b.2) as f32;
    // no need to sqrt since it only brings complexity
    // for f32 type and sorting of heap
    (dx * dx + dy * dy + dz * dz) as u128
}

fn part1(input: &str) -> usize {
    let lines = aoc::parse_lines(input);
    let points: Vec<Point> = lines
        .iter()
        .map(|s| {
            let mut parts = s.split(",").map(|n| n.parse::<i32>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect();

    let mut dist: BinaryHeap<(u128, i32, i32)> = BinaryHeap::new();

    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            // skip pairs that have been seen
            if i >= j {
                continue;
            }
            dist.push((calc_distance(*p1, *p2), i as i32, j as i32));
        }
    }
    let mut circuits: HashMap<i32, i32> = HashMap::new(); //k: i, circuit_id
    let mut circuit_idx = 0;
    let dist = dist.into_sorted_vec();

    // union find
    for d in dist.iter().take(NUM_CONNECTIONS) {
        // cool way to get value or None for match
        let p1 = circuits.get(&d.1).copied();
        let p2 = circuits.get(&d.2).copied();

        match (p1, p2) {
            (Some(c1), Some(c2)) if c1 == c2 => continue,
            // if both points belong to different circuits
            // merge them together
            (Some(c1), Some(c2)) => {
                for v in circuits.values_mut() {
                    if *v == c2 {
                        *v = c1;
                    }
                }
            }
            // if only one belong to a circuit add the other to it
            (Some(c1), None) => {
                circuits.insert(d.2, c1);
            }
            // if only one belong to a circuit add the other to it
            (None, Some(c2)) => {
                circuits.insert(d.1, c2);
            }
            // if none belong create a new circuit
            (None, None) => {
                circuits.insert(d.1, circuit_idx);
                circuits.insert(d.2, circuit_idx);
                circuit_idx += 1;
            }
        }
    }
    let mut circuit_sizes: HashMap<i32, usize> = HashMap::new();
    for &id in circuits.values() {
        *circuit_sizes.entry(id).or_insert(0) += 1;
    }

    let mut groups: Vec<(i32, usize)> = circuit_sizes.into_iter().collect();
    groups.sort_by_key(|(_, size)| std::cmp::Reverse(*size));

    groups[0].1 * groups[1].1 * groups[2].1
}

fn count_circuits(circuits: &HashMap<i32, i32>) -> usize {
    let mut circuit_sizes: HashMap<i32, usize> = HashMap::new();
    for &id in circuits.values() {
        *circuit_sizes.entry(id).or_insert(0) += 1;
    }
    circuit_sizes.len()
}
fn part2(input: &str) -> u128 {
    let lines = aoc::parse_lines(input);
    let points: Vec<Point> = lines
        .iter()
        .map(|s| {
            let mut parts = s.split(",").map(|n| n.parse::<i32>().unwrap());
            (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            )
        })
        .collect();

    let mut dist: BinaryHeap<(u128, i32, i32)> = BinaryHeap::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i >= j {
                continue;
            }
            let cur_dist = calc_distance(*p1, *p2);
            dist.push((cur_dist, i as i32, j as i32));
        }
    }
    let mut circuits: HashMap<i32, i32> = HashMap::new();
    let mut circuit_idx = 0;
    let dist = dist.into_sorted_vec();
    let mut total: u128 = 0;
    for d in dist {
        let p1 = circuits.get(&d.1).copied();
        let p2 = circuits.get(&d.2).copied();
        match (p1, p2) {
            (Some(c1), Some(c2)) if c1 == c2 => continue,
            (Some(c1), Some(c2)) => {
                for v in circuits.values_mut() {
                    if *v == c2 {
                        *v = c1;
                    }
                }
            }
            (Some(c1), None) => {
                circuits.insert(d.2, c1);
            }
            (None, Some(c2)) => {
                circuits.insert(d.1, c2);
            }
            (None, None) => {
                circuits.insert(d.1, circuit_idx);
                circuits.insert(d.2, circuit_idx);
                circuit_idx += 1;
            }
        }
        // all the points are in the same circuit
        if circuits.len() == points.len() && count_circuits(&circuits) == 1 {
            total = points[d.1 as usize].0 as u128 * points[d.2 as usize].0 as u128;
            break;
        }
    }
    total
}

fn main() {
    let input = aoc::read_input(8);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 25272);
    }
}
