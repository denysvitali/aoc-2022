use std::io::{Error};
use std::str::Lines;
use std::fs;
use std::fs::{File, read};
use std::io::{BufRead, BufReader};
use clap::builder::Str;
use tokio::io::AsyncBufReadExt;
use crate::days::day02::Shape::{Paper, Rock, Scissors};

#[cfg(test)]
mod tests {
    use std::io::Error;
    use crate::days::day02::run;

    #[test]
    fn example() {
        run("./resources/day02/example.txt").unwrap();
    }
}

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self, opponent: &Shape) -> bool{
        if &self.beats_target() == opponent {
            return true
        }

        return false
    }

    fn beats_target(&self) -> Shape {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_target(&self) -> Shape {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn value(&self) -> u8 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn recognize(input: &str) -> Shape {
    match input {
        "A" => Rock,
        "B" => Paper,
        "C" => Scissors,
        "X" => Rock,
        "Y" => Paper,
        "Z" => Scissors,
        _ => {
            panic!("invalid input")
        }
    }
}

fn score(round: Vec<Shape>) -> i32 {
    if round.len() != 2 {
        panic!("More than two players")
    }
    let opponent = &round[0];
    let player = &round[1];

    if opponent == player {
        // Draw
        return (player.value() + 3) as i32
    }

    if opponent.beats(player) {
        return (player.value()) as i32
    }

    return (player.value() + 6) as i32
}

fn score_2(round: Vec<Shape>) -> i32 {
    if round.len() != 2 {
        panic!("More than two players")
    }
    let opponent = &round[0];
    let player = &round[1];

    (match player {
        Rock => {
            // X = Loose
            opponent.beats_target().value() + 0
        }
        Paper => {
            // Y = Draw
            opponent.value() + 3
        }
        Scissors => {
            // Z = Win
            opponent.loses_target().value() + 6
        }
        _ => 0
    }) as i32
}

pub fn run(input_file: &str) -> Result<i32, Error>{
    let content = fs::read_to_string(input_file)?;
    let lines = content.lines();

    // Part A
    println!("Part A: {}", part_a(lines.clone()));
    println!("Part B: {}", part_b(lines.clone()));
    Ok(0)
}

fn part_a(lines: Lines) -> i32 {
    return lines.map(|x|{
        score(x.split(" ").map(recognize).collect())
    }).sum();
}

fn part_b(lines: Lines) -> i32 {
    return lines.map(|x|{
        score_2(x.split(" ").map(recognize).collect())
    }).sum();
}