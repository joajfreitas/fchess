use clap::Parser;

use anyhow::Result;
use fchess::{Board, Solver};

/// fchess cli interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    fen: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut solver = Solver::new();

    solver
        .best_move(&Board::from_fen(&args.fen).unwrap())
        .unwrap();
    Ok(())
}
