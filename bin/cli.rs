use rustyline::{Editor, Result};

use fchess::board::Board;
use fchess::moves::{Move, Scope, Side};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    if rl.load_history(".fchess_history").is_err() {
        println!("No previous history");
    }

    let mut board =
        Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string());
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
            Side::Black => board.best_move(Scope::Black).unwrap().to_algebraic(),
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
