
use crate::board::{Board, Cell};

pub trait Solver<const SIZE: usize> {
    // true on success, false on failure
    fn solve(&mut self, board: &mut Board<SIZE>) -> bool;
}


pub struct SimpleSolver;

impl<const SIZE: usize> Solver<SIZE> for SimpleSolver {
    fn solve(&mut self, board: &mut Board<SIZE>) -> bool {
        Self::rec_solve(board, 0, 0)
    }
}

impl SimpleSolver {
    fn rec_solve<const SIZE: usize>(board: &mut Board<SIZE>, row: usize, col: usize) -> bool {
        if col >= SIZE {
            return Self::rec_solve(board, row+1, 0);
        }

        if row >= SIZE {
            return true;
        }

        let cell = board.at(row, col).unwrap();
        if *cell != Cell::Empty {
            return Self::rec_solve(board, row, col+1)
        }

        if !board.is_valid() {
            return false;
        }

        board.set(row, col, Cell::Red);
        if Self::rec_solve(board, row, col+1) {
            return true;
        }

        board.set(row, col, Cell::Blue);
        if Self::rec_solve(board, row, col+1) {
            return true;
        }

        board.unset(row, col);
        false
    }
}
