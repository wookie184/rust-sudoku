use std::iter::zip;

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::solver::SudokuSolver;

pub struct SudokuGenerator {
    solver: SudokuSolver,
    rng: StdRng,
}

impl Default for SudokuGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SudokuGenerator {
    pub fn new() -> Self {
        let solver = SudokuSolver::new();
        Self {
            solver,
            rng: StdRng::from_entropy(),
        }
    }

    fn try_remove(&mut self, sudoku: &mut [usize], to_remove: &Vec<usize>) -> bool {
        let saved_values: Vec<usize> = to_remove.iter().map(|&idx| sudoku[idx]).collect();

        for &idx in to_remove {
            sudoku[idx] = 0;
        }

        if self.solver.is_valid_puzzle(sudoku) {
            true
        } else {
            for (&idx, val) in zip(to_remove, saved_values) {
                sudoku[idx] = val;
            }
            false
        }
    }

    pub fn generate_sudoku(&mut self) -> Vec<usize> {
        let mut question = self.solver.solve_random(&vec![0; 81]).unwrap();

        let mut to_remove: Vec<usize> = (0..81).collect();
        to_remove.shuffle(&mut self.rng);

        let mut chunk_size = 15;

        while !to_remove.is_empty() {
            // Take `chunk` cells from the end to remove them.
            let chunk: Vec<usize> = to_remove.iter().rev().take(chunk_size).copied().collect();

            // TODO: improve/remove these funky heuristics
            if self.try_remove(&mut question, &chunk) || chunk_size == 1 {
                to_remove.truncate(to_remove.len() - chunk_size);

                if chunk_size == 1 {
                    if to_remove.len() >= 60 {
                        chunk_size = 10;
                    } else if to_remove.len() >= 50 {
                        chunk_size = 5;
                    }
                }
            } else {
                chunk_size /= 4;
                if chunk_size <= 2 {
                    chunk_size = 1;
                }
            }
        }
        question
    }

    pub fn generate_sudoku_with_empty(&mut self, empty_cells: usize) -> Vec<usize> {
        loop {
            let sudoku = self.generate_sudoku();

            let empty_count = sudoku.iter().filter(|&&c| c == 0).count();
            if empty_count == empty_cells {
                return sudoku;
            }
        }
    }
}
