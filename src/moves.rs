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
}

impl Not for Scope {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Scope::White => Scope::Black,
            Scope::Black => Scope::White,
            Scope::All => Scope::None,
            Scope::None => Scope::All,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Move {
    src: Square,
    dst: Square,
    target: Option<Square>,
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

    pub fn from_full_algebraic(algebra: &str) -> Option<Move> {
        let mov: Vec<char> = algebra.chars().collect();
        if mov.len() != 4 {
            None
        } else {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            Some(Move::new(
                Square::from_rank_file(src_rank, src_file),
                Square::from_rank_file(dst_rank, dst_file),
            ))
        }
    }

    fn san_match_type(letter: char, piece_type: PieceType) -> bool {
        let piece = (letter, piece_type);

        matches!(piece, ('P', PieceType::WhitePawn))
            | matches!(piece, ('P', PieceType::BlackPawn))
            | matches!(piece, ('R', PieceType::WhiteRook))
            | matches!(piece, ('R', PieceType::BlackRook))
            | matches!(piece, ('N', PieceType::WhiteKnight))
            | matches!(piece, ('N', PieceType::BlackKnight))
            | matches!(piece, ('B', PieceType::WhiteBishop))
            | matches!(piece, ('B', PieceType::BlackBishop))
            | matches!(piece, ('Q', PieceType::WhiteQueen))
            | matches!(piece, ('Q', PieceType::BlackQueen))
            | matches!(piece, ('K', PieceType::WhiteKing))
            | matches!(piece, ('K', PieceType::BlackKing))
    }

    pub fn from_san(algebra: &str, board: &Board) -> Option<Move> {
        let mov: Vec<char> = algebra.chars().collect();
        let move_generator = MoveGenerator::new();

        if mov.len() == 2 {
            let dst_rank = (mov[1] as u8) - b'1';
            let dst_file = (mov[0] as u8) - b'a';

            let dst = Square::from_rank_file(dst_rank, dst_file);

            let moves = move_generator.generate_moves(board);

            let mut resulting_move: Option<Move> = None;
            for moveset in moves {
                for mov in moveset.into_iter() {
                    if mov.dst == dst {
                        resulting_move = Some(mov);
                    }
                }
            }

            resulting_move
        } else if mov.len() == 3 {
            let piece_type = mov[0];
            let dst_rank = (mov[2] as u8) - b'1';
            let dst_file = (mov[1] as u8) - b'a';

            let dst = Square::from_rank_file(dst_rank, dst_file);

            let moves = move_generator.generate_moves(board);

            let mut resulting_move: Option<Move> = None;
            for moveset in moves {
                for mov in moveset.into_iter() {
                    if Move::san_match_type(piece_type, moveset.piece) && mov.dst == dst {
                        resulting_move = Some(mov);
                    }
                }
            }
            resulting_move
        } else if mov.len() == 4 {
            if mov[1] != 'x' {
                return None;
            }
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';
            let dst = Square::from_rank_file(dst_rank, dst_file);
            let moves = move_generator.generate_moves(board);

            let mut resulting_move: Option<Move> = None;
            for moveset in moves {
                for mov in moveset.into_iter() {
                    if mov.dst == dst && mov.get_src().get_file() == src_file {
                        resulting_move = Some(mov);
                    }
                }
            }

            resulting_move
        } else {
            None
        }
    }

    pub fn from_algebraic(mov: &str) -> Option<Move> {
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            None
        } else if mov.len() == 4 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            let mov = Move::new(
                Square::from_rank_file(src_rank, src_file),
                Square::from_rank_file(dst_rank, dst_file),
            );
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

        format!("{}{}{}{}", src_rank, src_file, dst_rank, dst_file)
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
