use crate::bitboard::{Bitboard, BitboardExt};

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
