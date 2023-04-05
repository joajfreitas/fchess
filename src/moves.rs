use std::convert::TryInto;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::print_board;
use crate::board::Board;
use crate::common::*;
use crate::dumb7fill::dumb7fill;
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
pub struct MoveSet {
    pub src: Square,
    pub piece: PieceType,
    pub mov: u64,
}

impl MoveSet {
    pub fn new(src: Square, piece: PieceType, x: u64) -> MoveSet {
        MoveSet { src, piece, mov: x }
    }

    pub fn shift(self: &MoveSet, x: i8) -> MoveSet {
        if x > 0 {
            MoveSet::new(self.src, self.piece, self.mov << x)
        } else {
            MoveSet::new(self.src, self.piece, self.mov >> -x)
        }
    }

    pub fn contains(&self, mov: &Move) -> bool {
        (self.mov >> mov.dst.get_index()) & 1 == 1
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

impl<'a> IntoIterator for &'a MoveSet {
    type Item = Move;
    type IntoIter = MoveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MoveIterator {
            mov: self,
            index: 0,
        }
    }
}

pub struct MoveIterator<'a> {
    mov: &'a MoveSet,
    index: u8,
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        for i in self.index..64 {
            self.index += 1;
            if (self.mov.mov >> i) & 1 == 1 {
                return Some(Move::new(self.mov.src, Square::from_index(i)));
            }
        }
        None
    }
}

impl BitOr for MoveSet {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov | rhs.mov,
        }
    }
}

impl BitAnd for MoveSet {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov & rhs.mov,
        }
    }
}

impl BitXor for MoveSet {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov ^ rhs.mov,
        }
    }
}

impl fmt::Display for MoveSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = (0..64)
            .filter(|x| self.mov >> x & 1 == 1)
            .map(|x| Piece::new(Square::from_index(x), PieceType::Marker))
            .collect::<Vec<Piece>>();
        v.push(Piece::new(self.src, PieceType::SourceMarker));

        print_board(v, f)
    }
}

pub fn generate_knight_moves() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    let not_a = 0x7F7F7F7F7F7F7F7F;
    let not_h = 0xFEFEFEFEFEFEFEFE;

    for i in 0..64 {
        let mut mov = 0;
        let fill: Bitboard = 1 << i;
        mov |= fill.shift_p(N, not_a).shift_p(E, not_h).shift_p(E, not_h);
        mov |= fill.shift_p(N, not_a).shift_p(W, not_a).shift_p(W, not_a);
        mov |= fill.shift_p(N, not_a).shift_p(N, not_a).shift_p(E, not_h);
        mov |= fill.shift_p(N, not_a).shift_p(N, not_a).shift_p(W, not_a);
        mov |= fill.shift_p(S, not_h).shift_p(E, not_h).shift_p(E, not_h);
        mov |= fill.shift_p(S, not_h).shift_p(W, not_a).shift_p(W, not_a);
        mov |= fill.shift_p(S, not_h).shift_p(S, not_h).shift_p(E, not_h);
        mov |= fill.shift_p(S, not_h).shift_p(S, not_h).shift_p(W, not_a);
        vec.push(mov);
    }

    vec
}

pub fn generate_white_pawn_moves() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for i in 0..64 {
        let mut mov = 0;
        let mut fill: Bitboard = 1 << i;
        if i / 8 == 1 {
            fill = fill.shift(N);
            mov |= fill;
        }
        mov |= fill.shift(N);

        vec.push(mov);
    }

    vec
}

pub fn generate_black_pawn_moves() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for i in 0..64 {
        let mut mov = 0;
        let mut fill: Bitboard = 1 << i;
        if i / 8 == 6 {
            fill = fill.shift(S);
            mov |= fill;
        }
        mov |= fill.shift(S);

        vec.push(mov);
    }

    vec
}

pub fn generate_white_pawn_attacks() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for i in 0..64 {
        let mut mov = 0;
        let fill: Bitboard = 1 << i;
        mov |= fill.shift(NE) & 0xFEFEFEFEFEFEFEFE;
        mov |= fill.shift(NW) & 0x7F7F7F7F7F7F7F7F;

        vec.push(mov);
    }

    vec
}

pub fn generate_black_pawn_attacks() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    for i in 0..64 {
        let mut mov = 0;
        let fill: Bitboard = 1 << i;
        mov |= fill.shift(SE) & 0xFEFEFEFEFEFEFEFE;
        mov |= fill.shift(SW) & 0x7F7F7F7F7F7F7F7F;

        vec.push(mov);
    }

    vec
}

#[derive(Clone, Copy, Debug)]
pub struct MoveGenerator {
    knight_moves: [u64; 64],
    black_pawn_moves: [u64; 64],
    white_pawn_moves: [u64; 64],
    black_pawn_attacks: [u64; 64],
    white_pawn_attacks: [u64; 64],
}

impl Default for MoveGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveGenerator {
    pub fn new() -> MoveGenerator {
        MoveGenerator {
            knight_moves: generate_knight_moves().try_into().unwrap(),
            black_pawn_moves: generate_black_pawn_moves().try_into().unwrap(),
            white_pawn_moves: generate_white_pawn_moves().try_into().unwrap(),
            black_pawn_attacks: generate_black_pawn_attacks().try_into().unwrap(),
            white_pawn_attacks: generate_white_pawn_attacks().try_into().unwrap(),
        }
    }

    pub fn generate_moves(&self, board: &Board) -> Vec<MoveSet> {
        let turn = Scope::from(board.get_turn());
        let scoped_board = board.scoped(turn);

        scoped_board
            .into_iter()
            .map(|piece| self.attack(board, &piece))
            .collect::<Vec<MoveSet>>()
    }

    pub fn generate_moves_for_piece(&self, board: &Board, square: Square) -> Option<MoveSet> {
        Some(self.attack(board, &Piece::new(square, board.piece_at(square)?)))
    }

    pub fn attack(&self, board: &Board, piece: &Piece) -> MoveSet {
        let square = piece.get_square();

        let occupied = board.occupied(Scope::from(board.get_turn()));
        let enemy = board.occupied(Scope::from(!board.get_turn()));

        let piece = board.piece_at(square).unwrap();

        let mov = match piece {
            PieceType::BlackRook | PieceType::WhiteRook => {
                self.rook_attacks(piece, square, !(occupied | enemy))
            }
            PieceType::BlackBishop | PieceType::WhiteBishop => {
                self.bishop_attacks(piece, square, !(occupied | enemy))
            }
            PieceType::BlackQueen | PieceType::WhiteQueen => {
                self.bishop_attacks(piece, square, !(occupied | enemy))
                    | self.rook_attacks(piece, square, !(occupied | enemy))
            }
            PieceType::BlackKing | PieceType::WhiteKing => self.king_attacks(
                piece,
                square,
                !board.occupied(Scope::from(board.get_turn())),
            ),
            PieceType::BlackPawn => {
                self.black_pawn_attacks(piece, square, occupied, enemy, board.get_enpassant())
            }
            PieceType::WhitePawn => {
                self.white_pawn_attacks(piece, square, occupied, enemy, board.get_enpassant())
            }
            PieceType::BlackKnight | PieceType::WhiteKnight => self.knight_attacks(
                piece,
                square,
                !board.occupied(Scope::from(board.get_turn())),
            ),
            _ => {
                MoveSet::new(square, piece, 1)
                //panic!(),
            }
        };

        // all except
        let m = mov.mov ^ (mov.mov & occupied);
        MoveSet::new(mov.src, mov.piece, m)
    }

    pub fn black_pawn_attacks(
        &self,
        piece: PieceType,
        from: Square,
        friendlies: u64,
        enemy: u64,
        enpassant: Option<Square>,
    ) -> MoveSet {
        let mut enemy = enemy;
        if let Some(enpassant) = enpassant {
            enemy |= 1 << enpassant.get_index()
        }
        let mov = self.black_pawn_moves[from.get_index() as usize];
        let mov = mov & !friendlies;
        let attack = self.black_pawn_attacks[from.get_index() as usize];
        let attacks = attack & enemy;
        MoveSet::new(from, piece, mov | attacks)
    }

    pub fn white_pawn_attacks(
        &self,
        piece: PieceType,
        from: Square,
        friendlies: u64,
        enemy: u64,
        enpassant: Option<Square>,
    ) -> MoveSet {
        let mut enemy = enemy;
        if let Some(enpassant) = enpassant {
            enemy |= 1 << enpassant.get_index();
        }
        let mov = self.white_pawn_moves[from.get_index() as usize];
        let mov = mov & !friendlies & !enemy;
        let attack = self.white_pawn_attacks[from.get_index() as usize];
        let attacks = attack & enemy;
        MoveSet::new(from, piece, mov | attacks)
    }
    pub fn knight_attacks(&self, piece: PieceType, from: Square, free: u64) -> MoveSet {
        MoveSet::new(
            from,
            piece,
            self.knight_moves[(from.get_index()) as usize] & free,
        )
    }

    pub fn bishop_attacks(&self, piece: PieceType, from: Square, free: u64) -> MoveSet {
        let fill = 1 << from.get_index();
        let mut targets = 0;

        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, NE).shift_p(NE, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, SE).shift_p(SE, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, SW).shift_p(SW, 0x7F7F7F7F7F7F7F7F);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, NW).shift_p(NW, 0x7F7F7F7F7F7F7F7F);

        MoveSet::new(from, piece, targets)
    }

    pub fn rook_attacks(&self, piece: PieceType, from: Square, free: u64) -> MoveSet {
        let fill = 1 << from.get_index();
        let mut targets = 0;

        targets |= dumb7fill(fill, free, N).shift(N);
        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, E).shift_p(E, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, W).shift_p(W, 0x7F7F7F7F7F7F7F7F);
        targets |= dumb7fill(fill, free, S).shift(S);

        MoveSet::new(from, piece, targets)
    }

    pub fn king_attacks(&self, piece: PieceType, from: Square, free: u64) -> MoveSet {
        let fill = 1 << from.get_index();
        let mut flood = fill;
        flood |= fill.shift(N) & free;
        flood |= fill.shift(E) & 0xFEFEFEFEFEFEFEFE & free;
        flood |= fill.shift(S) & free;
        flood |= fill.shift(W) & 0x7F7F7F7F7F7F7F7F & free;
        flood |= fill.shift(NE) & 0xFEFEFEFEFEFEFEFE & free;
        flood |= fill.shift(SE) & 0xFEFEFEFEFEFEFEFE & free;
        flood |= fill.shift(SW) & 0x7F7F7F7F7F7F7F7F & free;
        flood |= fill.shift(NW) & 0x7F7F7F7F7F7F7F7F & free;

        MoveSet::new(from, piece, flood)
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
