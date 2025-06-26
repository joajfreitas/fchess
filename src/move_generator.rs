use std::convert::TryInto;

use crate::bitboard::{Bitboard, BitboardExt};
use crate::board::Board;
use crate::common::*;
use crate::dumb7fill::dumb7fill;
use crate::moves::Move;
use crate::moves::Scope;
use crate::moveset::MoveSet;
use crate::piece::{ColoredPieceType, Piece, PieceType};
use crate::side::Side;
use crate::square::Square;
use crate::utils::print_u64;

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
        let fill: Bitboard = 1 << i;
        if i / 8 == 1 {
            mov |= fill.shift(N).shift(N);
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
        let fill: Bitboard = 1 << i;
        if i / 8 == 6 {
            mov |= fill.shift(S).shift(S);
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
        MoveGenerator::new()
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

    pub fn attacked_square(&self, board: &Board, square: Square) -> bool {
        let opposite_side = dbg!(!board.get_turn()); // white

        let occupied = board.occupied(Scope::from(board.get_turn()));
        let enemy = board.occupied(Scope::from(!board.get_turn()));
        let bitboard: Bitboard = 1u64 << square.get_index();

        let mov = self.rook_attacks(
            PieceType::Rook.with_color(opposite_side),
            square,
            !(occupied | enemy),
        );

        let mask = board.get_piece_mask(PieceType::Rook.with_color(opposite_side))
            | board.get_piece_mask(PieceType::Queen.with_color(opposite_side));

        let mut attacked = (mask & mov.mov) != 0;

        let mov = self.bishop_attacks(
            PieceType::Rook.with_color(opposite_side),
            square,
            !(occupied | enemy),
        );

        let mask = board.get_piece_mask(PieceType::Bishop.with_color(opposite_side))
            | board.get_piece_mask(PieceType::Queen.with_color(opposite_side));

        attacked |= (mask & mov.mov) != 0;

        let mov = if board.get_turn() == Side::White {
            bitboard.shift(NW) | bitboard.shift(NE)
        } else {
            bitboard.shift(SW) | bitboard.shift(SE)
        };

        let mask = board.get_piece_mask(PieceType::Pawn.with_color(opposite_side));

        print_u64(mask);
        print_u64(mov);

        attacked |= (mask & mov) != 0;

        let mov = self.knight_attacks(
            PieceType::Knight.with_color(board.get_turn()),
            square,
            !occupied,
        );

        let mask = board.get_piece_mask(PieceType::Knight.with_color(opposite_side));

        attacked |= (mask & mov.mov) != 0;

        let mov = self.king_attacks(
            PieceType::King.with_color(opposite_side),
            square,
            !occupied,
            board,
        );

        let mask = board.get_piece_mask(PieceType::King.with_color(opposite_side));

        attacked |= (mask & mov.mov) != 0;

        attacked
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
            ColoredPieceType::BlackRook | ColoredPieceType::WhiteRook => {
                self.rook_attacks(piece, square, !(occupied | enemy))
            }
            ColoredPieceType::BlackBishop | ColoredPieceType::WhiteBishop => {
                self.bishop_attacks(piece, square, !(occupied | enemy))
            }
            ColoredPieceType::BlackQueen | ColoredPieceType::WhiteQueen => {
                self.bishop_attacks(piece, square, !(occupied | enemy))
                    | self.rook_attacks(piece, square, !(occupied | enemy))
            }
            ColoredPieceType::BlackKing | ColoredPieceType::WhiteKing => self.king_attacks(
                piece,
                square,
                !board.occupied(Scope::from(board.get_turn())),
                board,
            ),
            ColoredPieceType::BlackPawn | ColoredPieceType::WhitePawn => {
                self.pawn_attacks(piece, square, occupied, enemy, board.get_enpassant())
            }
            ColoredPieceType::BlackKnight | ColoredPieceType::WhiteKnight => self.knight_attacks(
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
        piece: ColoredPieceType,
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

        enemy |= ((1 << from.get_index()).shift(S) & enemy).shift(S);
        let mov = mov & !friendlies & !enemy;
        let attack = self.black_pawn_attacks[from.get_index() as usize];
        let attacks = attack & enemy;
        MoveSet::new(from, piece, mov | attacks)
    }

    pub fn white_pawn_attacks(
        &self,
        piece: ColoredPieceType,
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

        enemy |= ((1 << from.get_index()).shift(N) & enemy).shift(N);

        let mov = mov & !friendlies & !enemy;
        let attack = self.white_pawn_attacks[from.get_index() as usize];
        let attacks = attack & enemy;
        MoveSet::new(from, piece, mov | attacks)
    }

    pub fn pawn_attacks(
        &self,
        piece: ColoredPieceType,
        from: Square,
        friendlies: u64,
        enemy: u64,
        enpassant: Option<Square>,
    ) -> MoveSet {
        match piece {
            ColoredPieceType::WhitePawn => {
                self.white_pawn_attacks(piece, from, friendlies, enemy, enpassant)
            }
            ColoredPieceType::BlackPawn => {
                self.black_pawn_attacks(piece, from, friendlies, enemy, enpassant)
            }
            _ => panic!("Invalid piece type for pawn attacks"),
        }
    }

    pub fn knight_attacks(&self, piece: ColoredPieceType, from: Square, free: u64) -> MoveSet {
        MoveSet::new(
            from,
            piece,
            self.knight_moves[(from.get_index()) as usize] & free,
        )
    }

    pub fn bishop_attacks(&self, piece: ColoredPieceType, from: Square, free: u64) -> MoveSet {
        let fill = 1 << from.get_index();
        let mut targets = 0;

        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, NE).shift_p(NE, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, SE).shift_p(SE, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, SW).shift_p(SW, 0x7F7F7F7F7F7F7F7F);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, NW).shift_p(NW, 0x7F7F7F7F7F7F7F7F);

        MoveSet::new(from, piece, targets)
    }

    pub fn rook_attacks(&self, piece: ColoredPieceType, from: Square, free: u64) -> MoveSet {
        let fill = 1 << from.get_index();
        let mut targets = 0;

        targets |= dumb7fill(fill, free, N).shift(N);
        targets |= dumb7fill(fill, free & 0xFEFEFEFEFEFEFEFE, E).shift_p(E, 0xFEFEFEFEFEFEFEFE);
        targets |= dumb7fill(fill, free & 0x7F7F7F7F7F7F7F7F, W).shift_p(W, 0x7F7F7F7F7F7F7F7F);
        targets |= dumb7fill(fill, free, S).shift(S);

        MoveSet::new(from, piece, targets)
    }

    pub fn king_attacks(
        &self,
        piece: ColoredPieceType,
        from: Square,
        free: u64,
        board: &Board,
    ) -> MoveSet {
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

        let mut aux = board.clone();
        aux.set_turn(!aux.get_turn());
        aux.set_piece_type(!piece, 0);

        let moves = self.generate_moves(&aux);

        let enemies = moves
            .iter()
            .map(|mov| mov.mov)
            .reduce(|mov1, mov2| mov1 | mov2)
            .unwrap_or(0);

        if piece == ColoredPieceType::WhiteKing {
            let b1 = Square::from_rank_file(0, 1);
            let c1 = Square::from_rank_file(0, 2);
            let d1 = Square::from_rank_file(0, 3);
            let f1 = Square::from_rank_file(0, 5);
            let g1 = Square::from_rank_file(0, 6);

            let long_unoccupied = board.piece_at(b1) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(c1) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(d1) == Some(ColoredPieceType::NoPiece)
                && enemies >> b1.get_index() == 0
                && enemies >> c1.get_index() == 0
                && enemies >> d1.get_index() == 0;
            let short_unoccupied = board.piece_at(f1) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(g1) == Some(ColoredPieceType::NoPiece)
                && enemies >> f1.get_index() == 0
                && enemies >> g1.get_index() == 0;
            if board.get_castling_white_long() && long_unoccupied {
                flood |= 1 << Square::from_rank_file(0, 2).get_index();
            }
            if board.get_castling_white_short() && short_unoccupied {
                flood |= 1 << Square::from_rank_file(0, 6).get_index();
            }
        }

        if piece == ColoredPieceType::BlackKing {
            let b8 = Square::from_rank_file(7, 1);
            let c8 = Square::from_rank_file(7, 2);
            let d8 = Square::from_rank_file(7, 3);
            let f8 = Square::from_rank_file(7, 5);
            let g8 = Square::from_rank_file(7, 6);
            let long_unoccupied = board.piece_at(b8) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(c8) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(d8) == Some(ColoredPieceType::NoPiece)
                && enemies >> b8.get_index() == 0
                && enemies >> c8.get_index() == 0
                && enemies >> d8.get_index() == 0;
            let short_unoccupied = board.piece_at(f8) == Some(ColoredPieceType::NoPiece)
                && board.piece_at(g8) == Some(ColoredPieceType::NoPiece)
                && enemies >> f8.get_index() == 0
                && enemies >> g8.get_index() == 0;
            if board.get_castling_black_long() && long_unoccupied {
                flood |= 1 << Square::from_rank_file(7, 2).get_index();
            }
            if board.get_castling_black_short() && short_unoccupied {
                flood |= 1 << Square::from_rank_file(7, 6).get_index();
            }
        }

        flood &= !(1 << from.get_index());

        let moveset = MoveSet::new(from, piece, flood);
        moveset
    }

    pub fn check(&self, board: &Board) -> bool {
        self.attacked_square(board, board.king(board.get_turn()))
    }

    pub fn checkmate(&self, board: &Board) -> bool {
        if !self.check(board) {
            println!("Not in check");
            return false;
        }

        let moves = self.generate_moves(board);

        let mut checkmate = true;

        for moveset in moves {
            println!("{}", moveset);
            for mov in moveset.into_iter() {
                let result = board.apply(&mov);
                if result.is_none() {
                    continue;
                }

                let mut result = result.unwrap();
                result.set_turn(dbg!(!result.get_turn()));

                println!("Checking move: {}", mov.to_algebraic());
                println!("Resulting board:\n{}", result);
                checkmate &= self.check(&result);
            }
        }

        checkmate
        //moves
        //    .iter()
        //    .map(|moveset| {
        //        moveset
        //            .into_iter()
        //            .map(|mov| board.apply(&mov))
        //            .any(|board| {
        //                board
        //                    .as_ref()
        //                    .map(|board| {
        //                        let mut board = board.clone();
        //                        board.set_turn(!board.get_turn());
        //                        self.check(&board)
        //                    })
        //                    .unwrap_or(false)
        //            })
        //    })
        //    .any(|check| check) // board.apply(mov)).any(|board| !self.check(&board))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;
    use crate::board_builder::BoardBuilder;
    use crate::side::Side;

    #[rstest]
    #[case("d7", ColoredPieceType::WhiteRook)]
    #[case("b7", ColoredPieceType::WhiteBishop)]
    #[case("c4", ColoredPieceType::WhitePawn)]
    #[case("c7", ColoredPieceType::WhiteKnight)]
    fn test_attacked_square(#[case] square: &str, #[case] piece_type: ColoredPieceType) {
        let board = BoardBuilder::new()
            .with_piece("e1", ColoredPieceType::WhiteKing)
            .with_piece("d5", ColoredPieceType::BlackKing)
            .with_piece(square, piece_type)
            .with_turn(Side::Black)
            .build();

        println!("Board:\n{}", board);

        let mov_gen = MoveGenerator::new();

        assert!(mov_gen.check(&board));
    }

    #[rstest]
    #[case("1R3k2/2R5/8/8/8/1K6/8/8 b - - 0 1")]
    #[case("8/8/1k6/8/8/8/2r5/1r3K2 w - - 0 1")]
    #[case("8/6N1/3R4/6k1/5Pp1/1K2P3/8/4B1R1 b - f3 0 1")]
    #[case("4b1r1/8/1k2p3/5pP1/6K1/3r4/6n1/8 w - f6 0 1")]
    #[case("kr6/ppN5/8/8/8/8/2K5/8 b - - 0 1")]
    #[case("8/2k5/8/8/8/8/PPn5/KR6 w - - 0 1")]
    #[case("k1K5/p1N5/8/8/8/8/8/8 b - - 0 1")]
    #[case("8/8/8/8/8/8/P1n5/K1k5 w - - 0 1")]
    fn test_checkmate(#[case] fen: &str) {
        let board = BoardBuilder::new().with_fen(fen).unwrap().build();

        let mov_gen = MoveGenerator::new();

        assert!(mov_gen.checkmate(&board));
    }
}
