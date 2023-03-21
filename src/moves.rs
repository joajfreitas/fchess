use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::print_board;
use crate::common::*;
use crate::piece::{Piece, PieceType};

pub enum Side {
    White,
    Black,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}

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

#[derive(Clone)]
pub struct MoveSet {
    pub src: (u8, u8),
    pub piece: PieceType,
    pub mov: u64,
}

impl MoveSet {
    pub fn new(piece: PieceType, src: (u8, u8), x: u64) -> MoveSet {
        MoveSet { src, piece, mov: x }
    }

    pub fn shift(self: &MoveSet, x: i8) -> MoveSet {
        if x > 0 {
            MoveSet::new(self.piece, self.src, self.mov << x)
        } else {
            MoveSet::new(self.piece, self.src, self.mov >> -x)
        }
    }

    pub fn contains(&self, mov: &Move) -> bool {
        let index = (mov.dst.0 * 8) + mov.dst.1;
        (self.mov >> index) & 1 == 1
    }
}

#[derive(Clone)]
pub struct Move {
    pub src: (u8, u8),
    pub dst: (u8, u8),
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = vec![
            Piece::new(self.src.0, self.src.1, PieceType::Marker),
            Piece::new(self.dst.0, self.dst.1, PieceType::Marker),
        ];

        print_board(v, f)
    }
}

impl Move {
    pub fn from_algebraic_notation(mov: &str) -> Option<Move> {
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            None
        } else if mov.len() == 4 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            let mov = Move {
                src: (src_rank, src_file),
                dst: (dst_rank, dst_file),
            };
            Some(mov)
        } else {
            None
        }
    }
}

pub fn algebraic(mov: Move) -> String {
    let dst_rank = (mov.dst.1 + b'a') as char;
    let dst_file = (mov.dst.0 + b'1') as char;
    let src_rank = (mov.src.1 + b'a') as char;
    let src_file = (mov.src.0 + b'1') as char;

    format!("{}{}{}{}", src_rank, src_file, dst_rank, dst_file)
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
                    dst: (i / 8, i % 8),
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
            .map(|x| Piece::new(x >> 3, x & 0x7, PieceType::Marker))
            .collect::<Vec<Piece>>();
        v.push(Piece::new(self.src.0, self.src.1, PieceType::SourceMarker));

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
