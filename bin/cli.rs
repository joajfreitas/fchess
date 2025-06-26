use clap::Parser;

use anyhow::{anyhow, Result};
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

    #[arg(short, long)]
    side: Option<String>,
}

fn read_line(rl: &mut rustyline::DefaultEditor) -> Result<String> {
    let line = rl.readline("> ");
    match line {
        Ok(line) => {
            rl.add_history_entry(line.as_str())?;
            rl.save_history(".fchess_history").unwrap();
            Ok(line)
        }
        Err(err) => {
            println!("Error: {err:?}");
            Err(anyhow!(err))
        }
    }
}

fn get_best_move(board: &Board, book: &Option<Book>, solver: &mut Solver) -> Result<String> {
    Ok(match book.as_ref().and_then(|b| b.get_best_move(board)) {
        Some(mov) => {
            println!("=> Book move");
            mov
        }
        _ => {
            println!("=> Search move");
            solver.best_move(board).unwrap()
        }
    }
    .to_algebraic())
}

fn main() -> Result<()> {
    // Setup shell history
    let mut rl = rustyline::DefaultEditor::new()?;
    if rl.load_history(".fchess_history").is_err() {
        println!("No previous history");
    }

    let args = Args::parse();

    let side = match args
        .side
        .unwrap_or("white".to_string())
        .to_lowercase()
        .as_str()
    {
        "white" => Side::White,
        "black" => Side::Black,
        side => panic!("Error: unexpected side argument: {}", side),
    };

    let book = args.book.map(|book| Book::from_filename(&book));

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0")?;
    let mut solver = Solver::new();
    println!("{board}");

    loop {
        let algebra = &if board.get_turn() == side {
            read_line(&mut rl)
        } else {
            get_best_move(&board, &book, &mut solver)
        }?;
        let mov: Move = Move::from_san(algebra, &board).unwrap();

        board = match board.apply(&mov) {
            Some(board) => board,
            None => continue,
        };
        println!("{board}");
    }
}
