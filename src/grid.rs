use crate::pattern::Pattern;
use std::ops::{Index, IndexMut};

#[derive(Default, Clone)]
pub struct Grid {
    inner: Vec<bool>,
}

impl Grid {
    pub fn empty(size: usize) -> Self {
        let mut grid = Vec::with_capacity(size * size);
        for _ in 0..size * size {
            grid.push(false);
        }
        Self { inner: grid }
    }

    // Creates a grid with a pattern centered in the middle
    pub fn from_pattern(size: usize, pattern: &Pattern) -> Grid {
        let mut grid: Grid = Self::empty(size);
        let start_y = size / 2 - pattern.height / 2;
        let start_x = size / 2 - pattern.width / 2;
        for y in 0..pattern.height {
            for x in 0..pattern.width {
                let val = pattern.pattern[y * pattern.height + x];
                grid[(start_y + y) * size + start_x + x] = val;
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
}

impl Index<usize> for Grid {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
