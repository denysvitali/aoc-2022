use std::io::Error;
use std::fs;

pub fn run(input_file: &str) -> Result<i32, Error>{
    let content = fs::read_to_string(input_file)?;

    let mut lines = content.split("\n\n").map(|x|
        x.lines().map(|y| y.parse::<i32>().unwrap()).sum()
    ).collect::<Vec<i32>>();

    lines.sort();
    lines.reverse();

    // Part A
    println!("Part A: {}", lines[0]);
    println!("Part B: {}", lines[0..3].iter().sum::<i32>());
    Ok(0)
}