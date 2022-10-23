use std::io::prelude::*;
use std::io;

use fchess::moves::{Board, Scope, algebraic, Side, Move};

fn main() -> io::Result<()> {
    let mut board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string());
    println!("{:?}", board);
    
    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut side = Side::White;

    
    loop {
        let mov: Move = Move::from_algebraic_notation(&match side {
            Side::White => {
                let mut user_input = String::new();
                stdin.read_line(&mut user_input);
                println!("{:?}", user_input.chars());
                user_input[.. user_input.len()-1].to_string()
            },
            Side::Black => {
                algebraic(board.best_move(Scope::Black).unwrap())

            }
        }).unwrap();
        
        println!("{:?}", mov);
        board = match board.apply(mov) {
            Some(board) => board,
            None => continue,
        };
        println!("{:?}", board);

        side = match side {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };

    }
    Ok(())
}
