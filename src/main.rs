use oh_hi::board::Board;
use oh_hi::solver::{self, Solver};

use std::time::Instant;

fn main() {
    let mut board = Board::new(10);
    println!("{:?}", board);

    let mut solver = solver::Smarter::new_solver(&mut board);
    let start = Instant::now();
    let solved = solver.solve();
    let elapsed = start.elapsed();

    if solved {
        println!("Board solved successfully!")
    } else {
        println!("Unable to solve the board.")
    }

    println!("{:?}", board);
    println!("{:?}", elapsed);
}
