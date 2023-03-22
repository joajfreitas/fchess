use rustyline::{Editor, Result};
use std::env;

use fchess::board::Board;
use fchess::book::Book;
use fchess::moves::{Move, Scope, Side};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".fchess_history").is_err() {
        println!("No previous history");
    }

    let book_filename = args.get(1).unwrap();

    let book = Book::from_filename(book_filename);

    let mut board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1");
    println!("{:?}", board);

    let mut side = Side::White;

    loop {
        let mov: Move = Move::from_algebraic(&match side {
            Side::White => {
                let line = rl.readline("> ");
                match line {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        rl.save_history(".fchess_history").unwrap();
                        line
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        return Err(err);
                    }
                }
            }
            Side::Black => match dbg!(book.get_best_move(&board, &Side::Black)) {
                Some(mov) => mov,
                _ => board.best_move(Scope::Black).unwrap(),
            }
            .to_algebraic(),
        })
        .unwrap();

        board = match board.apply(mov) {
            Some(board) => board,
            None => continue,
        };
        println!("{:?}", board);

        side = !side;
    }
}
