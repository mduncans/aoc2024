use aoc2024::{cli::args, days};
use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let command = args.command;

    let result = match command {
        args::Commands::Day1 => days::day1::solve(args.input, &args.part).unwrap(),
        args::Commands::Day2 => days::day2::solve(args.input, &args.part).unwrap(),
        args::Commands::Day3 => days::day3::solve(args.input, &args.part).unwrap(),
        args::Commands::Day4 => days::day4::solve(args.input, &args.part).unwrap(),
    };

    println!("{}", result);
}
