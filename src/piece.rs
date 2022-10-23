use std::fmt;

#[derive(Copy, Clone, FromPrimitive, PartialEq)]
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
    NoPiece,
}


impl fmt::Debug for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pieces = [
            "♟︎", "♜", "♞", "♝", "♛", "♚", "♙", "♖", "♘", "♗", "♕", "♔", "*", " ",
        ];
        f.write_str(pieces[*self as usize])
    }
}

impl PieceType {
    pub fn is_black(self: &PieceType) -> bool {
        match self {
            PieceType::BlackPawn => true,
            PieceType::BlackRook => true,
            PieceType::BlackKnight => true,
            PieceType::BlackBishop => true,
            PieceType::BlackQueen => true,
            PieceType::BlackKing => true,
            _ => false,
        }
    }
    
    pub fn is_white(self: &PieceType) -> bool {
        match self {
            PieceType::WhitePawn => true,
            PieceType::WhiteRook => true,
            PieceType::WhiteKnight => true,
            PieceType::WhiteBishop => true,
            PieceType::WhiteQueen => true,
            PieceType::WhiteKing => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Piece {
    pub x: u8,
    pub y: u8,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(x:u8, y:u8, piece_type: PieceType) -> Piece {
        Piece {
            x,
            y,
            piece_type : piece_type,
        }
    }
}
