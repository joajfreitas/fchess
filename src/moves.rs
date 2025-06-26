use anyhow::Result;
use std::fmt;
use std::ops::Not;

use crate::board::print_board;
use crate::board::Board;
use crate::piece::{ColoredPieceType, Piece};
use crate::san::read_san;
use crate::side::Side;
use crate::square::Square;

#[derive(Clone, Copy, Debug)]
pub enum Scope {
    All = 0,
    White = 1,
    Black = 2,
    None = 3,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
}

impl Not for Scope {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Scope::White => Scope::Black,
            Scope::Black => Scope::White,
            Scope::All => Scope::None,
            Scope::None => Scope::All,
            Scope::WhitePawn => Scope::BlackPawn,
            Scope::WhiteRook => Scope::BlackRook,
            Scope::WhiteKnight => Scope::BlackKnight,
            Scope::WhiteBishop => Scope::BlackBishop,
            Scope::WhiteQueen => Scope::BlackQueen,
            Scope::WhiteKing => Scope::BlackKing,
            Scope::BlackPawn => Scope::WhitePawn,
            Scope::BlackRook => Scope::WhiteRook,
            Scope::BlackKnight => Scope::WhiteKnight,
            Scope::BlackBishop => Scope::WhiteBishop,
            Scope::BlackQueen => Scope::WhiteQueen,
            Scope::BlackKing => Scope::WhiteKing,
        }
    }
}

impl Scope {
    pub fn to_range(self: &Scope) -> std::ops::Range<usize> {
        match self {
            Scope::All => 0..12,
            Scope::White => 0..6,
            Scope::Black => 6..12,
            Scope::None => 0..0,
            Scope::WhitePawn => 0..1,
            Scope::WhiteRook => 1..2,
            Scope::WhiteKnight => 2..3,
            Scope::WhiteBishop => 3..4,
            Scope::WhiteQueen => 4..5,
            Scope::WhiteKing => 5..6,
            Scope::BlackPawn => 6..7,
            Scope::BlackRook => 7..8,
            Scope::BlackKnight => 8..9,
            Scope::BlackBishop => 9..10,
            Scope::BlackQueen => 10..11,
            Scope::BlackKing => 11..12,
        }
    }

    pub fn reverse(self: &Scope) -> Scope {
        match self {
            Scope::White => Scope::Black,
            Scope::Black => Scope::White,
            _ => panic!(),
        }
    }
}

impl From<Side> for Scope {
    fn from(side: Side) -> Self {
        match side {
            Side::White => Scope::White,
            Side::Black => Scope::Black,
        }
    }
}

impl From<ColoredPieceType> for Scope {
    fn from(piece: ColoredPieceType) -> Self {
        match piece {
            ColoredPieceType::WhitePawn => Scope::WhitePawn,
            ColoredPieceType::WhiteRook => Scope::WhiteRook,
            ColoredPieceType::WhiteKnight => Scope::WhiteKnight,
            ColoredPieceType::WhiteBishop => Scope::WhiteBishop,
            ColoredPieceType::WhiteQueen => Scope::WhiteQueen,
            ColoredPieceType::WhiteKing => Scope::WhiteKing,
            ColoredPieceType::BlackPawn => Scope::BlackPawn,
            ColoredPieceType::BlackRook => Scope::BlackRook,
            ColoredPieceType::BlackKnight => Scope::BlackKnight,
            ColoredPieceType::BlackBishop => Scope::BlackBishop,
            ColoredPieceType::BlackQueen => Scope::BlackQueen,
            ColoredPieceType::BlackKing => Scope::BlackKing,
            ColoredPieceType::Marker => panic!(),
            ColoredPieceType::SourceMarker => panic!(),
            ColoredPieceType::NoPiece => panic!(),
        }
    }
}

/// Chess move
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Move {
    src: Square,
    dst: Square,
    target: Option<Square>,
    promotion: Option<ColoredPieceType>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = vec![
            Piece::new(self.src, ColoredPieceType::Marker),
            Piece::new(self.dst, ColoredPieceType::Marker),
        ];

        print_board(v, f)
    }
}

impl Move {
    pub fn new(src: Square, dst: Square) -> Move {
        Move {
            src,
            dst,
            target: None,
            promotion: None,
        }
    }

    pub fn with_promotion(src: Square, dst: Square, promotion: ColoredPieceType) -> Move {
        Move {
            src,
            dst,
            target: None,
            promotion: Some(promotion),
        }
    }

    pub fn get_src(&self) -> Square {
        self.src
    }
    pub fn get_dst(&self) -> Square {
        self.dst
    }
    pub fn get_target(&self) -> Option<Square> {
        self.target
    }
    pub fn set_target(&mut self, target: Option<Square>) {
        self.target = target
    }

    pub fn set_promotion(&mut self, promotion: Option<ColoredPieceType>) {
        self.promotion = promotion
    }

    pub fn get_promotion(&self) -> Option<ColoredPieceType> {
        self.promotion
    }

    pub fn from_full_algebraic(algebra: &str) -> Option<Move> {
        let mov: Vec<char> = algebra.chars().collect();
        if mov.len() == 4 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            Some(Move::new(
                Square::from_rank_file(src_rank, src_file),
                Square::from_rank_file(dst_rank, dst_file),
            ))
        } else if mov.len() == 5 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            let promotion = ColoredPieceType::from_char(&mov[4]);
            let mut mov = Move::new(
                Square::from_rank_file(src_rank, src_file),
                Square::from_rank_file(dst_rank, dst_file),
            );
            mov.set_promotion(promotion);
            Some(mov)
        } else {
            None
        }
    }

    pub fn from_san(algebra: &str, board: &Board) -> Result<Move> {
        read_san(algebra, board)
    }

    pub fn from_algebraic(algebra: &str) -> Option<Move> {
        let algebra: Vec<char> = algebra.chars().collect();
        if algebra.len() == 2 {
            None
        } else if algebra.len() == 4 || algebra.len() == 5 {
            let src_rank = (algebra[1] as u8) - b'1';
            let src_file = (algebra[0] as u8) - b'a';
            let dst_rank = (algebra[3] as u8) - b'1';
            let dst_file = (algebra[2] as u8) - b'a';

            let mut mov = Move::new(
                Square::from_rank_file(src_rank, src_file),
                Square::from_rank_file(dst_rank, dst_file),
            );

            if algebra.len() == 5 {
                mov.set_promotion(ColoredPieceType::from_char(&algebra[4]));
            }
            Some(mov)
        } else {
            None
        }
    }

    pub fn to_algebraic(&self) -> String {
        let dst_rank = (self.dst.get_file() + b'a') as char;
        let dst_file = (self.dst.get_rank() + b'1') as char;
        let src_rank = (self.src.get_file() + b'a') as char;
        let src_file = (self.src.get_rank() + b'1') as char;

        let promotion = if self.promotion.is_some() {
            self.promotion
                .unwrap()
                .to_char()
                .unwrap()
                .to_string()
                .to_uppercase()
        } else {
            "".to_string()
        };
        format!("{src_rank}{src_file}{dst_rank}{dst_file}{promotion}")
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::move_generator::MoveGenerator;
    use crate::square::Square;

    use super::Board;
    use super::Move;

    #[test]
    fn test_king_move() -> Result<()> {
        let move_generator = MoveGenerator::new();
        let board = Board::from_fen("8/K7/8/8/8/8/8/8")?;
        let origin = Square::from_algebraic("a7").unwrap();
        let moveset = move_generator
            .generate_moves_for_piece(&board, origin)
            .unwrap();

        assert_eq!(
            moveset.into_iter().collect::<Vec<Move>>(),
            vec![
                Move::new(origin, Square::from_algebraic("a6").unwrap()),
                Move::new(origin, Square::from_algebraic("b6").unwrap()),
                Move::new(origin, Square::from_algebraic("b7").unwrap()),
                Move::new(origin, Square::from_algebraic("a8").unwrap()),
                Move::new(origin, Square::from_algebraic("b8").unwrap()),
            ]
        );

        Ok(())
    }
}
