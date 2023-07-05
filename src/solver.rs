
use crate::board::{Board, Cell};

pub trait Solver {
    // true on success, false on failure
    fn solve(&mut self, board: &mut Board) -> bool;
}


pub struct SimpleSolver;

impl Solver for SimpleSolver {
    fn solve(&mut self, board: &mut Board) -> bool {
        Self::rec_solve(board, 0, 0)
    }
}

impl SimpleSolver {
    fn rec_solve(board: &mut Board, row: usize, col: usize) -> bool {
        if col >= board.size() {
            return Self::rec_solve(board, row+1, 0);
        }

        if row >= board.size() {
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
