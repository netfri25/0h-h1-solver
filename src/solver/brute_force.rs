use crate::board::{Board, Cell};
use super::Solver;

pub struct BruteForce<'board> {
    // for the `smarter` module
    pub(super) board: &'board mut Board,
}

impl<'board> Solver<'board> for BruteForce<'board> {
    fn new_solver(board: &'board mut Board) -> Self {
        Self { board }
    }

    fn solve(&mut self) -> bool {
        if !self.is_valid() {
            return false
        }

        self.rec_solve(0, 0)
    }

    fn is_valid(&mut self) -> bool {
        let mut last = Cell::Empty;
        let mut matching_lasts = 0;

        // column-wise checking for adjacent cells
        for row in 0..self.board.size() {
            for col in 0..self.board.size() {
                let cell = self.board.at(row, col).unwrap();

                if *cell == Cell::Empty {
                    continue;
                }

                if *cell == last {
                    matching_lasts += 1;
                } else {
                    matching_lasts = 1;
                    last = *cell;
                }

                // three adjacent cells with the same color isn't valod
                if matching_lasts >= 3 {
                    return false;
                }
            }

            // clear for the next iteration
            last = Cell::Empty;
            matching_lasts = 0;
        }

        // row-wise checking for adjacent cells
        for col in 0..self.board.size() {
            for row in 0..self.board.size() {
                let cell = self.board.at(row, col).unwrap();
                if *cell == Cell::Empty {
                    continue;
                }

                if *cell == last {
                    matching_lasts += 1;
                } else {
                    matching_lasts = 0;
                    last = *cell;
                }

                if matching_lasts >= 3 {
                    return false;
                }
            }

            // clear for the next iteration
            last = Cell::Empty;
            matching_lasts = 0;
        }

        // similar rows checking
        for row1 in 0..self.board.size() {
            'next_row: for row2 in row1 + 1..self.board.size() {
                for col in 0..self.board.size() {
                    let cell1 = self.board.at(row1, col).unwrap();
                    let cell2 = self.board.at(row2, col).unwrap();
                    if *cell1 == Cell::Empty || *cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_row;
                    }
                }
                return false;
            }
        }

        // similar columns checking
        for col1 in 0..self.board.size() {
            'next_col: for col2 in col1 + 1..self.board.size() {
                for row in 0..self.board.size() {
                    let cell1 = self.board.at(row, col1).unwrap();
                    let cell2 = self.board.at(row, col2).unwrap();
                    if *cell1 == Cell::Empty || *cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_col;
                    }
                }
                return false;
            }
        }

        // even amounts in each row
        'next_row: for row in 0..self.board.size() {
            let mut sum = 0;
            for col in 0..self.board.size() {
                let cell = self.board.at(row, col).unwrap();
                sum += match cell {
                    Cell::Empty => continue 'next_row,
                    Cell::Red => -1,
                    Cell::Blue => 1,
                };
            }

            if sum != 0 {
                return false;
            }
        }

        // even amounts in each row
        'next_col: for col in 0..self.board.size() {
            let mut sum = 0;
            for row in 0..self.board.size() {
                let cell = self.board.at(row, col).unwrap();
                sum += match cell {
                    Cell::Empty => continue 'next_col,
                    Cell::Red => -1,
                    Cell::Blue => 1,
                };
            }
            if sum != 0 {
                return false;
            }
        }

        true
    }
}

impl<'board> BruteForce<'board> {
    fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.board.set(row, col, cell)
    }

    fn unset(&mut self, row: usize, col: usize) {
        self.set(row, col, Cell::Empty)
    }

    fn rec_solve(&mut self, row: usize, col: usize) -> bool {
        if col >= self.board.size() {
            return self.rec_solve(row+1, 0);
        }

        if row >= self.board.size() {
            return true;
        }

        let cell = self.board.at(row, col).unwrap();
        if *cell != Cell::Empty {
            return self.rec_solve(row, col+1);
        }

        if !self.is_valid() {
            return false;
        }

        self.set(row, col, Cell::Red);
        let is_solved = self.rec_solve(row, col+1);
        if is_solved {
            return true;
        }

        self.set(row, col, Cell::Blue);
        let is_solved = self.rec_solve(row, col+1);
        if is_solved {
            return true;
        }

        self.unset(row, col);
        false
    }
}
