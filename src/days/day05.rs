use core::str;


use std::collections::VecDeque;
use std::fs;
use std::io::Error;


use regex::{Regex};

#[cfg(test)]
mod tests {
    use crate::days::day05::{parse_input, part_a, part_b};

    const EXAMPLE_FILE: &str = "./resources/day05/example.txt";
    const INPUT_FILE: &str = "./resources/day05/input.txt";

    #[test]
    fn example_part_a_1() {
        let (mut stacks, commands) = parse_input(EXAMPLE_FILE);
        assert_eq!("CMZ", part_a(&mut stacks, &commands));
    }

    #[test]
    fn example_part_b() {
        let (mut stacks, commands) = parse_input(EXAMPLE_FILE);
        assert_eq!("MCD", part_b(&mut stacks, &commands));
    }

    #[test]
    fn test_part_a() {
        let (mut stacks, commands) = parse_input(INPUT_FILE);
        assert_eq!("HBTMTBSDC", part_a(&mut stacks, &commands));
    }

    #[test]
    fn test_part_b() {
        let (mut stacks, commands) = parse_input(INPUT_FILE);
        assert_eq!("PQTJRSHWS", part_b(&mut stacks, &commands));
    }
}

#[derive(Debug)]
struct Command {
    how_many: u32,
    from: u32,
    to: u32,
}

fn parse_command(c: String) -> Command {
    let re = Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
    let captures = re.captures(&c).unwrap();
    Command {
        how_many: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        from: captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        to: captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
    }
}

fn parse_input(input_file: &str) -> (Vec<VecDeque<char>>, Vec<Command>) {
    let content = fs::read_to_string(input_file).unwrap();
    let lines = content.lines();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut commands_idx = 0;
    let mut stop = false;
    for (idx, line) in lines.clone().enumerate() {
        if stop {
            commands_idx = idx;
            break;
        }
        for (idx, space) in line.as_bytes().chunks(4).enumerate() {
            if stacks.get(idx).is_none() {
                stacks.push(VecDeque::new());
            }
            let value = parse_container(space);
            if value == "1" {
                stop = true;
                break;
            }
            if value != "" {
                stacks[idx].push_front(value.chars().nth(0).unwrap())
            }
        }
    }
    let commands = lines.clone()
        .skip(commands_idx + 1)
        .map(|x| x.to_string())
        .map(parse_command)
        .collect::<Vec<Command>>();

    return (stacks, commands);
}

pub fn run(input_file: &str) -> Result<i32, Error> {
    let (stacks, commands) = parse_input(input_file);
    let stacks_a = stacks.clone();
    let stacks_b = stacks.clone();
    let a = part_a(&mut stacks_a.clone(), &commands);
    visualize_result(&stacks_a);
    let b = part_b(&mut stacks_b.clone(), &commands);
    visualize_result(&stacks_b);
    println!("Part A: {}", a);
    println!("Part B: {}", b);
    Ok(0)
}

fn visualize_result(p0: &Vec<VecDeque<char>>) {
    let max_height = p0.iter().map(|x|x.len()).max().unwrap();
    for i in 0..max_height+1 {
        for v in p0 {
            if max_height - i >= v.len() {
                print!("    ")
            } else {
                print!("[{}] ", v[max_height - i])
            }
        }
        println!()
    }
}

fn parse_container(space: &[u8]) -> String {
    String::from_utf8(Vec::from(space)).
        unwrap().
        trim().
        replace("[", "").
        replace("]", "")
}

fn part_a(stacks: &mut Vec<VecDeque<char>>, commands: &Vec<Command>) -> String {
    // Execute commands
    for command in commands {
        for _ in 0..command.how_many {
            let element = stacks.get_mut((command.from - 1) as usize).unwrap().pop_back().unwrap();
            stacks.get_mut((command.to - 1) as usize).unwrap().push_back(element);
        }
    }

    stacks.iter().map(|x|x.back().unwrap().to_string()).collect::<Vec<String>>().join("")
}

fn part_b(stacks: &mut Vec<VecDeque<char>>, commands: &Vec<Command>) -> String {
    for command in commands {
        let mut elements = Vec::new();
        for _ in 0..command.how_many {
            elements.push(stacks.get_mut((command.from - 1) as usize).unwrap().pop_back().unwrap());
        }
        elements.reverse();
        let s = stacks.get_mut((command.to - 1) as usize).unwrap();
        elements.iter().for_each(|&e|s.push_back(e));
    }

    stacks.iter().map(|x|x.back().unwrap().to_string()).collect::<Vec<String>>().join("")
}