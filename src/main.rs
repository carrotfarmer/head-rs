use clap::Parser;

use std::fs::{self, File};
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

struct HeadOp {
    files: Vec<path::PathBuf>,
    lines: Option<usize>,
    bytes: Option<usize>,
}

impl HeadOp {
    fn new(files: Vec<path::PathBuf>, lines: Option<usize>, bytes: Option<usize>) -> Self {
        Self {
            files,
            lines,
            bytes,
        }
    }

    fn head(&mut self) -> Option<String> {
        match (self.lines, self.bytes) {
            (Some(_), Some(_)) => {
                println!("head-rs: can\'t combine line and byte counts");
                exit(1)
            }
            (None, None) => self.lines = Some(10),
            _ => (),
        }

        if self.files.len() == 0 {
            println!("head-rs: No files specified");
            exit(1);
        }

        for file in &self.files {
            if let Some(_) = self.lines {
                return Some(fs::read_to_string(file)
                    .unwrap()
                    .lines()
                    .take(self.lines.unwrap())
                    .collect::<Vec<&str>>()
                    .join("\n"));
            }
        }

        None
    }
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
