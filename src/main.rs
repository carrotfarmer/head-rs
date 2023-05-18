mod head;

use crate::head::HeadOp;

use clap::Parser;

use std::fs::File;
use std::io::ErrorKind;
use std::path;

#[derive(Debug, Parser)]
struct Args {
    filenames: Vec<path::PathBuf>,

    #[clap(short = 'n', long)]
    lines: Option<usize>,

    #[clap(short = 'c', long)]
    bytes: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let mut filenames = args.filenames.clone();

    for filename in filenames.clone() {
        match File::open(&filename) {
            Ok(_) => continue,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    println!("head-rs: No such file: {}", &filename.to_str().unwrap());
                    filenames = filenames
                        .into_iter()
                        .filter(|f| f.to_str() != filename.to_str())
                        .collect();
                }
                _ => println!(
                    "head-rs: Error while opening file {}: {}",
                    &filename.to_str().unwrap(),
                    e
                ),
            },
        }
    }

    let mut head_op = HeadOp::new(filenames, args.lines, args.bytes);
    println!("{}", head_op.head().unwrap());
}
