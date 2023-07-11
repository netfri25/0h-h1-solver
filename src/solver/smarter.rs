use itertools::Itertools;

use super::Solver;
use crate::board::{Board, Cell};

// Things needed for checking if the cell placement is valid:
//  - similar rows/cols
//  - adjacency in a search depth of 2 in each direction
//  - counts of reds/blues in a row/col

type BitsType = u32;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BitsRepr {
    pub reds: BitsType,
    pub blues: BitsType,
}

pub struct Smarter<'board> {
    board: &'board mut Board,
    pub rows: Vec<BitsRepr>,
    pub cols: Vec<BitsRepr>,
}

impl<'board> Solver<'board> for Smarter<'board> {
    fn new_solver(board: &'board mut Board) -> Self {
        let size = board.size();

        let mut rows = vec![BitsRepr::default(); size];
        let mut cols = vec![BitsRepr::default(); size];

        for (row, row_bits) in rows.iter_mut().enumerate() {
            for (col, col_bits) in cols.iter_mut().enumerate() {
                let cell = board.at(row, col).unwrap();
                match cell {
                    Cell::Empty => {}

                    Cell::Red => {
                        row_bits.reds |= 1 << col;
                        col_bits.reds |= 1 << row;
                    }

                    Cell::Blue => {
                        row_bits.blues |= 1 << col;
                        col_bits.blues |= 1 << row;
                    }
                }
            }
        }

        Self { board, rows, cols }
    }

    fn solve(&mut self) -> bool {
        self.rec_solve(0, 0)
    }

    fn is_valid(&mut self) -> bool {
        let size = self.board.size() as u32;
        let half_size = size / 2;

        // check for similar rows and rows counts
        for (start, row1) in self.rows.iter().enumerate() {
            let reds_count = row1.reds.count_ones();
            let blues_count = row1.blues.count_ones();

            if reds_count > half_size || blues_count > half_size {
                return false;
            }

            // if there are still some empty cells in the current row
            if reds_count + blues_count < size {
                continue
            }

            for row2 in self.rows.iter().skip(start + 1) {
                if row1 == row2 {
                    return false;
                }
            }
        }

        // check for similar cols and cols counts
        for (start, col1) in self.cols.iter().enumerate() {
            let reds_count = col1.reds.count_ones();
            let blues_count = col1.blues.count_ones();

            if reds_count > half_size || blues_count > half_size {
                return false;
            }

            // if there are still some empty cells in the current row
            if reds_count + blues_count < size {
                continue
            }

            for col2 in self.cols.iter().skip(start + 1) {
                if col1 == col2 {
                    return false;
                }
            }
        }

        // adjacent can be moved to the `set` function for better performance
        true
    }
}

impl<'board> Smarter<'board> {
    fn set(&mut self, row: usize, col: usize, cell: Cell) -> bool {
        self.rows[row].reds |= ((cell == Cell::Red) as BitsType) << col;
        self.rows[row].blues |= ((cell == Cell::Blue) as BitsType) << col;
        self.cols[col].reds |= ((cell == Cell::Red) as BitsType) << row;
        self.cols[col].blues |= ((cell == Cell::Blue) as BitsType) << row;
        self.board.set(row, col, cell);

        // check for triplets
        // TODO: manual checking for the triplets, because using an iterator might be nicer to
        //       write but it's actually slow af (it needs to skip a lot of bits at the start)
        let triplet = |bits, lower| bits_iter(bits).skip(lower).take(5).tuple_windows().any(|(a, b, c)| a && b && c);

        let row_bits = &self.rows[row];
        let lower_row = col.saturating_sub(2);
        let row_triplet = triplet(row_bits.reds, lower_row) || triplet(row_bits.blues, lower_row);

        let col_bits = &self.cols[col];
        let lower_col = row.saturating_sub(2);
        let col_triplet = triplet(col_bits.reds, lower_col) || triplet(col_bits.blues, lower_col);

        !row_triplet && !col_triplet
    }

    fn unset(&mut self, row: usize, col: usize) {
        self.rows[row].reds  &= !(1 << col);
        self.rows[row].blues &= !(1 << col);
        self.cols[col].reds  &= !(1 << row);
        self.cols[col].blues &= !(1 << row);
        self.board.unset(row, col);
    }

    fn rec_solve(&mut self, row: usize, col: usize) -> bool {
        let size = self.board.size();

        if row >= size {
            // reached the end
            return true;
        }

        if col >= size {
            // reached the end of the row, go to the next row
            return self.rec_solve(row + 1, 0);
        }

        // can unwrap because the previous 2 if's assures that row and col are in bounds
        let cell = *self.board.at(row, col).unwrap();
        if cell != Cell::Empty {
            // wasn't assign by me, so skip it
            return self.rec_solve(row, col + 1);
        }

        for cell in [Cell::Red, Cell::Blue] {
            self.unset(row, col);
            if !self.set(row, col, cell) || !self.is_valid() { continue; }
            let solved = self.rec_solve(row, col + 1);
            if solved { return true; }
        }

        self.unset(row, col);
        false
    }
}

pub struct BitsIter {
    iteration: usize,
    value: BitsType,
}

impl Iterator for BitsIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= std::mem::size_of::<BitsType>() * 8 {
            None
        } else {
            let result = self.value & (1 << self.iteration);
            self.iteration += 1;
            Some(result > 0)
        }
    }
}

pub fn bits_iter(value: BitsType) -> impl Iterator<Item = bool> {
    BitsIter { value, iteration: 0 }
}
