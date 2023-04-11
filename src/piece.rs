use std::fmt;
use std::ops::Not;

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

    pub fn from_string(s: &char) -> Option<PieceType> {
        match s {
            'P' => Some(PieceType::WhitePawn),
            'R' => Some(PieceType::WhiteRook),
            'N' => Some(PieceType::WhiteKnight),
            'B' => Some(PieceType::WhiteBishop),
            'Q' => Some(PieceType::WhiteQueen),
            'K' => Some(PieceType::WhiteKing),
            'p' => Some(PieceType::BlackPawn),
            'r' => Some(PieceType::BlackRook),
            'n' => Some(PieceType::BlackKnight),
            'b' => Some(PieceType::BlackBishop),
            'q' => Some(PieceType::BlackQueen),
            'k' => Some(PieceType::BlackKing),
            _ => None,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            PieceType::WhitePawn => 'P',
            PieceType::WhiteRook => 'R',
            PieceType::WhiteKnight => 'N',
            PieceType::WhiteBishop => 'B',
            PieceType::WhiteQueen => 'Q',
            PieceType::WhiteKing => 'K',
            PieceType::BlackPawn => 'p',
            PieceType::BlackRook => 'r',
            PieceType::BlackKnight => 'n',
            PieceType::BlackBishop => 'b',
            PieceType::BlackQueen => 'q',
            PieceType::BlackKing => 'k',
            _ => panic!(),
        }
    }
}

impl Not for PieceType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PieceType::WhitePawn => PieceType::BlackPawn,
            PieceType::WhiteRook => PieceType::BlackRook,
            PieceType::WhiteKnight => PieceType::BlackKnight,
            PieceType::WhiteBishop => PieceType::BlackBishop,
            PieceType::WhiteQueen => PieceType::BlackQueen,
            PieceType::WhiteKing => PieceType::BlackKing,
            PieceType::BlackPawn => PieceType::WhitePawn,
            PieceType::BlackRook => PieceType::WhiteRook,
            PieceType::BlackKnight => PieceType::WhiteKnight,
            PieceType::BlackBishop => PieceType::WhiteBishop,
            PieceType::BlackQueen => PieceType::WhiteQueen,
            PieceType::BlackKing => PieceType::WhiteKing,
            _ => self,
        }
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
