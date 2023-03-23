use std::env;

use fchess::board::{Board, Side};
use fchess::book::Book;

fn main() {
    let args: Vec<String> = env::args().collect();

    let book = Book::from_filename(args.get(1).unwrap());

    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{:?}", book.get_best_move(&board, &Side::White).unwrap());
}
