use rustyline::{Editor, Result};
use std::env;

use fchess::board::Board;
use fchess::book::Book;
use fchess::moves::Move;
use fchess::side::Side;
use fchess::solver::Solver;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".fchess_history").is_err() {
        println!("No previous history");
    }

    let book_filename = args.get(1).unwrap();

    let book = Book::from_filename(book_filename);
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0");
    let solver = Solver::new();
    println!("{}", board);

    loop {
        let mov: Move = Move::from_algebraic(&match board.get_turn() {
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
            Side::Black => dbg!(match book.get_best_move(&board) {
                Some(mov) => {
                    println!("Book move");
                    mov
                }
                _ => {
                    println!("Search move");
                    solver.best_move(&board).unwrap()
                }
            }
            .to_algebraic()),
        })
        .unwrap();

        board = match board.apply(mov) {
            Some(board) => board,
            None => continue,
        };
        println!("{}", board);
    }
}
