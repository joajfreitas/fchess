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

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Scope {
    All = 0,
    White = 1,
    Black = 2,
}

impl Scope {
    pub fn to_range(self: &Scope) -> std::ops::Range<usize> {
        match self {
            Scope::All => 0..12,
            Scope::White => 0..6,
            Scope::Black => 6..12,
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Move {
    src: Square,
    dst: Square,
}

impl fmt::Debug for Move {
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
        Move { src, dst }
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

    pub fn get_src(&self) -> Square {
        self.src
    }
    pub fn get_dst(&self) -> Square {
        self.dst
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

            let mov = Move {
                src: Square::from_rank_file(src_rank, src_file),
                dst: Square::from_rank_file(dst_rank, dst_file),
            };
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
                return Some(Move {
                    src: self.mov.src,
                    dst: Square::from_index(i),
                });
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

impl fmt::Debug for MoveSet {
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
        let board = board.scoped(&turn);

        println!("{:?}", board);

        board
            .into_iter()
            .map(|piece| self.attack(&board, &piece))
            .collect::<Vec<MoveSet>>()
    }

    pub fn generate_moves_for_piece(&self, board: &Board, square: Square) -> Option<MoveSet> {
        Some(self.attack(board, &Piece::new(square, board.piece_at(square)?)))
    }

    pub fn attack(&self, board: &Board, piece: &Piece) -> MoveSet {
        println!("{:?} {:?}", piece, &Scope::from(board.get_turn()));
        let square = piece.get_square();

        let occupied = board.occupied(&Scope::from(board.get_turn()));
        let enemy = board.occupied(&Scope::from(board.get_turn()).reverse());

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
                !board.occupied(&Scope::from(board.get_turn())),
            ),
            PieceType::BlackPawn => self.black_pawn_attacks(piece, square, occupied, enemy),
            PieceType::WhitePawn => self.white_pawn_attacks(piece, square, occupied, enemy),
            PieceType::BlackKnight | PieceType::WhiteKnight => self.knight_attacks(
                piece,
                square,
                !board.occupied(&Scope::from(board.get_turn())),
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
    ) -> MoveSet {
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
    ) -> MoveSet {
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
        flood |= fill.shift(N) & 0x7F7F7F7F7F7F7F7F & free;
        flood |= fill.shift(E) & 0xFEFEFEFEFEFEFEFE & free;
        flood |= fill.shift(S) & 0xFEFEFEFEFEFEFEFE & free;
        flood |= fill.shift(W) & 0x7F7F7F7F7F7F7F7F & free;

        MoveSet::new(from, piece, flood)
    }
}
