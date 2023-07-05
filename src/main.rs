use oh_hi::board::Board;
use oh_hi::solver::{SimpleSolver, Solver};

fn main() {
    let mut board = Board::new(10);
    println!("{:?}", board);

    SimpleSolver.solve(&mut board);
    println!("{:?}", board);
}
