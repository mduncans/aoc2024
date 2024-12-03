use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Args {
    // Path to input data
    #[arg(short, long, help = "Specify the path to the input data file.")]
    pub input: PathBuf,

    // Problem part - a or b
    #[arg(
        short,
        long,
        default_value = "a",
        help = "Specify the problem part ('a' or 'b')."
    )]
    pub part: String,

    // What day to solve
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Day1,
    Day2,
}
