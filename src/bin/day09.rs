use std::collections::HashMap;

fn part1(input: &str) -> u128 {
    let lines = aoc::parse_lines(input);
    let nums: Vec<(i32, i32)> = lines
        .iter()
        .map(|l| {
            let mut num = l.split(",").map(|n| n.parse::<i32>().unwrap());
            (num.next().unwrap(), num.next().unwrap())
        })
        .collect();
    let mut largest_area: u128 = 0;

    for (i, n1) in nums.iter().enumerate() {
        for (j, n2) in nums.iter().enumerate() {
            if i >= j {
                continue;
            }
            let area: u128 = ((n1.0 - n2.0).unsigned_abs() + 1) as u128
                * ((n1.1 - n2.1).unsigned_abs() + 1) as u128;
            if area > largest_area {
                largest_area = area;
            }
        }
    }
    largest_area
}

fn part2(input: &str) -> i32 {
    let lines = aoc::parse_lines(input);
    let red_tiles: Vec<(i32, i32)> = lines
        .iter()
        .map(|l| {
            let mut num = l.split(",").map(|n| n.parse::<i32>().unwrap());
            (num.next().unwrap(), num.next().unwrap())
        })
        .collect();

    let ranges = get_valid_ranges(&red_tiles);

    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in (i + 1)..red_tiles.len() {
            let (x1, y1) = red_tiles[i];
            let (x2, y2) = red_tiles[j];

            if is_rect_valid(x1, y1, x2, y2, &ranges) {
                let area = ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1);
                max_area = max_area.max(area);
            }
        }
    }
    max_area
}

/// Computes valid x-ranges for each y-coordinate using the scanline algorithm.
///
/// For the polygon defined by vertices, returns a HashMap where:
/// - Key: y-coordinate
/// - Value: list of (x_start, x_end) ranges that are inside or on the boundary
///
/// Example polygon:
/// ```
///        7,1----11,1
///          |      |
///    2,3--7,3     |
///     |           |
///    2,5----9,5   |
///            |    |
///         9,7---11,7
/// ```
///
/// At y=4, the valid range is x=[2,9] (inside the polygon)
/// At y=2, the valid range is x=[7,11] (inside the upper part)
fn get_valid_ranges(vertices: &[(i32, i32)]) -> HashMap<i32, Vec<(i32, i32)>> {
    // for each y store sorted list of x-ranges
    let mut ranges: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let min_y = vertices.iter().map(|p| p.1).min().unwrap();
    let max_y = vertices.iter().map(|p| p.1).max().unwrap();
    let mut edges: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for i in 0..vertices.len() {
        edges.push((vertices[i], vertices[(i + 1) % vertices.len()]));
    }
    // Process each horizontal scanline
    for y in min_y..=max_y {
        let mut segments = Vec::new();

        // Step 1: Add horizontal edges that lie exactly on this scanline
        //
        // Example: At y=1, the edge (7,1)->(11,1) adds segment (7,11)
        // These are boundary segments that are part of the valid region
        for &((x1, y1), (x2, y2)) in &edges {
            if y1 == y2 && y1 == y {
                let (lo, hi) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                segments.push((lo, hi));
            }
        }
        // Step 2: Find where vertical edges cross this scanline
        //
        // The scanline algorithm: as we sweep horizontally, each time we
        // cross a vertical edge, we toggle between inside/outside.
        //
        // Example at y=4:
        //   - Vertical edge at x=2 (from y=3 to y=5): crossing!
        //   - Vertical edge at x=9 (from y=5 to y=7): crossing!
        //
        //   Crossings: [2, 9]
        //   Between x=2 and x=9, we're inside the polygon
        //
        // The condition `lo < y && y <= hi` ensures:
        //   - We count the TOP endpoint (y <= hi)
        //   - We DON'T count the BOTTOM endpoint (lo < y)
        //   - This prevents double-counting at corners
        let mut crossings: Vec<i32> = Vec::new();
        for &((x1, y1), (x2, y2)) in &edges {
            //hit a vertical edge
            if x1 == x2 {
                let (lo, hi) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                if lo < y && y <= hi {
                    crossings.push(x1);
                }
            }
        }
        crossings.sort();
        // Step 3: Sort crossings and pair them into interior segments
        //
        // After sorting: [2, 9]
        // Pair into chunks: (2, 9) means x=[2,9] is inside
        //
        // If we had 4 crossings [1, 3, 7, 10]:
        //   - (1,3): inside
        //   - (7,10): inside
        //   - x=[3,7]: outside (between the two interior regions)
        for chunk in crossings.chunks(2) {
            if chunk.len() == 2 {
                segments.push((chunk[0], chunk[1]));
            }
        }
        segments.sort();
        //merge overlapping segments
        let merged = merge_segments(segments);
        ranges.insert(y, merged);
    }
    ranges
}

fn merge_segments(segments: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    if segments.is_empty() {
        return vec![];
    }
    let mut result = vec![segments[0]];
    for &(lo, hi) in &segments[1..] {
        let last = result.last_mut().unwrap();
        if lo <= last.1 {
            // New segment starts before or at the end of last segment
            // Extend the last segment to cover both
            //
            // Before: last = (2, 5), new = (4, 9)
            // After:  last = (2, 9)
            last.1 = last.1.max(hi);
        } else {
            result.push((lo, hi));
        }
    }
    result
}

fn is_rect_valid(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    ranges: &HashMap<i32, Vec<(i32, i32)>>,
) -> bool {
    let min_x = x1.min(x2);
    let min_y = y1.min(y2);
    let max_x = x1.max(x2);
    let max_y = y1.max(y2);

    for y in min_y..=max_y {
        let Some(row_ranges) = ranges.get(&y) else {
            return false;
        };
        let covered = row_ranges
            .iter()
            .any(|&(lo, hi)| lo <= min_x && max_x <= hi);
        if !covered {
            return false;
        }
    }
    true
}

fn main() {
    let input = aoc::read_input(9);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 24);
    }
}
