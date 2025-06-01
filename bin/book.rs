use std::env;

use anyhow::Result;
use fchess::{Board, Book};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let book = Book::from_filename(args.get(1).unwrap());

    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    println!("{:?}", book.get_best_move(&board).unwrap());
    Ok(())
}
