use std::fs;
use std::path;
use std::process::exit;

pub struct HeadOp {
    pub file: path::PathBuf,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_lines() {
        let mut head = HeadOp::new(path::PathBuf::from("test.txt"), Some(2), None);
        assert_eq!(
            head.head().unwrap(),
            "Never Gonna Give You Up - Rick Astley\n"
        );
    }

    #[test]
    fn test_head_bytes() {
        let mut head = HeadOp::new(path::PathBuf::from("test.txt"), None, Some(5));
        assert_eq!(head.head().unwrap(), "Never");
    }

    #[test]
    fn test_head_default() {
        let mut head = HeadOp::new(path::PathBuf::from("test.txt"), None, None);
        assert_eq!(
            head.head().unwrap(),
            "Never Gonna Give You Up - Rick Astley\n\nI'm no stranger to love\nYou know the rules and so do I\nA full commitment's what I'm thinking of\nYou wouldn't get this from any other guy\n\nI just wanna tell you how I'm feeling\nGotta make you understand\n"
        );
    }
}
