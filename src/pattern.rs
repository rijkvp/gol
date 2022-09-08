use crate::error::Error;
use rand::Rng;
use std::{fs, path::Path};

pub struct Pattern {
    pub pattern: Vec<bool>,
    pub size: (usize, usize),
}

impl Pattern {
    pub fn from_plaintext_file<P: AsRef<Path>>(path: P) -> Result<Pattern, Error> {
        let input = fs::read_to_string(path)?;
        let cleaned: Vec<&str> = input.lines().filter(|l| !l.starts_with('!')).collect();
        let width = cleaned
            .iter()
            .map(|l| l.len())
            .max()
            .ok_or(Error::ParseError("invalid rows".to_string(), 0, 0))?;
        let mut pattern = Vec::new();
        let mut height = 0;
        for line in cleaned.iter() {
            for col in 0..width {
                if Some("O") == line.get(col..col + 1) {
                    pattern.push(true);
                } else {
                    pattern.push(false);
                }
            }
            height += 1;
        }
        Ok(Pattern {
            pattern,
            size: (width, height),
        })
    }

    pub fn from_random(size: (usize, usize)) -> Pattern {
        let mut rng = rand::thread_rng();
        let len = size.0 * size.1;
        let mut pattern = Vec::with_capacity(len);
        for _ in 0..len {
            pattern.push(rng.gen_bool(0.3));
        }
        Pattern { pattern, size }
    }

    pub fn print(&self) {
        println!("Pattern {}x{}", self.size.0, self.size.1);
        for y in 0..self.size.1 {
            let mut line = String::new();
            for x in 0..self.size.0 {
                if self.pattern[y * self.size.0 + x] {
                    line.push('O')
                } else {
                    line.push('.')
                }
            }
            println!("{}", line);
        }
    }
}
