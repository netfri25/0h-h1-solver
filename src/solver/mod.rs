use crate::board::Board;

// re-exports
pub mod brute_force;
pub use brute_force::*;

pub mod smarter;
pub use smarter::*;

pub trait Solver<'board> {
    // creates the solver with the board
    fn new_solver(board: &'board mut Board) -> Self;

    // true on success, false on failure
    fn solve(&mut self) -> bool;

    // doesn't tell if the board is solvable or not, only if it's current  state is a valid state
    fn is_valid(&mut self) -> bool;
}

