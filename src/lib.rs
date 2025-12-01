// Common utilities for Advent of Code solutions

use std::fs;

/// Read input file for a given day
pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

/// Parse input into lines
pub fn parse_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

/// Parse input into a grid of characters
pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Parse input into numbers (one per line)
pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse number"))
        .collect()
}

/// Common grid directions (up, down, left, right)
pub const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

/// All 8 directions including diagonals
pub const DIRECTIONS_8: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

/// Check if a position is within grid bounds
pub fn in_bounds(row: i32, col: i32, rows: usize, cols: usize) -> bool {
    row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "line1\nline2\nline3";
        let lines = parse_lines(input);
        assert_eq!(lines, vec!["line1", "line2", "line3"]);
    }

    #[test]
    fn test_in_bounds() {
        assert!(in_bounds(0, 0, 10, 10));
        assert!(in_bounds(9, 9, 10, 10));
        assert!(!in_bounds(-1, 0, 10, 10));
        assert!(!in_bounds(10, 0, 10, 10));
    }
}
