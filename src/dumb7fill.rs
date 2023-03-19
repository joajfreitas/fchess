use crate::bitboard::{Bitboard, BitboardExt};
use crate::common::*;
use crate::moves::{Board, MoveSet};
use crate::piece::PieceType;

pub fn dumb7fill(fill: Bitboard, free: Bitboard, shift: i8) -> Bitboard {
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

pub fn bishop_attacks(piece: PieceType, from: (u8, u8), free: u64) -> MoveSet {
    let (rank, file) = from;
    let fill = 1 << (8 * rank + file);
    let mut targets = 0;

    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, NE).shift_p(NE, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, SE).shift_p(SE, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, SW).shift_p(SW, 0x7F7F7F7F7F7F7F7F);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, NW).shift_p(NW, 0x7F7F7F7F7F7F7F7F);

    MoveSet::new(piece, from, targets)
}

pub fn rook_attacks(piece: PieceType, from: (u8, u8), free: u64) -> MoveSet {
    let (rank, file) = from;
    let fill = 1 << (8 * rank + file);
    let mut targets = 0;

    targets |= dumb7fill(fill, free, N).shift(N);
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, E).shift_p(E, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, W).shift_p(W, 0x7F7F7F7F7F7F7F7F);
    targets |= dumb7fill(fill, free, S).shift(S);

    MoveSet::new(piece, from, targets)
}

pub fn black_pawn_attacks(
    board: &Board,
    piece: PieceType,
    from: (u8, u8),
    friendlies: u64,
    enemy: u64,
) -> MoveSet {
    let mov = board.black_pawn_moves[(8 * from.0 + from.1) as usize];
    let mov = mov & !friendlies;
    let attack = board.black_pawn_attacks[(8 * from.0 + from.1) as usize];
    let attacks = attack & enemy;
    MoveSet::new(piece, from, mov | attacks)
}

pub fn white_pawn_attacks(
    board: &Board,
    piece: PieceType,
    from: (u8, u8),
    friendlies: u64,
    enemy: u64,
) -> MoveSet {
    let mov = board.white_pawn_moves[(8 * from.0 + from.1) as usize];
    let mov = mov & !friendlies & !enemy;
    let attack = board.white_pawn_attacks[(8 * from.0 + from.1) as usize];
    let attacks = attack & enemy;
    MoveSet::new(piece, from, mov | attacks)
}

pub fn king_attacks(piece: PieceType, from: (u8, u8), free: u64) -> MoveSet {
    let (rank, file) = from;
    let fill = 1 << (8 * rank + file);
    let mut flood = fill;
    flood |= fill.shift(N) & 0x7F7F7F7F7F7F7F7F & free;
    flood |= fill.shift(E) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(S) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(W) & 0x7F7F7F7F7F7F7F7F & free;

    MoveSet::new(piece, from, flood)
}

pub fn knight_attacks(board: &Board, piece: PieceType, from: (u8, u8), free: u64) -> MoveSet {
    MoveSet::new(
        piece,
        from,
        board.knight_moves[(8 * from.0 + from.1) as usize] & free,
    )
}
