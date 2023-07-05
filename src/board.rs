// TODO: BoardBuilder

use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Red,
    Blue,
}

#[derive(Clone)]
pub struct Board {
    size: usize,
    cells: Vec<Cell>,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.cells.chunks(self.size) {
            writeln!(f, "{:?}", row)?
        }

        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Self {
        assert!(size % 2 == 0, "size should be an even integer number");
        let cells = vec![Cell::Empty; size * size];
        Self { size, cells }
    }

    pub const fn size(&self) -> usize {
        self.size
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row * self.size + col)
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.cells.get_mut(row * self.size + col)
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        if let Some(mut_cell) = self.at_mut(row, col) {
            *mut_cell = cell
        }
    }

    pub fn unset(&mut self, row: usize, col: usize) {
        self.set(row, col, Cell::Empty)
    }

    pub fn is_valid(&self) -> bool {
        let mut last = Cell::Empty;
        let mut matching_lasts = 0;

        // column-wise checking for adjacent cells
        for row in 0..self.size() {
            for col in 0..self.size() {
                let cell = self.at(row, col).unwrap();

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
        for col in 0..self.size() {
            for row in 0..self.size() {
                let cell = self.at(row, col).unwrap();
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
        for row1 in 0..self.size() {
            'next_row: for row2 in row1 + 1..self.size() {
                for col in 0..self.size() {
                    let cell1 = self.at(row1, col).unwrap();
                    let cell2 = self.at(row2, col).unwrap();
                    if *cell1 == Cell::Empty || *cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_row;
                    }
                }
                return false;
            }
        }

        // similar columns checking
        for col1 in 0..self.size() {
            'next_col: for col2 in col1 + 1..self.size() {
                for row in 0..self.size() {
                    let cell1 = self.at(row, col1).unwrap();
                    let cell2 = self.at(row, col2).unwrap();
                    if *cell1 == Cell::Empty || *cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_col;
                    }
                }
                return false;
            }
        }

        // even amounts in each row
        'next_row: for row in 0..self.size() {
            let mut sum = 0;
            for col in 0..self.size() {
                let cell = self.at(row, col).unwrap();
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
        'next_col: for col in 0..self.size() {
            let mut sum = 0;
            for row in 0..self.size() {
                let cell = self.at(row, col).unwrap();
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
