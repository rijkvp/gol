use crate::error::Error;
use rand::Rng;
use std::fs;

pub struct Pattern {
    pub pattern: Vec<bool>,
    pub width: usize,
    pub height: usize,
}

impl Pattern {
    pub fn from_plaintext_file(path: &str) -> Result<Pattern, Error> {
        let input = fs::read_to_string(path)?;
        let cleaned: Vec<&str> = input.lines().filter(|l| !l.starts_with("!")).collect();
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
            width,
            height,
        })
    }

    pub fn from_random(width: usize, height: usize) -> Pattern {
        let mut rng = rand::thread_rng();
        let mut pattern = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            pattern.push(rng.gen_bool(0.3));
        }
        Pattern {
            pattern,
            width,
            height,
        }
    }

    pub fn print(&self) {
        println!("Pattern {}x{}", self.width, self.height);
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if self.pattern[y * self.width + x] {
                    line.push('O')
                } else {
                    line.push('.')
                }
            }
            println!("{}", line);
        }
    }
}
