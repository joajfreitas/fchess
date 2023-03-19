use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

use crate::bitboard::{Bitboard, BitboardExt};
use crate::common::*;
use crate::dumb7fill::{
    bishop_attacks, black_pawn_attacks, king_attacks, knight_attacks, rook_attacks,
    white_pawn_attacks,
};
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

fn print_board(pieces: Vec<Piece>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let coords: Vec<(u8, u8)> = pieces
        .clone()
        .into_iter()
        .map(|piece| (piece.x, piece.y))
        .collect();
    writeln!(f, "    a   b   c   d   e   f   g   h  ")?;
    writeln!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐")?;
    for i in 0..8 {
        write!(f, "{} ", 8 - i)?;
        for j in 0..8 {
            if coords.contains(&(7 - i, j)) {
                let index = coords.iter().position(|r| r == &(7 - i, j)).unwrap();
                write!(f, "│ {:?} ", pieces[index].piece_type)?;
            } else {
                write!(f, "│   ")?;
            }
        }

        if i != 7 {
            write!(f, "│\n  ├───┼───┼───┼───┼───┼───┼───┼───┤\n")?;
        }
    }
    write!(f, "│\n  └───┴───┴───┴───┴───┴───┴───┴───┘\n")?;
    f.write_str("")
}

pub enum Scope {
    All = 0,
    White = 1,
    Black = 2,
}

impl Scope {
    fn to_range(self: &Scope) -> std::ops::Range<usize> {
        match self {
            Scope::All => 0..12,
            Scope::White => 0..6,
            Scope::Black => 6..12,
        }
    }

    fn reverse(self: &Scope) -> Scope {
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
    src: (u8, u8),
    dst: (u8, u8),
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

#[derive(Default, Clone)]
pub struct Board {
    pieces: [u64; 13],
    pub knight_moves: Vec<u64>,
    pub black_pawn_moves: Vec<u64>,
    pub white_pawn_moves: Vec<u64>,
    pub black_pawn_attacks: Vec<u64>,
    pub white_pawn_attacks: Vec<u64>,
}

impl fmt::Debug for Board {
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
            let rank = board_index / 8;
            let file = board_index % 8;
            if (self.board.pieces[pieces_index as usize] >> board_index) & 1 == 1 {
                return Some(Piece::new(rank, file, self.board.piece_at(rank, file)?));
            }
        }

        None
    }
}

impl Board {
    pub fn new() -> Board {
        Board::read_fen("8/8/8/8/8/8/8/8".to_string())
    }

    pub fn read_fen(fen: String) -> Board {
        let vec = ['P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k'];

        let mut board = Board {
            knight_moves: generate_knigh_moves(),
            white_pawn_moves: generate_white_pawn_moves(),
            black_pawn_moves: generate_black_pawn_moves(),
            white_pawn_attacks: generate_white_pawn_attacks(),
            black_pawn_attacks: generate_black_pawn_attacks(),
            ..Default::default()
        };

        let mut rank: u8 = 0;
        let mut file: u8 = 0;

        for c in fen.chars() {
            let pos = vec.iter().position(|&r| r == c);

            match (pos, c) {
                (Some(o), _) => {
                    let piece: PieceType = num::FromPrimitive::from_usize(o).unwrap();
                    board.set(&piece, (7 - file, rank));
                    rank += 1;
                }
                (_, '/') => {
                    rank = 0;
                    file += 1;
                }
                (_, ' ') => break,
                (_, '0'..='9') => rank += c.to_digit(10).unwrap() as u8,
                _ => (),
            };
        }

        board
    }

    pub fn set(self: &mut Board, piece: &PieceType, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8 * x + y;
        let piece_index = *piece as u8;
        self.pieces[piece_index as usize] |= 1 << index;
    }

    /*
    fn clear(self: &mut Board, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8 * x + y;

        for i in 0..12 {
            self.pieces[i] &= !(1 << index);
        }
    }
    */

    // Create board with scope
    fn scoped(self: &Board, scope: &Scope) -> Board {
        let mut board = Board::new();
        for i in scope.to_range() {
            board.pieces[i] = self.pieces[i];
        }
        board
    }

    fn occupied(self: &Board, scope: &Scope) -> u64 {
        let mut occupancy: u64 = 0;

        for i in scope.to_range() {
            occupancy |= self.pieces[i];
        }
        occupancy
    }

    /*
    fn check_occupancy(self: &Board, point: (u8, u8), scope: &Scope) -> bool {
        let occupancy = self.occupied(scope);
        let (x, y) = point;
        let index = 8 * x + y;

        return ((occupancy >> index) & 1) == 1;
    }
    */

    /*fn get_pieces(self: &mut Board, scope: &Scope) -> Vec<(u8, u8, Piece)> {
        let mut pieces: Vec<(u8, u8, Piece)> = Vec::new();

        for i in scope.to_range() {
            for x in 0..8 {
                for y in 0..8 {
                    let index = 8 * x + y;
                    if (self.pieces[i as usize] >> index) & 1 == 1 {
                        pieces.push((x, y, num::FromPrimitive::from_usize(i).unwrap()));
                    }
                }
            }
        }

        pieces
    }*/

    /*fn move_piece(self: &mut Board, piece: &Piece, src: (u8, u8), dst: (u8, u8)) {
        self.clear(src);
        self.set(piece, dst);
    }*/

    fn piece_at(self: &Board, x: u8, y: u8) -> Option<PieceType> {
        let index = 8 * x + y;

        for piece_index in 0..13 {
            let bit = (self.pieces[piece_index] >> index) & 1;
            if bit == 1 {
                return num::FromPrimitive::from_usize(piece_index);
            }
        }
        Some(PieceType::NoPiece)
    }

    fn scope_at(self: &Board, x: u8, y: u8) -> Option<Scope> {
        let piece_type = self.piece_at(x, y)?;
        if piece_type.is_white() {
            Some(Scope::White)
        } else {
            Some(Scope::Black)
        }
    }

    fn attack(self: &Board, piece: &Piece, scope: &Scope) -> MoveSet {
        let x = piece.x;
        let y = piece.y;

        let occupied = self.occupied(scope);
        let enemy = self.occupied(&scope.reverse());

        let piece = self.piece_at(x, y).unwrap();

        let mov = match piece {
            PieceType::BlackRook | PieceType::WhiteRook => {
                rook_attacks(piece, (x, y), !(occupied | enemy))
            }
            PieceType::BlackBishop | PieceType::WhiteBishop => {
                bishop_attacks(piece, (x, y), !(occupied | enemy))
            }
            PieceType::BlackQueen | PieceType::WhiteQueen => {
                bishop_attacks(piece, (x, y), !(occupied | enemy))
                    | rook_attacks(piece, (x, y), !(occupied | enemy))
            }
            PieceType::BlackKing | PieceType::WhiteKing => {
                king_attacks(piece, (x, y), !self.occupied(scope))
            }
            PieceType::BlackPawn => black_pawn_attacks(self, piece, (x, y), occupied, enemy),
            PieceType::WhitePawn => white_pawn_attacks(self, piece, (x, y), occupied, enemy),
            PieceType::BlackKnight | PieceType::WhiteKnight => {
                knight_attacks(self, piece, (x, y), !self.occupied(scope))
            }
            _ => {
                MoveSet::new(piece, (x, y), 1)
                //panic!(),
            }
        };

        // all except
        let m = mov.mov ^ (mov.mov & occupied);
        MoveSet::new(mov.piece, mov.src, m)
    }

    pub fn generate_moves(self: &Board, scope: &Scope) -> Vec<MoveSet> {
        let board = self.scoped(scope);
        //let board = self;
        let mut v = Vec::new();

        for piece in board.into_iter() {
            let attack = self.attack(&piece, scope);
            v.push(attack);
        }
        v
    }

    pub fn generate_moves_for_piece(
        self: &Board,
        scope: &Scope,
        piece: (u8, u8),
    ) -> Option<MoveSet> {
        Some(self.attack(
            &Piece::new(piece.0, piece.1, self.piece_at(piece.0, piece.1)?),
            scope,
        ))
    }

    pub fn apply(self: &Board, mov: Move) -> Option<Board> {
        let mut result = self.clone();

        let (src_rank, src_file) = mov.src;
        let (dst_rank, dst_file) = mov.dst;

        //let piece_index = mov.piece as usize;
        let piece_index = self.piece_at(mov.src.0, mov.src.1).unwrap() as usize;

        let possible_moves =
            self.generate_moves_for_piece(&self.scope_at(mov.src.0, mov.src.1)?, mov.src)?;

        if !possible_moves.contains(&mov) {
            return None;
        }

        for i in Scope::All.to_range() {
            if i == piece_index {
                result.pieces[i] &= 0xFFFFFFFFFFFFFFFF ^ (1 << (8 * src_rank + src_file));
                result.pieces[i] |= 1 << (8 * dst_rank + dst_file);
            } else {
                result.pieces[i] &= 0xFFFFFFFFFFFFFFFF ^ (1 << (8 * dst_rank + dst_file));
            }
        }
        Some(result)
    }

    pub fn apply_algebraic_notation(self: &Board, mov: String) -> Option<Board> {
        let board = self.clone();
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            panic!();
        } else if mov.len() == 4 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';
            let dst_rank = (mov[3] as u8) - b'1';
            let dst_file = (mov[2] as u8) - b'a';

            let mov = Move {
                src: (src_rank, src_file),
                dst: (dst_rank, dst_file),
            };
            Some(board.apply(mov)?)
        } else {
            None
        }
    }

    // evaled from the point of view of white
    fn eval(self: &Board) -> f32 {
        let pieces_values: [f32; 14] = [
            1.0, 5.0, 3.0, 3.0, 9.0, 100.0, -1.0, -5.0, -3.0, -3.0, -9.0, -100.0, 0.0, 0.0,
        ];

        let mut s: f32 = 0.0;
        for i in Scope::All.to_range() {
            s += (self.pieces[i].count_ones() as f32) * pieces_values[i];
        }

        if s > 100.0 {
            s = 100.0;
        }

        s
    }

    fn checkmate(self: &Board) -> bool {
        if self.pieces[PieceType::WhiteKing as usize] == 0 {
            return true;
        }

        if self.pieces[PieceType::BlackKing as usize] == 0 {
            return true;
        }

        false
    }

    fn min_max(self: &Board, scope: Scope, depth: u8) -> Option<(f32, u32)> {
        //let mut best = None;
        let mut score = -500.0;
        let mut evals = 1;

        if depth == 0 || self.checkmate() {
            return Some((self.eval(), evals));
        }

        for piece in self.generate_moves(&scope) {
            for mov in piece.into_iter() {
                let b = self.apply(mov.clone())?;
                let sc = b.min_max(scope.reverse(), depth - 1);
                if sc.unwrap().0 > score {
                    score = sc.unwrap().0;
                }
                evals += sc.unwrap().1;
            }
        }

        Some((score, evals))
    }

    pub fn best_move(self: &Board, scope: Scope) -> Option<Move> {
        let mut best = None;
        let mut score = -500.0;

        let mut evals = 0;
        for piece in self.generate_moves(&scope) {
            for mov in piece.into_iter() {
                let b = self.apply(mov.clone())?;
                let (sc, min_max_evals) = b.min_max(scope.reverse(), 3).unwrap();
                if score < sc {
                    best = Some(mov);
                    score = sc;
                }
                evals += min_max_evals;
            }
        }

        println!("evaluations: {}", evals);
        best
    }
}

fn generate_knigh_moves() -> Vec<u64> {
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

fn generate_white_pawn_moves() -> Vec<u64> {
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

fn generate_black_pawn_moves() -> Vec<u64> {
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

fn generate_white_pawn_attacks() -> Vec<u64> {
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

fn generate_black_pawn_attacks() -> Vec<u64> {
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
