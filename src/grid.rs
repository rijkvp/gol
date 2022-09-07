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
    pub fn from_pattern(size: usize, pattern: &[bool]) -> Grid {
        let psize = (pattern.len() as f64).sqrt() as usize;
        assert_eq!(pattern.len() / psize, psize);
        let mut grid: Grid = Self::empty(size);
        let start_x = size / 2 - psize / 2;
        let start_y = size / 2 - psize / 2;
        for x in 0..psize {
            for y in 0..psize {
                let val = pattern[y * psize + x];
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
