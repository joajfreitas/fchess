use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor};

use crate::board::print_board;
use crate::moves::Move;
use crate::piece::{Piece, PieceType};
use crate::square::Square;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MoveSet {
    pub src: Square,
    pub piece: PieceType,
    pub mov: u64,
}

impl MoveSet {
    pub fn new(src: Square, piece: PieceType, x: u64) -> MoveSet {
        MoveSet { src, piece, mov: x }
    }

    pub fn shift(self: &MoveSet, x: i8) -> MoveSet {
        if x > 0 {
            MoveSet::new(self.src, self.piece, self.mov << x)
        } else {
            MoveSet::new(self.src, self.piece, self.mov >> -x)
        }
    }

    pub fn contains(&self, mov: &Move) -> bool {
        (self.mov >> mov.get_dst().get_index()) & 1 == 1
    }
}

impl<'a> IntoIterator for &'a MoveSet {
    type Item = Move;
    type IntoIter = MoveIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MoveIterator {
            mov: self,
            index: 0,
            promotion_index: 0,
        }
    }
}

pub struct MoveIterator<'a> {
    mov: &'a MoveSet,
    index: u8,
    promotion_index: u8,
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = Move;

    fn next(&mut self) -> Option<Move> {
        for i in self.index..64 {
            if (self.mov.mov >> i) & 1 == 1 {
                let destination = Square::from_index(i);
                if self.mov.piece == PieceType::WhitePawn && destination.get_rank() == 7 {
                    let mov = match self.promotion_index {
                        0 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::WhiteQueen,
                        )),
                        1 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::WhiteRook,
                        )),
                        2 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::WhiteBishop,
                        )),
                        3 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::WhiteKnight,
                        )),
                        _ => panic!(),
                    };

                    if self.promotion_index >= 3 {
                        self.index += 1;
                        self.promotion_index = 0;
                    } else {
                        self.promotion_index += 1;
                    }

                    return mov;
                } else if self.mov.piece == PieceType::BlackPawn && destination.get_rank() == 0 {
                    let mov = match self.promotion_index {
                        0 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::BlackQueen,
                        )),
                        1 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::BlackRook,
                        )),
                        2 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::BlackBishop,
                        )),
                        3 => Some(Move::with_promotion(
                            self.mov.src,
                            destination,
                            PieceType::BlackKnight,
                        )),
                        _ => panic!(),
                    };

                    if self.promotion_index >= 3 {
                        self.index += 1;
                        self.promotion_index = 0;
                    } else {
                        self.promotion_index += 1;
                    }

                    return mov;
                } else {
                    self.index += 1;
                    return Some(Move::new(self.mov.src, destination));
                }
            } else {
                self.index += 1;
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

impl fmt::Display for MoveSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = (0..64)
            .filter(|x| self.mov >> x & 1 == 1)
            .map(|x| Piece::new(Square::from_index(x), PieceType::Marker))
            .collect::<Vec<Piece>>();
        v.push(Piece::new(self.src, PieceType::SourceMarker));

        print_board(v, f)
    }
}
