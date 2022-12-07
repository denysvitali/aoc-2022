use std::collections::{HashMap, HashSet};
use std::io::{Error};
use std::str::Lines;
use std::fs;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::days::day03::{part_a, part_b, run};

    const EXAMPLE_FILE: &str = "./resources/day03/example.txt";
    const INPUT_FILE: &str = "./resources/day03/input.txt";

    #[test]
    fn example() {
        run("./resources/day03/example.txt").unwrap();
    }

    #[test]
    fn example_part_a(){
        let content = fs::read_to_string(EXAMPLE_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(157, part_a(lines));
    }

    #[test]
    fn example_part_b(){
        let content = fs::read_to_string(EXAMPLE_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(70, part_b(lines));
    }

    #[test]
    fn test_part_a(){
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(7701, part_a(lines));
    }

    #[test]
    fn test_part_b(){
        let content = fs::read_to_string(INPUT_FILE).unwrap();
        let lines = content.lines();
        assert_eq!(2644, part_b(lines));
    }
}

pub fn run(input_file: &str) -> Result<i32, Error>{
    let content = fs::read_to_string(input_file)?;
    let lines = content.lines();

    let a = part_a(lines.clone());
    let b = part_b(lines.clone());
    println!("Part A: {}", a);
    println!("Part B: {}", b);
    Ok(0)
}

fn get_priority(c: char) -> u32 {
    if !c.is_ascii() {
        panic!("non ascii character {}", c);
    }
    if c.is_ascii_uppercase() {
        c as u32 - 'A' as u32 + 27
    } else {
        c as u32 - 'a' as u32 + 1
    }
}

fn char_map(line: &str) -> HashMap<char, bool> {
    let mut hm: HashMap<char, bool> = HashMap::new();
    line.chars().for_each(|x| {
        hm.insert(x, true);
    });
    hm
}

fn get_keys(m: HashMap<char, bool>) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    m.keys().for_each(|&x|{set.insert(x);});
    set
}

fn part_a(lines: Lines) -> u32 {
    let mut priorities = 0;
    for line in lines.into_iter() {
        let m1 = char_map(line.get(0..line.len()/2).unwrap());
        let m2 = char_map(line.get(line.len()/2..).unwrap());
        let k1 = get_keys(m1);
        let k2 = get_keys(m2);

        let intersection: Vec<&char> = k1.intersection(&k2).collect();

        let common_char = intersection.get(0).unwrap();
        let priority = get_priority(**common_char);
        priorities += priority;
    }
    priorities
}

fn part_b(lines: Lines) -> u32 {
    let mut priorities = 0;
    for chunk in lines.collect::<Vec<&str>>().chunks(3) {
        let c1 = char_map(chunk[0]);
        let c2 = char_map(chunk[1]);
        let c3 = char_map(chunk[2]);
        let k1 = get_keys(c1);
        let k2 = get_keys(c2);
        let k3 = get_keys(c3);

        let intersection: HashSet<char> = k1.intersection(&k2).map(|&c|c).collect::<HashSet<char>>();
        let intersection2: Vec<&char> = intersection.intersection(&k3).collect();

        let common_char = intersection2.get(0).unwrap();
        let priority = get_priority(**common_char);
        priorities += priority;
    }
    priorities
}