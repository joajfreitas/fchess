use std::fmt;

use crate::bitwise;

use crate::fen::read_fen;
use crate::moves::{Move, Scope};
use crate::piece::{ColoredPieceType, Piece};
use crate::side::Side;
use crate::square::Square;
use crate::zobrist_hash::zobrist_hash;
use anyhow::Result;

pub trait Mask {
    fn shift_down(&self) -> Self;
    fn shift_up(&self) -> Self;
    fn shift_left(&self) -> Self;
    fn shift_right(&self) -> Self;
}

impl Mask for u64 {
    fn shift_down(&self) -> u64 {
        self >> 8
    }

    fn shift_up(&self) -> u64 {
        self << 8
    }

    fn shift_left(&self) -> u64 {
        self >> 1
    }

    fn shift_right(&self) -> u64 {
        self << 1
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Castling {
    WhiteShort,
    WhiteLong,
    BlackShort,
    BlackLong,
}

/// Bitboard representation of the chess board
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Board {
    pieces: [u64; 13],
    turn: Side,
    castling_rights: u8,
    enpassant: Option<Square>,
    half_move_clock: u8,
    full_move_clock: u8,
}

pub fn print_board(pieces: Vec<Piece>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let coords: Vec<Square> = pieces
        .clone()
        .into_iter()
        .map(|piece| (piece.get_square()))
        .collect();
    writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;
    for i in 0..8 {
        write!(f, "{} ", 8 - i)?;
        for j in 0..8 {
            if coords.contains(&Square::from_rank_file(7 - i, j)) {
                let index = coords
                    .iter()
                    .position(|r| *r == Square::from_rank_file(7 - i, j))
                    .unwrap();
                write!(f, "│ {} ", pieces[index].get_type())?;
            } else {
                write!(f, "│   ")?;
            }
        }

        if i != 7 {
            write!(f, "│\n  ├───┼───┼───┼───┼───┼───┼───┼───┤\n")?;
        }
    }
    write!(f, "│\n  └───┴───┴───┴───┴───┴───┴───┴───┘\n")?;
    writeln!(f, "    a   b   c   d   e   f   g   h  ")?;
    f.write_str("")
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pieces = self.into_iter().collect();
        print_board(pieces, f)
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = Piece;
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            board: self,
            index: 0,
        }
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    index: u32,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Piece;

    fn next(&mut self) -> Option<Piece> {
        while (self.index as u64) < 64_u64 * 13_u64 {
            let pieces_index: u8 = (self.index / 64) as u8;
            let board_index: u8 = (self.index % 64) as u8;

            self.index += 1;
            if (self.board.pieces[pieces_index as usize] >> board_index) & 1 == 1 {
                return Some(Piece::new(
                    Square::from_index(board_index),
                    self.board.piece_at(Square::from_index(board_index))?,
                ));
            }
        }

        None
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            full_move_clock: 1,
            ..Default::default()
        }
    }

    pub fn get_turn(&self) -> Side {
        self.turn
    }

    pub fn set_turn(&mut self, turn: Side) {
        self.turn = turn
    }

    pub fn set_castling_white_short(&mut self, enabled: bool) {
        self.castling_rights = bitwise::set_bit(self.castling_rights, 0, enabled as u8);
    }

    pub fn set_castling_white_long(&mut self, enabled: bool) {
        self.castling_rights = bitwise::set_bit(self.castling_rights, 1, enabled as u8);
    }

    pub fn set_castling_black_short(&mut self, enabled: bool) {
        self.castling_rights = bitwise::set_bit(self.castling_rights, 2, enabled as u8);
    }

    pub fn set_castling_black_long(&mut self, enabled: bool) {
        self.castling_rights = bitwise::set_bit(self.castling_rights, 3, enabled as u8);
    }

    pub fn get_castling_rights(&self) -> u8 {
        self.castling_rights
    }

    pub fn get_castling_white_short(&self) -> bool {
        bitwise::get_bit(self.castling_rights, 0) != 0
    }

    pub fn get_castling_white_long(&self) -> bool {
        bitwise::get_bit(self.castling_rights, 1) != 0
    }

    pub fn get_castling_black_short(&self) -> bool {
        bitwise::get_bit(self.castling_rights, 2) != 0
    }

    pub fn get_castling_black_long(&self) -> bool {
        bitwise::get_bit(self.castling_rights, 3) != 0
    }

    pub fn set_enpassant(&mut self, square: Option<Square>) {
        self.enpassant = square;
    }

    pub fn get_enpassant(&self) -> Option<Square> {
        self.enpassant
    }

    pub fn get_half_move_clock(&self) -> u8 {
        self.half_move_clock
    }

    pub fn set_half_move_clock(&mut self, half_move_clock: u8) {
        self.half_move_clock = half_move_clock;
    }

    pub fn get_full_move_clock(&self) -> u8 {
        self.full_move_clock
    }

    pub fn set_full_move_clock(&mut self, full_move_clock: u8) {
        self.full_move_clock = full_move_clock;
    }

    pub fn from_basic_board() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Board> {
        read_fen(fen)
    }

    pub fn get_piece_mask(&self, piece_type: ColoredPieceType) -> u64 {
        self.pieces[piece_type as usize]
    }

    // Create board with scope
    pub fn scoped(self: &Board, scope: Scope) -> Board {
        let mut board = self.clone();
        for i in (!scope).to_range() {
            board.pieces[i] = 0;
        }
        board
    }

    pub fn occupied(self: &Board, scope: Scope) -> u64 {
        let mut occupancy: u64 = 0;

        for i in scope.to_range() {
            occupancy |= self.pieces[i];
        }
        occupancy
    }

    pub fn piece_at(self: &Board, square: Square) -> Option<ColoredPieceType> {
        for piece_index in 0..13 {
            let bit = (self.pieces[piece_index] >> square.get_index()) & 1;
            if bit == 1 {
                return num::FromPrimitive::from_usize(piece_index);
            }
        }
        Some(ColoredPieceType::NoPiece)
    }

    pub fn set_piece(&mut self, square: Square, piece_type: ColoredPieceType) {
        self.pieces[piece_type as usize] =
            bitwise::enable_bit(self.pieces[piece_type as usize], square.get_index());
    }

    pub fn set_piece_type(&mut self, piece_type: ColoredPieceType, mask: u64) {
        self.pieces[piece_type as usize] = mask;
    }

    fn is_castle(&self, mov: Move) -> Option<(Move, Move, Castling)> {
        let piece_type = self.piece_at(mov.get_src())?;
        let src_rank = mov.get_src().get_rank();
        let src_file = mov.get_src().get_file();
        let dst_rank = mov.get_dst().get_rank();
        let dst_file = mov.get_dst().get_file();

        match ((src_rank, src_file), (dst_rank, dst_file), piece_type) {
            ((0, 4), (0, 2), ColoredPieceType::WhiteKing) => Some((
                mov,
                Move::new(Square::from_rank_file(0, 0), Square::from_rank_file(0, 3)),
                Castling::WhiteLong,
            )),
            ((0, 4), (0, 6), ColoredPieceType::WhiteKing) => Some((
                mov,
                Move::new(Square::from_rank_file(0, 7), Square::from_rank_file(0, 5)),
                Castling::WhiteShort,
            )),
            ((7, 4), (7, 2), ColoredPieceType::BlackKing) => Some((
                mov,
                Move::new(Square::from_rank_file(7, 0), Square::from_rank_file(7, 3)),
                Castling::BlackLong,
            )),
            ((7, 4), (7, 6), ColoredPieceType::BlackKing) => Some((
                mov,
                Move::new(Square::from_rank_file(7, 7), Square::from_rank_file(7, 5)),
                Castling::BlackShort,
            )),
            _ => None,
        }
    }

    fn apply_single_move(self: &Board, mov: Move) -> Option<Board> {
        let mut result = self.clone();

        let piece_type = self.piece_at(mov.get_src()).unwrap() as usize;

        for i in Scope::All.to_range() {
            result.pieces[i] &= 0xFFFFFFFFFFFFFFFF ^ (1 << mov.get_dst().get_index());
        }

        result.pieces[piece_type] &= 0xFFFFFFFFFFFFFFFF ^ (1 << mov.get_src().get_index());

        result.pieces[piece_type] |= 1 << mov.get_dst().get_index();

        Some(result)
    }

    pub fn apply(self: &Board, mov: &Move) -> Option<Board> {
        let castle = self.is_castle(mov.clone());
        let mut result = if castle.is_some() {
            let castle = castle.unwrap();
            let mut board = self.apply_single_move(castle.0)?;
            match castle.2 {
                Castling::WhiteShort => board.set_castling_white_short(false),
                Castling::WhiteLong => board.set_castling_white_long(false),
                Castling::BlackShort => board.set_castling_black_short(false),
                Castling::BlackLong => board.set_castling_black_long(false),
            }
            board.apply_single_move(castle.1)
        } else {
            self.apply_single_move(mov.clone())
        }?;

        if mov.get_promotion().is_some() {
            let piece_type = self.piece_at(mov.get_src()).unwrap() as usize;
            result.pieces[piece_type] &= 0xFFFFFFFFFFFFFFFF ^ (1 << mov.get_dst().get_index());
            result.pieces[mov.get_promotion().unwrap() as usize] |= 1 << mov.get_dst().get_index();
        }

        match (mov.get_src().get_rank(), mov.get_src().get_file()) {
            (0, 0) => result.set_castling_white_long(false),
            (0, 7) => result.set_castling_white_short(false),
            (7, 0) => result.set_castling_black_long(false),
            (7, 7) => result.set_castling_black_short(false),
            (0, 4) => {
                result.set_castling_white_long(false);
                result.set_castling_white_short(false);
            }
            (7, 4) => {
                result.set_castling_black_long(false);
                result.set_castling_black_short(false);
            }
            _ => {}
        }
        result.set_enpassant(None);

        let piece_type = self.piece_at(mov.get_src()).unwrap();

        if piece_type == ColoredPieceType::WhitePawn {
            if mov.get_dst().get_rank() - mov.get_src().get_rank() == 2 {
                result.set_enpassant(Some(Square::from_rank_file(
                    mov.get_src().get_rank() + 1,
                    mov.get_src().get_file(),
                )));
            }
        } else if piece_type == ColoredPieceType::BlackPawn
            && mov.get_src().get_rank() - mov.get_dst().get_rank() == 2
        {
            result.set_enpassant(Some(Square::from_rank_file(
                mov.get_dst().get_rank() + 1,
                mov.get_dst().get_file(),
            )));
        }

        if Some(mov.get_dst()) == self.get_enpassant() {
            let clear_square = if piece_type == ColoredPieceType::BlackPawn {
                Some(Square::from_rank_file(
                    mov.get_dst().get_rank() + 1,
                    mov.get_dst().get_file(),
                ))
            } else if piece_type == ColoredPieceType::WhitePawn {
                Some(Square::from_rank_file(
                    mov.get_dst().get_rank() - 1,
                    mov.get_dst().get_file(),
                ))
            } else {
                None
            };

            if let Some(clear_square) = clear_square {
                result.pieces[!piece_type as usize] &= !(1 << clear_square.get_index());
            }
        }

        result.set_turn(!self.get_turn());
        if result.get_turn() == Side::White {
            result.set_full_move_clock(result.get_full_move_clock() + 1);
        }

        let moved_piece = self.piece_at(mov.get_src()).unwrap();
        let target_piece = self.piece_at(mov.get_dst()).unwrap();

        let mov = (moved_piece, target_piece);

        let halfmove_clock_reset = matches!(mov, (ColoredPieceType::WhitePawn, _))
            | matches!(mov, (ColoredPieceType::BlackPawn, _))
            | matches!(mov, (_, ColoredPieceType::WhitePawn))
            | matches!(mov, (_, ColoredPieceType::BlackPawn))
            | (target_piece != ColoredPieceType::NoPiece);

        if halfmove_clock_reset {
            result.set_half_move_clock(0);
        } else {
            result.set_half_move_clock(self.get_half_move_clock() + 1);
        }

        if self.get_enpassant().is_some() {
            result.set_enpassant(None);
        }

        Some(result)
    }

    pub fn checkmate(self: &Board) -> bool {
        if self.pieces[ColoredPieceType::WhiteKing as usize] == 0 {
            return true;
        }

        if self.pieces[ColoredPieceType::BlackKing as usize] == 0 {
            return true;
        }

        false
    }

    pub fn zobrist_hash(&self) -> u64 {
        zobrist_hash(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Board;
    use super::ColoredPieceType;
    use super::Piece;
    use super::Scope;
    use super::Square;

    #[test]
    fn test_board_iterator() {
        //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
        // 8 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 7 │   │   │   │   │ ♙ │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 6 │   │   │   │ ♖ │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 5 │ ♛ │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 4 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 3 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 2 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 1 │   │   │   │   │   │   │   │   │
        //   └───┴───┴───┴───┴───┴───┴───┴───┘
        //     a   b   c   d   e   f   g   h

        let board = Board::from_fen("8/4p3/3r4/Q7/8/8/8/8").unwrap();
        assert_eq!(
            board.into_iter().collect::<Vec<Piece>>(),
            vec![
                Piece::new(
                    Square::from_algebraic("a5").unwrap(),
                    ColoredPieceType::WhiteQueen
                ),
                Piece::new(
                    Square::from_algebraic("e7").unwrap(),
                    ColoredPieceType::BlackPawn
                ),
                Piece::new(
                    Square::from_algebraic("d6").unwrap(),
                    ColoredPieceType::BlackRook
                ),
            ]
        );
    }

    #[test]
    fn test_board_scoped() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap()
                .scoped(Scope::White),
            Board::from_fen("8/8/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
        )
    }

    #[test]
    fn test_board_occupied_white() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap()
                .occupied(Scope::White),
            0x000000000000FFFF
        )
    }
    #[test]
    fn test_board_occupied_all() {
        assert_eq!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
                .unwrap()
                .occupied(Scope::All),
            0xFFFF00000000FFFF
        )
    }

    #[test]
    fn test_board_piece_at() {
        let board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(
            Some(ColoredPieceType::WhiteRook),
            board.piece_at(Square::from_algebraic("a1").unwrap()),
        )
    }

    #[test]
    fn test_board_set_piece() {
        let mut board = Board::new();
        board.set_piece(Square::from_rank_file(7, 0), ColoredPieceType::WhiteKing);

        assert_eq!(
            board
                .into_iter()
                .next()
                .expect("There should be a piece on the board")
                .get_square()
                .to_algebraic(),
            "a8".to_string()
        );
    }
}
