use std::fmt;
use std::ops::Not;

use crate::side::Side;
use crate::square::Square;

#[allow(dead_code)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn with_color(self, side: Side) -> ColoredPieceType {
        match (self, side) {
            (PieceType::Pawn, Side::White) => ColoredPieceType::WhitePawn,
            (PieceType::Rook, Side::White) => ColoredPieceType::WhiteRook,
            (PieceType::Knight, Side::White) => ColoredPieceType::WhiteKnight,
            (PieceType::Bishop, Side::White) => ColoredPieceType::WhiteBishop,
            (PieceType::Queen, Side::White) => ColoredPieceType::WhiteQueen,
            (PieceType::King, Side::White) => ColoredPieceType::WhiteKing,
            (PieceType::Pawn, Side::Black) => ColoredPieceType::BlackPawn,
            (PieceType::Rook, Side::Black) => ColoredPieceType::BlackRook,
            (PieceType::Knight, Side::Black) => ColoredPieceType::BlackKnight,
            (PieceType::Bishop, Side::Black) => ColoredPieceType::BlackBishop,
            (PieceType::Queen, Side::Black) => ColoredPieceType::BlackQueen,
            (PieceType::King, Side::Black) => ColoredPieceType::BlackKing,
        }
    }
}

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
pub enum ColoredPieceType {
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

impl fmt::Display for ColoredPieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pieces = [
            "♟︎", "♜", "♞", "♝", "♛", "♚", "♙", "♖", "♘", "♗", "♕", "♔", "*", "+", " ",
        ];
        f.write_str(pieces[*self as usize])
    }
}

impl ColoredPieceType {
    pub fn is_black(self: &ColoredPieceType) -> bool {
        matches!(
            self,
            ColoredPieceType::BlackPawn
                | ColoredPieceType::BlackRook
                | ColoredPieceType::BlackKnight
                | ColoredPieceType::BlackBishop
                | ColoredPieceType::BlackQueen
                | ColoredPieceType::BlackKing
        )
    }

    pub fn is_white(self: &ColoredPieceType) -> bool {
        matches!(
            self,
            ColoredPieceType::WhitePawn
                | ColoredPieceType::WhiteRook
                | ColoredPieceType::WhiteKnight
                | ColoredPieceType::WhiteBishop
                | ColoredPieceType::WhiteQueen
                | ColoredPieceType::WhiteKing
        )
    }

    pub fn from_string(s: &char) -> Option<ColoredPieceType> {
        match s {
            'P' => Some(ColoredPieceType::WhitePawn),
            'R' => Some(ColoredPieceType::WhiteRook),
            'N' => Some(ColoredPieceType::WhiteKnight),
            'B' => Some(ColoredPieceType::WhiteBishop),
            'Q' => Some(ColoredPieceType::WhiteQueen),
            'K' => Some(ColoredPieceType::WhiteKing),
            'p' => Some(ColoredPieceType::BlackPawn),
            'r' => Some(ColoredPieceType::BlackRook),
            'n' => Some(ColoredPieceType::BlackKnight),
            'b' => Some(ColoredPieceType::BlackBishop),
            'q' => Some(ColoredPieceType::BlackQueen),
            'k' => Some(ColoredPieceType::BlackKing),
            _ => None,
        }
    }
    pub fn to_char(self) -> char {
        match self {
            ColoredPieceType::WhitePawn => 'P',
            ColoredPieceType::WhiteRook => 'R',
            ColoredPieceType::WhiteKnight => 'N',
            ColoredPieceType::WhiteBishop => 'B',
            ColoredPieceType::WhiteQueen => 'Q',
            ColoredPieceType::WhiteKing => 'K',
            ColoredPieceType::BlackPawn => 'p',
            ColoredPieceType::BlackRook => 'r',
            ColoredPieceType::BlackKnight => 'n',
            ColoredPieceType::BlackBishop => 'b',
            ColoredPieceType::BlackQueen => 'q',
            ColoredPieceType::BlackKing => 'k',
            _ => panic!(),
        }
    }
}

impl Not for ColoredPieceType {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            ColoredPieceType::WhitePawn => ColoredPieceType::BlackPawn,
            ColoredPieceType::WhiteRook => ColoredPieceType::BlackRook,
            ColoredPieceType::WhiteKnight => ColoredPieceType::BlackKnight,
            ColoredPieceType::WhiteBishop => ColoredPieceType::BlackBishop,
            ColoredPieceType::WhiteQueen => ColoredPieceType::BlackQueen,
            ColoredPieceType::WhiteKing => ColoredPieceType::BlackKing,
            ColoredPieceType::BlackPawn => ColoredPieceType::WhitePawn,
            ColoredPieceType::BlackRook => ColoredPieceType::WhiteRook,
            ColoredPieceType::BlackKnight => ColoredPieceType::WhiteKnight,
            ColoredPieceType::BlackBishop => ColoredPieceType::WhiteBishop,
            ColoredPieceType::BlackQueen => ColoredPieceType::WhiteQueen,
            ColoredPieceType::BlackKing => ColoredPieceType::WhiteKing,
            _ => self,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Piece {
    square: Square,
    piece_type: ColoredPieceType,
}

impl Piece {
    pub fn new(square: Square, piece_type: ColoredPieceType) -> Piece {
        Piece { square, piece_type }
    }

    pub fn get_square(&self) -> Square {
        self.square
    }

    pub fn get_type(&self) -> ColoredPieceType {
        self.piece_type
    }
}
