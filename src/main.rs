mod colors;
mod head;

use crate::head::HeadOp;

use clap::Parser;

use std::fs::File;
use std::io::ErrorKind;
use std::path;
use std::process::exit;

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

    if filenames.is_empty() {
        println!("head-rs: No files to read");
        exit(1);
    }

    let mut prev_color = colors::pick_rand_color();

    for filename in filenames {
        let mut head_op = HeadOp::new(filename, args.lines, args.bytes);
        let head = head_op.head().unwrap();

        let mut color = colors::pick_rand_color();

        if prev_color == color {
            color = colors::pick_rand_color();
        }

        println!(
            "{}",
            colors::colorize(
                &format!("\n==> {} <==", head_op.file.to_str().unwrap()),
                color.0.clone()
            )
        );

        println!("{}", 
            colors::colorize(
                head.as_ref(),
                color.1.clone()
            )
        );

        prev_color = color;
    }
}
