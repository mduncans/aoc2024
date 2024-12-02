use aoc2024::{cli::args, days};
use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let result = days::day1::solve(args.input).unwrap();

    println!("{}", result);
}
