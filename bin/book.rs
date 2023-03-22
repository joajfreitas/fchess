use std::env;

use fchess::board::Board;
use fchess::book::Book;
use fchess::moves::Side;

fn main() {
    let args: Vec<String> = env::args().collect();

    let book = Book::from_filename(args.get(1).unwrap());

    let board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{:?}", book.get_best_move(&board, &Side::White).unwrap());
}
