use crate::pattern::Pattern;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone)]
pub struct Grid {
    grid: Vec<bool>,
    size: (usize, usize),
}

impl Grid {
    pub fn empty(size: (usize, usize)) -> Self {
        let mut grid = Vec::with_capacity(size.0 * size.1);
        for _ in 0..size.0 * size.1 {
            grid.push(false);
        }
        Self { grid, size }
    }

    pub fn from_pattern(pattern: Pattern) -> Grid { 
        Grid { grid: pattern.pattern, size: pattern.size }
    }

    // Creates a grid with a pattern centered in the middle
    pub fn from_pattern_centered(size: (usize, usize), pattern: &Pattern) -> Grid {
        let mut grid: Grid = Self::empty(size);
        let start_x = size.0 / 2 - pattern.size.0 / 2;
        let start_y = size.1 / 2 - pattern.size.1 / 2;
        for y in 0..pattern.size.1  {
            for x in 0..pattern.size.0 {
                let val = pattern.pattern[y * pattern.size.0 + x];
                grid[(start_y + y) * size.0 + start_x + x] = val;
            }
        }
        grid
    }

    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = {
                if self[i] {
                    [0xff, 0xff, 0xff, 0xff]
                } else {
                    [0, 0, 0, 0]
                }
            };
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn print(&self) {
        println!("Grid {}x{}", self.size.0, self.size.1);
        for y in 0..self.size.1 {
            let mut line = String::new();
            for x in 0..self.size.0 {
                if self.grid[y * self.size.0 + x] {
                    line.push('O')
                } else {
                    line.push('.')
                }
            }
            println!("{}", line);
        }
    }
}

impl Index<usize> for Grid {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}
