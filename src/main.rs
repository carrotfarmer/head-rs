use clap::Parser;

use std::fs::File;
use std::io::ErrorKind;
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
    let filenames = args.filenames.clone();

    for filename in filenames {
        match File::open(&filename) {
            Ok(_) => println!("file exists!"),
            Err(e) => match e.kind() {
                ErrorKind::NotFound => println!("No such file: {}", &filename.to_str().unwrap()),
                _ => println!(
                    "Error while opening file {}: {}",
                    &filename.to_str().unwrap(),
                    e
                ),
            },
        }
    }
}
