// TODO: BoardBuilder

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Red,
    Blue,
}

#[derive(Debug, Clone)]
pub struct Board<const SIZE: usize>([[Cell; SIZE]; SIZE]);

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
}
