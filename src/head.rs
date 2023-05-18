use std::fs;
use std::path;
use std::process::exit;

pub struct HeadOp {
    file: path::PathBuf,
    lines: Option<usize>,
    bytes: Option<usize>,
}

impl HeadOp {
    pub fn new(file: path::PathBuf, lines: Option<usize>, bytes: Option<usize>) -> Self {
        Self { file, lines, bytes }
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

        if let Some(_) = self.lines {
            return Some(
                fs::read_to_string(&self.file)
                    .unwrap()
                    .lines()
                    .take(self.lines.unwrap())
                    .collect::<Vec<&str>>()
                    .join("\n"),
            );
        }

        if let Some(_) = self.bytes {
            return Some(
                fs::read_to_string(&self.file)
                    .unwrap()
                    .chars()
                    .take(self.bytes.unwrap())
                    .collect::<String>(),
            );
        }

        None
    }
}
