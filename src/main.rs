use clap::Parser;

use std::path;

#[derive(Debug, Parser)]
struct Args {
    filenames: Vec<path::PathBuf>,

    #[clap(short = 'n', long)]
    lines: Option<u32>,

    #[clap(short = 'c', long)]
    bytes: Option<u32>,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
