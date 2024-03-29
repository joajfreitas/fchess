use std::convert::TryInto;

use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::Board;
use crate::common::*;
use crate::dumb7fill::dumb7fill;
use crate::moves::Scope;
use crate::moveset::MoveSet;
use crate::piece::{Piece, PieceType};
use crate::square::Square;

pub fn generate_knight_moves() -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();

    let not_a = 0x7F7F7F7F7F7F7F7F;
    let not_h = 0xFEFEFEFEFEFEFEFE;
    let all = 0xFFFFFFFFFFFFFFFF;

    for i in 0..64 {
        let mut mov = 0;
        let fill: Bitboard = 1 << i;
        mov |= fill.shift_p(N, all).shift_p(E, not_h).shift_p(E, not_h);
        mov |= fill.shift_p(N, all).shift_p(W, not_a).shift_p(W, not_a);
        mov |= fill.shift_p(N, all).shift_p(N, all).shift_p(E, not_h);
        mov |= fill.shift_p(N, all).shift_p(N, all).shift_p(W, not_a);
        mov |= fill.shift_p(S, all).shift_p(E, not_h).shift_p(E, not_h);
        mov |= fill.shift_p(S, all).shift_p(W, not_a).shift_p(W, not_a);
        mov |= fill.shift_p(S, all).shift_p(S, all).shift_p(E, not_h);
        mov |= fill.shift_p(S, all).shift_p(S, all).shift_p(W, not_a);
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
