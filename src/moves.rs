use std::fmt;
use std::ops::Not;

use crate::board::print_board;
use crate::board::Board;
use crate::move_generator::MoveGenerator;
use crate::piece::{Piece, PieceType};
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

impl From<PieceType> for Scope {
    fn from(piece: PieceType) -> Self {
        match piece {
            PieceType::WhitePawn => Scope::WhitePawn,
            PieceType::WhiteRook => Scope::WhiteRook,
            PieceType::WhiteKnight => Scope::WhiteKnight,
            PieceType::WhiteBishop => Scope::WhiteBishop,
            PieceType::WhiteQueen => Scope::WhiteQueen,
            PieceType::WhiteKing => Scope::WhiteKing,
            PieceType::BlackPawn => Scope::BlackPawn,
            PieceType::BlackRook => Scope::BlackRook,
            PieceType::BlackKnight => Scope::BlackKnight,
            PieceType::BlackBishop => Scope::BlackBishop,
            PieceType::BlackQueen => Scope::BlackQueen,
            PieceType::BlackKing => Scope::BlackKing,
            PieceType::Marker => panic!(),
            PieceType::SourceMarker => panic!(),
            PieceType::NoPiece => panic!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Move {
    src: Square,
    dst: Square,
    target: Option<Square>,
    promotion: Option<PieceType>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = vec![
            Piece::new(self.src, PieceType::Marker),
            Piece::new(self.dst, PieceType::Marker),
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

    pub fn set_promotion(&mut self, promotion: Option<PieceType>) {
        self.promotion = promotion
    }

    pub fn get_promotion(&self) -> Option<PieceType> {
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

            let promotion = PieceType::from_string(&mov[4]);
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

    fn san_match_type(piece_type: PieceType, scope: Scope) -> bool {
        match (piece_type, scope) {
            (_, Scope::All) => true,
            (PieceType::WhitePawn, Scope::WhitePawn) => true,
            (PieceType::BlackPawn, Scope::BlackPawn) => true,
            (PieceType::WhiteRook, Scope::WhiteRook) => true,
            (PieceType::BlackRook, Scope::BlackRook) => true,
            (PieceType::WhiteKnight, Scope::WhiteKnight) => true,
            (PieceType::BlackKnight, Scope::BlackKnight) => true,
            (PieceType::WhiteBishop, Scope::WhiteBishop) => true,
            (PieceType::BlackBishop, Scope::BlackBishop) => true,
            (PieceType::WhiteQueen, Scope::WhiteQueen) => true,
            (PieceType::BlackQueen, Scope::BlackQueen) => true,
            (PieceType::WhiteKing, Scope::WhiteKing) => true,
            (PieceType::BlackKing, Scope::BlackKing) => true,
            _ => false,
        }
    }

    fn from_two_char_san(mov: Vec<char>) -> Option<Square> {
        let dst_rank = (mov[1] as u8) - b'1';
        let dst_file = (mov[0] as u8) - b'a';

        Some(Square::from_rank_file(dst_rank, dst_file))
    }

    fn from_four_char_san(mov: Vec<char>) -> Option<(u8, Square)> {
        let dst_rank = (mov[3] as u8) - b'1';
        let dst_file = (mov[2] as u8) - b'a';

        let src_file = (mov[0] as u8) - b'a';

        Some((src_file, Square::from_rank_file(dst_rank, dst_file)))
    }

    pub fn from_san(algebra: &str, board: &Board) -> Option<Move> {
        let mov: Vec<char> = algebra.chars().collect();
        let move_generator = MoveGenerator::new();

        let (src_rank, src_file, dst, scope, promotion) = if mov.len() == 2 {
            (None, None, Move::from_two_char_san(mov), Scope::All, None)
        } else if mov.len() == 3 {
            let mut piece = Scope::from(PieceType::from_string(&mov[0])?);
            if board.get_turn() == Side::Black {
                piece = !piece;
            }
            (
                None,
                None,
                Move::from_two_char_san(mov[1..].iter().map(|x| *x).collect::<Vec<char>>()),
                piece,
                None,
            )
        } else if mov.len() == 4 && mov[2] == '=' {
            (
                None,
                None,
                Move::from_two_char_san(mov.clone()),
                Scope::All,
                Some(PieceType::from_string(&mov[3])?),
            )
        } else if mov.len() == 4 && mov[1] == 'x' {
            let (src_file, dst) = Move::from_four_char_san(mov)?;
            (None, Some(src_file), Some(dst), Scope::All, None)
        } else if mov.len() == 4 {
            let mut piece = Scope::from(PieceType::from_string(&mov[0])?);
            if board.get_turn() == Side::Black {
                piece = !piece;
            }

            let src_file = (mov[1] as u8) - b'a';
            (
                None,
                Some(src_file),
                Move::from_two_char_san(mov[2..].iter().map(|x| *x).collect::<Vec<char>>()),
                piece,
                None,
            )
        } else if mov.len() == 6 && mov[4] == '=' && mov[1] == 'x' {
            let (src_file, dst) = Move::from_four_char_san(mov.clone())?;

            (
                None,
                Some(src_file),
                Some(dst),
                Scope::All,
                Some(PieceType::from_string(&mov[5])?),
            )
        } else {
            (None, None, None, Scope::All, None)
        };
        if !src_rank.is_some() && !src_file.is_some() && !dst.is_some() {
            return None;
        }

        let mut resulting_move: Option<Move> = None;
        let moves = move_generator.generate_moves(board);
        for moveset in moves {
            for mov in moveset.into_iter() {
                let piece_type = board.piece_at(mov.get_src())?;
                if (!src_rank.is_some() || Some(mov.get_src().get_rank()) == src_rank)
                    && (!src_file.is_some() || Some(mov.get_src().get_file()) == src_file)
                    && (!dst.is_some() || Some(mov.get_dst()) == dst)
                    && Move::san_match_type(piece_type, scope)
                {
                    resulting_move = Some(mov);
                }
            }
        }
        resulting_move.as_mut()?.set_promotion(promotion);
        resulting_move
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
                mov.set_promotion(PieceType::from_string(&algebra[4]));
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
            format!("{}", self.promotion.unwrap().to_string().to_uppercase())
        } else {
            "".to_string()
        };
        format!(
            "{}{}{}{}{}",
            src_rank, src_file, dst_rank, dst_file, promotion
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::square::Square;

    use super::Board;
    use super::Move;
    use super::MoveGenerator;

    #[test]
    fn test_king_move() {
        let move_generator = MoveGenerator::new();
        let board = Board::from_fen("8/K7/8/8/8/8/8/8");
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
        )
    }
}
