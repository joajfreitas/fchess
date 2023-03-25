use std::fmt;

use crate::square::Square;

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, Debug)]
pub enum PieceType {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11,
    Marker = 12,
    SourceMarker = 13,
    NoPiece,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pieces = [
            "♟︎", "♜", "♞", "♝", "♛", "♚", "♙", "♖", "♘", "♗", "♕", "♔", "*", "+", " ",
        ];
        f.write_str(pieces[*self as usize])
    }
}

impl PieceType {
    pub fn is_black(self: &PieceType) -> bool {
        matches!(
            self,
            PieceType::BlackPawn
                | PieceType::BlackRook
                | PieceType::BlackKnight
                | PieceType::BlackBishop
                | PieceType::BlackQueen
                | PieceType::BlackKing
        )
    }

    pub fn is_white(self: &PieceType) -> bool {
        matches!(
            self,
            PieceType::WhitePawn
                | PieceType::WhiteRook
                | PieceType::WhiteKnight
                | PieceType::WhiteBishop
                | PieceType::WhiteQueen
                | PieceType::WhiteKing
        )
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    square: Square,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(square: Square, piece_type: PieceType) -> Piece {
        Piece { square, piece_type }
    }

    pub fn get_square(&self) -> Square {
        self.square
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}
