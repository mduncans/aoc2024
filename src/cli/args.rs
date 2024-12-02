use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // Path to input data
    #[arg(short, long)]
    pub input: PathBuf,
}
