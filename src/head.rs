use std::process::exit;
use std::path;
use std::fs;

pub struct HeadOp {
    files: Vec<path::PathBuf>,
    lines: Option<usize>,
    bytes: Option<usize>,
}

impl HeadOp {
    pub fn new(files: Vec<path::PathBuf>, lines: Option<usize>, bytes: Option<usize>) -> Self {
        Self {
            files,
            lines,
            bytes,
        }
    }

    pub fn head(&mut self) -> Option<String> {
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

            if let Some(_) = self.bytes {
                return Some(fs::read_to_string(file)
                    .unwrap()
                    .chars()
                    .take(self.bytes.unwrap())
                    .collect::<String>());
            }
        }

        None
    }
}


