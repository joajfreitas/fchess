use std::fmt;
use std::ops::Not;

use crate::side::Side;
use crate::square::Square;

const LAST_WHITE_OFFSET: u32 = 6;
const LAST_BLACK_OFFSET: u32 = 12;

#[allow(dead_code)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum PieceType {
    Pawn = 0,
    Rook = 1,
    Knight = 2,
    Bishop = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
    pub fn with_color(self, side: Side) -> ColoredPieceType {
        let side_offset: u32 = match side {
            Side::White => 0,
            Side::Black => LAST_WHITE_OFFSET,
        };

        num::FromPrimitive::from_u32(num::ToPrimitive::to_u32(&self).unwrap() + side_offset)
            .unwrap()
    }
}

#[derive(Copy, Clone, FromPrimitive, ToPrimitive, Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
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
        let offset = num::ToPrimitive::to_u32(self).unwrap();

        LAST_WHITE_OFFSET <= offset && offset < LAST_BLACK_OFFSET
    }

    pub fn is_white(self: &ColoredPieceType) -> bool {
        let offset = num::ToPrimitive::to_u32(self).unwrap();

        offset < LAST_WHITE_OFFSET
    }

    pub fn from_char(c: &char) -> Option<ColoredPieceType> {
        match c {
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
    pub fn to_char(self) -> Option<char> {
        match self {
            ColoredPieceType::WhitePawn => Some('P'),
            ColoredPieceType::WhiteRook => Some('R'),
            ColoredPieceType::WhiteKnight => Some('N'),
            ColoredPieceType::WhiteBishop => Some('B'),
            ColoredPieceType::WhiteQueen => Some('Q'),
            ColoredPieceType::WhiteKing => Some('K'),
            ColoredPieceType::BlackPawn => Some('p'),
            ColoredPieceType::BlackRook => Some('r'),
            ColoredPieceType::BlackKnight => Some('n'),
            ColoredPieceType::BlackBishop => Some('b'),
            ColoredPieceType::BlackQueen => Some('q'),
            ColoredPieceType::BlackKing => Some('k'),
            _ => None,
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
