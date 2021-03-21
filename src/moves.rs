use crate::bitboard::{Bitboard, BitboardExt};
use crate::common::*;
use crate::Move;

pub fn dumb7fill(mut fill: Bitboard, free: Bitboard, shift: i8) -> Bitboard {
    let mut flood = 0;

    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;
    let fill = fill.shift(shift) & free;
    flood |= fill;

    flood
}

pub fn bishop_attacks(from: u8, free: u64) -> Move {
    let fill = 1 << from;
    let mut targets = 0;
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, NE).shift(NE) & 0xFEFEFEFEFEFEFEFE;
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, SE).shift(SE) & 0xFEFEFEFEFEFEFEFE;
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, SW).shift(SW) & 0x7F7F7F7F7F7F7F7F;
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, NW).shift(NW) & 0x7F7F7F7F7F7F7F7F;

    Move::new(targets)
}

pub fn rook_attacks(from: u8, free: u64) -> Move {
    let fill = 1 << from;
    let mut targets = 0;
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, N).shift(N) & 0x7F7F7F7F7F7F7F7F;
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, E).shift(E) & 0xFEFEFEFEFEFEFEFE;
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, W).shift(W) & 0xFEFEFEFEFEFEFEFE;
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, S).shift(S) & 0xFEFEFEFEFEFEFEFE;

    Move::new(targets)
}

pub fn black_pawn_attacks(from: u8, free: u64) -> Move {
    let mut fill = 1 << from;
    let mut flood = fill;
    let fill = fill.shift(S) & free;
    flood |= fill;

    Move::new(flood)
}

pub fn white_pawn_attacks(from: u8, free: u64) -> Move {
    let mut fill = 1 << from;
    let mut flood = fill;
    let fill = fill.shift(N) & free;
    flood |= fill;

    Move::new(flood)
}

pub fn king_attacks(from: u8, free: u64) -> Move {
    let mut fill = 1 << from;
    let mut flood = fill;
    flood |= fill.shift(N) & 0x7F7F7F7F7F7F7F7F & free;
    flood |= fill.shift(E) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(S) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(W) & 0x7F7F7F7F7F7F7F7F & free;


    Move::new(flood)
}
