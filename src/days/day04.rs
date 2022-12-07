use std::cmp::{max, min};
use std::fs;
use std::io::{Error};
use std::str::Lines;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::days::day04::{part_a, part_b};

    const EXAMPLE_FILE: &str = "./resources/day04/example.txt";
    const INPUT_FILE: &str = "./resources/day04/input.txt";

    #[test]
    fn example_part_a() {
        let content = fs::read_to_string(EXAMPLE_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(2, part_a(lines));
    }

    #[test]
    fn example_part_b() {
        let content = fs::read_to_string(EXAMPLE_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(4, part_b(lines));
    }

    #[test]
    fn test_part_a() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(524, part_a(lines));
    }

    #[test]
    fn test_part_b() {
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(798, part_b(lines));
    }
}

pub fn run(input_file: &str) -> Result<i32, Error> {
    let content = fs::read_to_string(input_file)?;
    let lines = content.lines();

    let a = part_a(lines.clone());
    let b = part_b(lines.clone());
    println!("Part A: {}", a);
    println!("Part B: {}", b);
    Ok(0)
}

fn parse_range(range_str: &str) -> (i32, i32) {
    let parts: Vec<i32> = range_str.split("-").map(|x| x.parse().unwrap()).collect();
    (parts[0], parts[1])
}

// A function that takes a list of ranges and returns the number of pairs
// where one range fully contains the other
fn count_overlapping_pairs(ranges: &Vec<(i32, i32)>) -> usize {
    let mut count = 0;

    // Loop through each pair of ranges
    for i in ranges.chunks(2) {
        // Parse the two ranges
        let range1 = i[0];
        let range2 = i[1];

        // If one range fully contains the other, increment the count
        if range1.0 <= range2.0 && range1.1 >= range2.1 {
            count += 1;
        } else if range2.0 <= range1.0 && range2.1 >= range1.1 {
            count += 1;
        }
    }

    // Return the final count
    count
}

fn ranges_overlap(range1: (i32, i32), range2: (i32, i32)) -> bool {
    // Check if the two ranges overlap
    let min1 = min(range1.0, range1.1);
    let max1 = max(range1.0, range1.1);
    let min2 = min(range2.0, range2.1);
    let max2 = max(range2.0, range2.1);

    // If the two ranges have a non-empty intersection, they overlap
    min2 <= max1 && min1 <= max2
}

fn count_partially_overlapping_pairs(ranges: &Vec<(i32, i32)>) -> usize {
    let mut count = 0;
    for i in ranges.chunks(2) {
        let first = i[0];
        let second = i[1];
        if ranges_overlap(first, second) {
            count += 1;
        }
    }
    count
}

fn get_ranges(lines: Lines) -> Vec<(i32, i32)> {
    lines
        .flat_map(|l| l.split(","))
        .map(|x| parse_range(x))
        .collect()
}

fn part_a(lines: Lines) -> u32 {
    count_overlapping_pairs(&get_ranges(lines)) as u32
}

fn part_b(lines: Lines) -> u32 {
    count_partially_overlapping_pairs(&get_ranges(lines)) as u32
}