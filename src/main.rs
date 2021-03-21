extern crate num;
#[macro_use]
extern crate num_derive;

mod bitboard;
mod common;
mod moves;

use bitboard::{Bitboard, BitboardExt};
use common::*;
use moves::{bishop_attacks, rook_attacks, black_pawn_attacks, white_pawn_attacks, king_attacks};
use std::fmt;
use std::ops::{BitOr, BitAnd, BitXor};

#[derive(Copy, Clone, FromPrimitive)]
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
}

fn print_board(parts: Vec<(u8, u8, Piece)>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let coords: Vec<(u8, u8)> = parts.clone().into_iter().map(|a| (a.0, a.1)).collect();
    write!(f, "    a   b   c   d   e   f   g   h  \n")?;
    write!(f, "  ┌───┬───┬───┬───┬───┬───┬───┬───┐\n")?;
    for i in 0..8 {
        write!(f, "{} ", i)?;
        for j in 0..8 {
            if coords.contains(&(i, j)) {
                let index = coords.iter().position(|r| r == &(i, j)).unwrap();
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

enum Scope {
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
}
enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
}

#[derive(Clone)]
pub struct Move {
    mov: u64,
}

impl Move {
    pub fn new(x: u64) -> Move {
        Move { mov: x }
    }

    pub fn shift(self: &Move, x: i8) -> Move {
        if x > 0 {
            Move::new(self.mov << x)
        } else {
            Move::new(self.mov >> -x)
        }
    }
}

impl BitOr for Move {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mov: self.mov | rhs.mov,
        }
    }
}

impl BitAnd for Move {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            mov: self.mov & rhs.mov,
        }
    }
}

impl BitXor for Move {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            mov: self.mov ^ rhs.mov,
        }
    }
}


impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print_board(
            (0..64)
                .filter(|x| self.mov >> x & 1 == 1)
                .map(|x| (x >> 3, x & 0x7, Piece::Marker))
                .collect(),
            f,
        )
    }
}

#[derive(Default)]
struct Board {
    pieces: [u64; 13],
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
    fn new() -> Board {
        Board::read_fen("8/8/8/8/8/8/8/8".to_string())
    }

    fn read_fen(fen: String) -> Board {
        let vec = ['p', 'r', 'n', 'b', 'q', 'k', 'P', 'R', 'N', 'B', 'Q', 'K'];

        let mut board = Board {
            ..Default::default()
        };

        let mut rank: u8 = 0;
        let mut file: u8 = 0;

        for c in fen.chars() {
            let pos = vec.iter().position(|&r| r == c);

            match (pos, c) {
                (Some(o), _) => {
                    let piece: Piece = num::FromPrimitive::from_usize(o).unwrap();
                    board.set(&piece, (file, rank));
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

    fn set(self: &mut Board, piece: &Piece, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8 * x + y;
        let piece_index = *piece as u8;
        self.pieces[piece_index as usize] |= 1 << index;
    }

    fn clear(self: &mut Board, coords: (u8, u8)) {
        let (x, y) = coords;
        let index = 8 * x + y;

        for i in 0..12 {
            self.pieces[i] &= !(1 << index);
        }
    }

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

    fn check_occupancy(self: &Board, point: (u8, u8), scope: &Scope) -> bool {
        let occupancy = self.occupied(scope);
        let (x, y) = point;
        let index = 8 * x + y;

        return ((occupancy >> index) & 1) == 1;
    }

    fn get_pieces(self: &mut Board, scope: &Scope) -> Vec<(u8, u8, Piece)> {
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
    }

    fn move_piece(self: &mut Board, piece: &Piece, src: (u8, u8), dst: (u8, u8)) {
        self.clear(src);
        self.set(piece, dst);
    }

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
        let index = 8 * x + y;
        
        println!("piece is white: {}", piece.is_white());
        let occupied = Move::new(if piece.is_white() {
            self.occupied(&Scope::White)
        }
        else {
            self.occupied(&Scope::Black)
        });

        let mut mov = match piece {
            Piece::BlackRook | Piece::WhiteRook => {
                rook_attacks(index, !self.occupied(scope))
            }
            Piece::BlackBishop | Piece::WhiteBishop => {
                bishop_attacks(index, !self.occupied(scope))
            }
            Piece::BlackQueen | Piece::WhiteQueen => {
                bishop_attacks(index, !self.occupied(scope))
                    | rook_attacks(index, !self.occupied(scope))
            }
            Piece::BlackKing | Piece::WhiteKing => {
                king_attacks(index, !self.occupied(scope))
            }
            Piece::BlackPawn => {
                black_pawn_attacks(index, !self.occupied(scope))
            }
            Piece::WhitePawn => {
                white_pawn_attacks(index, !self.occupied(scope))
            }
            _ => {
                Move::new(1)
                //panic!(),
            }
        };

        println!("occupied {}", occupied.mov);
        mov.clone() ^ (mov.clone() & occupied)
    }

    fn generate_moves(self: &Board, scope: &Scope) -> Vec<Move> {
        let board = self.scoped(scope);
        let mut v = Vec::new();

        for piece in board.into_iter() {
            v.push(board.attack(piece, scope));
        }
        v
    }
}

fn generate_knigh_moves() -> Vec<Move> {
    let mut vec : Vec<Move> = Vec::new();

    let notA = 0x7F7F7F7F7F7F7F7F;
    let notH = 0xFEFEFEFEFEFEFEFE;

    for i in 0..64 {
        let mut mov = 0;
        let fill = 1 << i;
        mov |= fill.shift_p(N, notA).shift_p(E, notH).shift_p(E, notH);
        mov |= fill.shift_p(N, notA).shift_p(W, notA).shift_p(W, notA);
        mov |= fill.shift_p(N, notA).shift_p(N, notA).shift_p(E, notH);
        mov |= fill.shift_p(N, notA).shift_p(N, notA).shift_p(W, notA);
        mov |= fill.shift_p(S, notH).shift_p(E, notH).shift_p(E, notH);
        mov |= fill.shift_p(S, notH).shift_p(W, notA).shift_p(W, notA);
        mov |= fill.shift_p(S, notH).shift_p(S, notH).shift_p(E, notH);
        mov |= fill.shift_p(S, notH).shift_p(S, notH).shift_p(W, notA);
        vec.push(Move::new(mov));
    }

    vec
}

fn main() {
    //let board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq e3 0 1".to_string());
    //
    //let board = Board::read_fen("8/8/8/3q4/8/8/3Q4/8".to_string());
    let board = Board::read_fen("8/4p3/8/8/8/8/4P3/8".to_string());
    println!("{:?}", board);

    println!("{:?}", board.generate_moves(&Scope::All));

    println!("{:?}", generate_knigh_moves());
}
