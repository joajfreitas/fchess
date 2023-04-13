use clap::Parser;
use rustyline::{Editor, Result};

use fchess::Board;
use fchess::Book;
use fchess::Move;
use fchess::Side;
use fchess::Solver;

/// fchess cli interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to polyglot openning book
    #[arg(short, long)]
    book: Option<String>,
}

fn main() -> Result<()> {
    // Setup shell history
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".fchess_history").is_err() {
        println!("No previous history");
    }

    let args = Args::parse();

    let book = args.book.map(|book| Book::from_filename(&book));

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
            Side::Black => match book.as_ref().and_then(|b| b.get_best_move(&board)) {
                Some(mov) => {
                    println!("=> Book move");
                    mov
                }
                _ => {
                    println!("=> Search move");
                    solver.best_move(&board).unwrap()
                }
            }
            .to_algebraic(),
        })
        .unwrap();

        board = match board.apply(mov) {
            Some(board) => board,
            None => continue,
        };
        println!("{}", board);
    }
}
