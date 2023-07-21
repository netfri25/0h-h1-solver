use std::fmt;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    #[default]
    Empty,
    Red,
    Blue,
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ansi_color = |text, color| write!(f, "{}{}\x1b[0m", color, text);

        match self {
            Self::Empty => ansi_color("_", "\x1b[37m"),
            Self::Red => ansi_color("R", "\x1b[31m"),
            Self::Blue => ansi_color("B", "\x1b[34m"),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    size: usize,
    cells: Vec<Cell>, // cells.len == size * size
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
        debug_assert!(size % 2 == 0, "size should be an even integer number");
        debug_assert!(size > 0, "size should be a natural number (integer bigger than 0)");
        let cells = vec![Cell::Empty; size * size];
        Self { size, cells }
    }

    pub fn builder(size: usize) -> BoardBuilder {
        BoardBuilder::new(size)
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
}

#[derive(Debug)]
pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new(size: usize) -> Self {
        let board = Board::new(size);
        Self { board }
    }

    // when insertion fails, it just ignores it
    pub fn insert(&mut self, row: usize, col: usize, cell: Cell) -> &mut Self {
        self.board.set(row, col, cell);
        self
    }

    // will be optimized by rust, so no need to worry about cloning
    pub fn build(&mut self) -> Board {
        self.board.clone()
    }
}
