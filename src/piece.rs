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
    SourceMarker = 13,
    NoPiece,
}

impl fmt::Debug for PieceType {
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
    pub x: u8,
    pub y: u8,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(x: u8, y: u8, piece_type: PieceType) -> Piece {
        Piece { x, y, piece_type }
    }
}
