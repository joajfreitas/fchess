use std::fmt;
use crate::board::print_board;
use crate::piece::{Piece, ColoredPieceType};
use crate::square::Square;

struct PrintableBoard(u64);

impl fmt::Display for PrintableBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pieces: Vec<Piece> = Vec::new();
        for i in 0..64 {
            if self.0 >> i & 1 == 1 {
                let square = Square::from_index(i);
                pieces.push(Piece::new(square, ColoredPieceType::Marker));
            }
        }
        print_board(pieces, f)
    }
}

pub fn print_u64(board: u64) {
    println!("{}", PrintableBoard(board));
}
