use std::path::PathBuf;

use crate::{error::Error, grid::Grid, pattern::Pattern};

#[derive(Debug, Clone)]
pub enum PatternConfig {
    Random,
    File(PathBuf),
}

#[derive(Debug, Clone)]
pub struct LifeConfig {
    pub size: (usize, usize),
    pub pattern: PatternConfig,
}

#[derive(Clone)]
pub struct Life {
    size: (usize, usize),
    pub tick: u128,
    grid1: Grid,
    grid2: Grid,
    mode: bool,
}

impl Life {
    pub fn new(size: (usize, usize), pattern: &Pattern) -> Self {
        Self {
            tick: 0,
            size,
            grid1: Grid::from_pattern_centered(size, pattern),
            grid2: Grid::empty(size),
            mode: false,
        }
    }

    pub fn from_config(cfg: &LifeConfig) -> Result<Self, Error> {
        let pattern = match &cfg.pattern {
            PatternConfig::Random => Pattern::from_random(cfg.size),
            PatternConfig::File(path) => Pattern::from_plaintext_file(&path)?,
        };

        Ok(Self::new(cfg.size, &pattern))
    }

    // The main Conway's Game of Life algorithm
    pub fn update(&mut self) {
        let (current, next) = {
            if self.mode {
                (&self.grid2, &mut self.grid1)
            } else {
                (&self.grid1, &mut self.grid2)
            }
        };

        // Iterate over the cells
        let max = self.size.0 * self.size.1;
        for i in 0..max {
            // Count the number of neighbour living cells
            let mut nb_count = 0u8;
            let row_len = self.size.0 as i64;
            // Iterate over the fixed offets of the 8 neighbours in the array
            for offset in [
                -row_len - 1,
                -row_len,
                -row_len + 1,
                -1,
                1,
                row_len - 1,
                row_len,
                row_len + 1,
            ] {
                let pos = (i as i64 + offset) as usize;
                // Make sure to check if the position is valid
                if pos > 0 && pos < max && current[pos] {
                    nb_count += 1;
                }
            }

            // Die from under or overpopulation
            let living = current[i];
            if living && (nb_count < 2 || nb_count > 3) {
                next[i] = false;
            }
            // New cells if 3 neighbours
            else if !living && nb_count == 3 {
                next[i] = true
            }
            // Remain the same
            else {
                next[i] = living;
            }
        }
        self.tick += 1;
        self.mode = !self.mode;
    }

    pub fn get_state(&self) -> &Grid {
        if self.mode {
            &self.grid2
        } else {
            &self.grid1
        }
    }
}
