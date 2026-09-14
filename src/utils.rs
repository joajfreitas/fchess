use crate::board::print_board;
use crate::piece::{ColoredPieceType, Piece};
use crate::square::Square;
use std::fmt;

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct BitBoard(pub u64);

impl std::ops::BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl From<u64> for BitBoard {
    fn from(item: u64) -> Self {
        BitBoard(item)
    }
}

fn print_u64(board: BitBoard, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut pieces: Vec<Piece> = Vec::new();
    for i in 0..64 {
        if board.0 >> i & 1 == 1 {
            let square = Square::from_index(i);
            pieces.push(Piece::new(square, ColoredPieceType::Marker));
        }
    }
    print_board(pieces, f)
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_u64(*self, f)
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //print_u64(*self, f)
        write!(f, "{:?}", self.0)
    }
}

//#[allow(dead_code)]
//pub fn print_u64(board: u64) {
//    println!("{}", BitBoard(board));
//}
