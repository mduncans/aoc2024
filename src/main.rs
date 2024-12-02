use aoc2024::{cli::cli, days};
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    let command = args.command;

    let result = match command {
        cli::Commands::Day1 => days::day1::solve(args.input, &args.part).unwrap(),
    };

    println!("{}", result);
}
