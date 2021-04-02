
use std::fmt;
use std::ops::{BitOr, BitAnd, BitXor};

use crate::dumb7fill::{bishop_attacks, rook_attacks, black_pawn_attacks, white_pawn_attacks, king_attacks, knight_attacks};
use crate::common::*;
use crate::bitboard::{Bitboard, BitboardExt};

#[derive(Copy, Clone, FromPrimitive, PartialEq)]
pub enum Piece {
    WhitePawn = 0,
    WhiteRook = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackRook = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen = 10,
    BlackKing = 11,
    Marker = 12,
    NoPiece,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pieces = [
            "♟︎", "♜", "♞", "♝", "♛", "♚", "♙", "♖", "♘", "♗", "♕", "♔", "*", " ",
        ];
        f.write_str(pieces[*self as usize])
    }
}

impl Piece {
    /*
    fn is_black(self: &Piece) -> bool {
        match self {
            Piece::BlackPawn => true,
            Piece::BlackRook => true,
            Piece::BlackKnight => true,
            Piece::BlackBishop => true,
            Piece::BlackQueen => true,
            Piece::BlackKing => true,
            _ => false,
        }
    }
    */
    
    /*
    fn is_white(self: &Piece) -> bool {
        match self {
            Piece::WhitePawn => true,
            Piece::WhiteRook => true,
            Piece::WhiteKnight => true,
            Piece::WhiteBishop => true,
            Piece::WhiteQueen => true,
            Piece::WhiteKing => true,
            _ => false,
        }
    }
    */
}

fn print_board(parts: Vec<(u8, u8, Piece)>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let coords: Vec<(u8, u8)> = parts.clone().into_iter().map(|a| (a.0, a.1)).collect();
    write!(f, "    a   b   c   d   e   f   g   h  \n")?;
    write!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n")?;
    for i in 0..8 {
        write!(f, "{} ", 8-i)?;
        for j in 0..8 {
            if coords.contains(&(7-i, j)) {
                let index = coords.iter().position(|r| r == &(7-i, j)).unwrap();
                write!(f, "│ {:?} ", parts[index].2)?;
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
pub struct Move {
    pub src: (u8, u8),
    pub piece: Piece,
    pub mov: u64,
}

impl Move {
    pub fn new(piece: Piece, src: (u8,u8), x: u64) -> Move {
        Move { 
            src: src,
            piece: piece,
            mov: x 
        }
    }

    pub fn shift(self: &Move, x: i8) -> Move {
        if x > 0 {
            Move::new(self.piece, self.src, self.mov << x)
        } else {
            Move::new(self.piece, self.src, self.mov >> -x)
        }
    }
}

#[derive(Clone)]
pub struct MoveAppliable {
    piece: Piece,
    src: (u8, u8),
    dst: (u8, u8),
}

impl fmt::Debug for MoveAppliable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v: Vec<(u8,u8,Piece)> = Vec::new();
        v.push((self.src.0, self.src.1, self.piece));
        v.push((self.dst.0, self.dst.1, Piece::Marker));

        print_board(v, f)
    }
}



pub fn algebraic(mov: MoveAppliable) -> String {
    let dst_rank = (mov.dst.1 + ('a' as u8)) as char; 
    let dst_file = (mov.dst.0 + ('1' as u8)) as char;
    let src_rank = (mov.src.1 + ('a' as u8)) as char;
    let src_file = (mov.src.0 + ('1' as u8)) as char;

    format!("{}{}{}{}",src_rank, src_file, dst_rank, dst_file)
}

impl<'a> IntoIterator for &'a Move {
    type Item = MoveAppliable;
    type IntoIter = MoveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MoveIterator {
            mov: self,
            index: 0,
        }
    }
}

pub struct MoveIterator<'a> {
    mov: &'a Move,
    index: u8,
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = MoveAppliable;

    fn next(&mut self) -> Option<MoveAppliable> {
        for i in self.index..64 {
            self.index += 1;
            if (self.mov.mov >> i) & 1 == 1 {
                return Some(MoveAppliable {
                    piece: self.mov.piece.clone(),
                    src: self.mov.src,
                    dst: (i/8, i%8),
                });
            }
        }
        return None;
    }
}


impl BitOr for Move {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov | rhs.mov,
        }
    }
}

impl BitAnd for Move {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov & rhs.mov,
        }
    }
}

impl BitXor for Move {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            piece: self.piece,
            src: self.src,
            mov: self.mov ^ rhs.mov,
        }
    }
}


impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = (0..64)
                .filter(|x| self.mov >> x & 1 == 1)
                .map(|x| (x >> 3, x & 0x7, Piece::Marker))
                .collect::<Vec<(u8,u8,Piece)>>();
        v.push((self.src.0, self.src.1,self.piece));

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
    type Item = (u8, u8, Piece);
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
    type Item = (u8, u8, Piece);

    fn next(&mut self) -> Option<(u8, u8, Piece)> {
        while (self.index as u64) < (64 as u64) * (13 as u64) {
            let pieces_index: u8 = (self.index / 64) as u8;
            let board_index: u8 = (self.index % 64) as u8;

            self.index += 1;
            let rank = board_index / 8;
            let file = board_index % 8;
            if (self.board.pieces[pieces_index as usize] >> board_index) & 1 == 1 {
                return Some((
                    rank,
                    file,
                    num::FromPrimitive::from_u8(pieces_index).unwrap(),
                ));
            }
        }

        return None;
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
                    let piece: Piece = num::FromPrimitive::from_usize(o).unwrap();
                    board.set(&piece, (7-file, rank));
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

    pub fn set(self: &mut Board, piece: &Piece, coords: (u8, u8)) {
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
            occupancy |= self.pieces[i as usize];
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

    fn piece_at(self: &Board, x: u8, y: u8) -> Piece {
        let index = 8 * x + y;

        for piece_index in 0..13 {
            let bit = (self.pieces[piece_index] >> index) & 1;
            if bit == 1 {
                return num::FromPrimitive::from_usize(piece_index).unwrap();
            }
        }
        return Piece::NoPiece;
    }

    fn attack(self: &Board, piece: (u8, u8, Piece), scope: &Scope) -> Move {
        let (x, y, piece) = piece;
        
        let occupied = self.occupied(scope);
        let enemy = self.occupied(&scope.reverse());

        let mov = match piece {
            Piece::BlackRook | Piece::WhiteRook => {
                rook_attacks(piece, (x,y), !(occupied | enemy))
            }
            Piece::BlackBishop | Piece::WhiteBishop => {
                bishop_attacks(piece, (x,y), !(occupied | enemy))
            }
            Piece::BlackQueen | Piece::WhiteQueen => {
                bishop_attacks(piece, (x,y), !(occupied | enemy))
                    | rook_attacks(piece, (x,y), !(occupied | enemy))
            }
            Piece::BlackKing | Piece::WhiteKing => {
                king_attacks(piece, (x,y), !self.occupied(scope))
            }
            Piece::BlackPawn => {
                black_pawn_attacks(self, piece, (x,y), occupied, enemy)
            }
            Piece::WhitePawn => {
                white_pawn_attacks(self, piece, (x,y), occupied, enemy)
            }
            Piece::BlackKnight | Piece::WhiteKnight => {
                knight_attacks(self, piece, (x,y), !self.occupied(scope))
            }
            _ => {
                Move::new(piece, (x,y), 1)
                //panic!(),
            }
        };
        
        // all except 
        let m = mov.mov ^ (mov.mov & occupied);
        Move::new(mov.piece, mov.src, m)
    }

    pub fn generate_moves(self: &Board, scope: &Scope) -> Vec<Move> {
        let board = self.scoped(scope);
        //let board = self;
        let mut v = Vec::new();

        for piece in board.into_iter() {
            let attack = self.attack(piece, scope);
            v.push(attack);
        }
        v
    }

    fn apply(self: &Board, mov: MoveAppliable) -> Board {
        let mut result = self.clone();
        
        let (src_rank, src_file) = mov.src;
        let (dst_rank, dst_file) = mov.dst;
        
        let piece_index = mov.piece as usize;
        for i in Scope::All.to_range() {
            if i == piece_index {
                result.pieces[i] &= 0xFFFFFFFFFFFFFFFF ^ (1 << (8*src_rank + src_file));
                result.pieces[i] |= 1 << (8*dst_rank + dst_file);
            }
            else {
                result.pieces[i] &= 0xFFFFFFFFFFFFFFFF ^ (1 << (8*dst_rank + dst_file));
            }
        }
        result.clone()
    }

    pub fn apply_algebraic_notation(self: &Board, mov: String) -> Board {
        let board = self.clone();
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            let rank = (mov[1] as u8) - ('1' as u8);
            let file = (mov[0] as u8) - ('a' as u8);
            let mov = MoveAppliable {
                src: (rank, file),
                dst: (rank, file),
                piece: Piece::WhitePawn,
            };

            return board.apply(mov);
        }

        if mov.len() == 4 {
            let src_rank = (mov[1] as u8) - ('1' as u8);
            let src_file = (mov[0] as u8) - ('a' as u8);
            let dst_rank = (mov[3] as u8) - ('1' as u8);
            let dst_file = (mov[2] as u8) - ('a' as u8);

            let piece = self.piece_at(src_rank, src_file);
            let mov = MoveAppliable {
                src: (src_rank, src_file),
                dst : (dst_rank, dst_file),
                piece: piece,
            };
            return board.apply(mov);
        }

        board
    }
    
    // evaled from the point of view of white
    fn eval(self: &Board) -> f32 {
        let pieces_values: [f32; 14] = [1.0, 5.0, 3.0, 3.0, 9.0, 100.0, -1.0, -5.0, -3.0, -3.0, -9.0, -100.0, 0.0, 0.0];
        
        let mut s : f32 = 0.0;
        for i in Scope::All.to_range() {
            s += (self.pieces[i].count_ones() as f32) * pieces_values[i];
        }

        if s > 100.0 {
            s = 100.0;
        }
        
        s
    }

    fn checkmate(self: &Board) -> bool {
        if self.pieces[Piece::WhiteKing as usize] == 0 {
            return true;
        }

        if self.pieces[Piece::BlackKing as usize] == 0 {
            return true;
        }

        return false;
    }
    
    fn min_max(self: &Board, scope: Scope, depth: u8) -> Option<f32> {
        //let mut best = None;
        let mut score = -500.0;

        if depth == 0 || self.checkmate() {
            return Some(self.eval());
        }


        for piece in self.generate_moves(&scope) {
            for mov in piece.into_iter() {
                let b = self.apply(mov.clone());
                let sc = b.min_max(scope.reverse(), depth-1);
                if sc.unwrap() > score {
                    score = sc.unwrap();
                    //best = Some(mov);
                }
            }
        }

        return Some(score);
    }

    pub fn best_move(self: &Board, scope: Scope) -> Option<MoveAppliable> {
        let mut best = None;
        let mut score = -500.0;
        for piece in self.generate_moves(&Scope::White) {
            for mov in piece.into_iter() {
                let b = self.apply(mov.clone());
                let sc = b.min_max(scope.reverse(), 2);
                if score < sc.unwrap() {
                    best = Some(mov);
                    score = sc.unwrap();
                }
            }
        }

        //println!("score: {}", score);
        return best;
    }
}

fn generate_knigh_moves() -> Vec<u64> {
    let mut vec : Vec<u64> = Vec::new();

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
        if i/8 == 1 {
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
        if i/8 == 6 {
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
        let mut fill: Bitboard = 1 << i;
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
        let mut fill: Bitboard = 1 << i;
        mov |= fill.shift(SE) & 0xFEFEFEFEFEFEFEFE;
        mov |= fill.shift(SW) & 0x7F7F7F7F7F7F7F7F;

        vec.push(mov);
    }

    vec
}
