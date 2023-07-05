// TODO: BoardBuilder

use std::fmt;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty = -1,
    Red = 0,
    Blue = 1,
}

#[derive(Clone)]
pub struct Board<const SIZE: usize>([[Cell; SIZE]; SIZE]);

impl<const SIZE: usize> fmt::Debug for Board<SIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.0.iter() {
            writeln!(f, "{:?}", row)?
        }

        Ok(())
    }
}

impl<const SIZE: usize> Default for Board<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> Board<SIZE> {
    pub fn new() -> Self {
        Self([[Cell::Empty; SIZE]; SIZE])
    }

    pub fn at(&self, row: usize, col: usize) -> Option<&Cell> {
        self.0.get(row).and_then(|row| row.get(col))
    }

    pub fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut Cell> {
        self.0.get_mut(row).and_then(|row| row.get_mut(col))
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
        for row in self.0.iter() {
            for cell in row.iter() {
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
        for col in 0..SIZE {
            for row in 0..SIZE {
                let cell = self.0[row][col];
                if cell == Cell::Empty {
                    continue;
                }

                if cell == last {
                    matching_lasts += 1;
                } else {
                    matching_lasts = 0;
                    last = cell;
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
        for row1 in 0..SIZE {
            'next_row: for row2 in row1+1..SIZE {
                for col in 0..SIZE {
                    let cell1 = self.0[row1][col];
                    let cell2 = self.0[row2][col];
                    if cell1 == Cell::Empty || cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_row
                    }
                }
                return false
            }
        }

        // similar columns checking
        for col1 in 0..SIZE {
            'next_col: for col2 in col1+1..SIZE {
                for row in 0..SIZE {
                    let cell1 = self.0[row][col1];
                    let cell2 = self.0[row][col2];
                    if cell1 == Cell::Empty || cell2 == Cell::Empty || cell1 != cell2 {
                        continue 'next_col
                    }
                }
                return false
            }
        }


        // even amounts in each row
        'next_row: for row in 0..SIZE {
            let mut sum = 0;
            for col in 0..SIZE {
                let cell = self.0[row][col];
                sum += match cell {
                    Cell::Empty => continue 'next_row,
                    Cell::Red => -1,
                    Cell::Blue => 1,
                };
            }
            if sum != 0 { return false }
        }

        // even amounts in each row
        'next_col: for col in 0..SIZE {
            let mut sum = 0;
            for row in 0..SIZE {
                let cell = self.0[row][col];
                sum += match cell {
                    Cell::Empty => continue 'next_col,
                    Cell::Red => -1,
                    Cell::Blue => 1,
                };
            }
            if sum != 0 { return false }
        }

        true
    }
}
