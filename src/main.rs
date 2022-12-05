extern crate core;

mod fetch;
mod days;

use std::env;
use clap::Parser;
use hyper::Client;

#[derive(Parser, Debug)]
#[command(version,about)]
struct Args {
    #[command(subcommand)]
    action: Action,

    #[arg(short,long)]
    day: i32
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Fetch,
    Run
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    match args.action {
        Action::Fetch => {
            let aoc_cookie = env::var("AOC_COOKIE");
            match aoc_cookie {
                Ok(c) => fetch::fetch(args.day, c.as_str()).await,
                Err(_) => Err(Box::from("Unable to get AOC_COOKIE env variable"))
            }.unwrap();
        },
        Action::Run => {
            let input_file = format!("resources/day{:02}/input.txt", args.day);
            match args.day {
                1 => days::day01::run(&input_file),
                2 => days::day02::run(&input_file),
                _ => {
                    panic!("This day hasn't been implemented yet")
                }
            }.unwrap();
        }
    }
}