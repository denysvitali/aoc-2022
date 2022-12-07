use core::str;
use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::Error;
use std::str::Lines;

use regex::{Captures, Regex};

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::fs;
    use std::str::Lines;

    use crate::days::day06::{parse_input, part_a, part_b};

    const EXAMPLE_FILE: &str = "./resources/day06/example.txt";
    const INPUT_FILE: &str = "./resources/day06/input.txt";

    #[test]
    fn example_part_a() {
        assert_eq!(7, part_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, part_a("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, part_a("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, part_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, part_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn example_part_b() {
        assert_eq!(19, part_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, part_b("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, part_b("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, part_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, part_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_part_a() {
        assert_eq!(1651, part_a(&parse_input(INPUT_FILE)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(3837, part_b(&parse_input(INPUT_FILE)));
    }

}

fn parse_input(input_file: &str) -> String {
    fs::read_to_string(input_file).unwrap()
}

pub fn run(input_file: &str) -> Result<i32, Error> {
    let buffer = parse_input(input_file);
    let a = part_a(&buffer);
    let b = part_b(&buffer);
    println!("Part A: {}", a);
    println!("Part B: {}", b);
    Ok(0)
}

fn unique(buffer: &str) -> bool {
    let mut hm: HashMap<char, bool> = HashMap::new();
    for k in buffer.chars() {
        if hm.contains_key(&k) {
            return false
        }
        hm.insert(k, true);
    }
    return true
}

fn part_a(buffer: &str) -> u32 {
    for idx in 4..buffer.len()+1 {
        if unique(&buffer[idx-4..idx]) {
            return idx as u32
        }
    }
    return 0
}

fn part_b(buffer: &str) -> u32 {
    for idx in 14..buffer.len()+1 {
        if unique(&buffer[idx-14..idx]) {
            return idx as u32
        }
    }
    return 0
}