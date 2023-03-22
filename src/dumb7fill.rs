use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::Board;
use crate::common::*;
use crate::moves::MoveSet;
use crate::piece::PieceType;
use crate::square::Square;

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

pub fn bishop_attacks(piece: PieceType, from: Square, free: u64) -> MoveSet {
    let fill = 1 << from.get_index();
    let mut targets = 0;

    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, NE).shift_p(NE, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, SE).shift_p(SE, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, SW).shift_p(SW, 0x7F7F7F7F7F7F7F7F);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, NW).shift_p(NW, 0x7F7F7F7F7F7F7F7F);

    MoveSet::new(from, piece, targets)
}

pub fn rook_attacks(piece: PieceType, from: Square, free: u64) -> MoveSet {
    let fill = 1 << from.get_index();
    let mut targets = 0;

    targets |= dumb7fill(fill, free, N).shift(N);
    targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, E).shift_p(E, 0xFEFEFEFEFEFEFEFE);
    targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, W).shift_p(W, 0x7F7F7F7F7F7F7F7F);
    targets |= dumb7fill(fill, free, S).shift(S);

    MoveSet::new(from, piece, targets)
}

pub fn black_pawn_attacks(
    board: &Board,
    piece: PieceType,
    from: Square,
    friendlies: u64,
    enemy: u64,
) -> MoveSet {
    let mov = board.black_pawn_moves[from.get_index() as usize];
    let mov = mov & !friendlies;
    let attack = board.black_pawn_attacks[from.get_index() as usize];
    let attacks = attack & enemy;
    MoveSet::new(from, piece, mov | attacks)
}

pub fn white_pawn_attacks(
    board: &Board,
    piece: PieceType,
    from: Square,
    friendlies: u64,
    enemy: u64,
) -> MoveSet {
    let mov = board.white_pawn_moves[from.get_index() as usize];
    let mov = mov & !friendlies & !enemy;
    let attack = board.white_pawn_attacks[from.get_index() as usize];
    let attacks = attack & enemy;
    MoveSet::new(from, piece, mov | attacks)
}

pub fn king_attacks(piece: PieceType, from: Square, free: u64) -> MoveSet {
    let fill = 1 << from.get_index();
    let mut flood = fill;
    flood |= fill.shift(N) & 0x7F7F7F7F7F7F7F7F & free;
    flood |= fill.shift(E) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(S) & 0xFEFEFEFEFEFEFEFE & free;
    flood |= fill.shift(W) & 0x7F7F7F7F7F7F7F7F & free;

    MoveSet::new(from, piece, flood)
}

pub fn knight_attacks(board: &Board, piece: PieceType, from: Square, free: u64) -> MoveSet {
    MoveSet::new(
        from,
        piece,
        board.knight_moves[(from.get_index()) as usize] & free,
    )
}
