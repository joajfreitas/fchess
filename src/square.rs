//          ┌───┬───┬───┬───┬───┬───┬───┬───┐
//        8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        7 │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        6 │   │   │   │   │   │   │   │   │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        5 │   │   │   │   │   │   │   │   │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
// rank ↑ 4 │   │   │   │   │   │   │   │   │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        3 │   │   │   │   │   │   │   │   │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        2 │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │
//          ├───┼───┼───┼───┼───┼───┼───┼───┤
//        1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
//          └───┴───┴───┴───┴───┴───┴───┴───┘
//            a   b   c   d   e   f   g   h
//
//                         -→
//                        file
#![allow(dead_code)]
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Square {
    square: u8,
}

impl Square {
    pub const fn from_rank_file(rank: u8, file: u8) -> Square {
        assert!(rank < 8, "Rank must be between 0 and 7");
        assert!(file < 8, "File must be between 0 and 7");
        Square {
            square: (rank << 3) + file,
        }
    }

    pub fn from_index(square: u8) -> Square {
        Square { square }
    }

    pub fn to_index(&self) -> u8 {
        self.square
    }

    pub fn from_algebraic(mov: &str) -> Option<Square> {
        let mov: Vec<char> = mov.chars().collect();
        if mov.len() == 2 {
            let src_rank = (mov[1] as u8) - b'1';
            let src_file = (mov[0] as u8) - b'a';

            Some(Square::from_rank_file(src_rank, src_file))
        } else {
            None
        }
    }

    pub fn to_algebraic(self) -> String {
        format!(
            "{}{}",
            (self.get_file() + b'a') as char,
            (self.get_rank() + b'1') as char
        )
    }

    pub fn get_rank(&self) -> u8 {
        (self.square >> 3) & 0b111
    }
    pub fn set_rank(&mut self, rank: u8) {
        self.square &= 0b000111;
        self.square |= rank << 3;
    }

    pub fn get_file(&self) -> u8 {
        self.square & 0b111
    }

    pub fn set_file(&mut self, file: u8) {
        self.square &= 0b111000;
        self.square |= file;
    }

    pub const fn get_index(&self) -> u8 {
        self.square
    }

    pub fn get_mask(&self) -> u64 {
        1 << self.square
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_algebraic())
    }
}

pub const A1: Square = Square::from_rank_file(0, 0);
pub const A2: Square = Square::from_rank_file(1, 0);
pub const A3: Square = Square::from_rank_file(2, 0);
pub const A4: Square = Square::from_rank_file(3, 0);
pub const A5: Square = Square::from_rank_file(4, 0);
pub const A6: Square = Square::from_rank_file(5, 0);
pub const A7: Square = Square::from_rank_file(6, 0);
pub const A8: Square = Square::from_rank_file(7, 0);

pub const B1: Square = Square::from_rank_file(0, 1);
pub const B2: Square = Square::from_rank_file(1, 1);
pub const B3: Square = Square::from_rank_file(2, 1);
pub const B4: Square = Square::from_rank_file(3, 1);
pub const B5: Square = Square::from_rank_file(4, 1);
pub const B6: Square = Square::from_rank_file(5, 1);
pub const B7: Square = Square::from_rank_file(6, 1);
pub const B8: Square = Square::from_rank_file(7, 1);

pub const C1: Square = Square::from_rank_file(0, 2);
pub const C2: Square = Square::from_rank_file(1, 2);
pub const C3: Square = Square::from_rank_file(2, 2);
pub const C4: Square = Square::from_rank_file(3, 2);
pub const C5: Square = Square::from_rank_file(4, 2);
pub const C6: Square = Square::from_rank_file(5, 2);
pub const C7: Square = Square::from_rank_file(6, 2);
pub const C8: Square = Square::from_rank_file(7, 2);

pub const D1: Square = Square::from_rank_file(0, 3);
pub const D2: Square = Square::from_rank_file(1, 3);
pub const D3: Square = Square::from_rank_file(2, 3);
pub const D4: Square = Square::from_rank_file(3, 3);
pub const D5: Square = Square::from_rank_file(4, 3);
pub const D6: Square = Square::from_rank_file(5, 3);
pub const D7: Square = Square::from_rank_file(6, 3);
pub const D8: Square = Square::from_rank_file(7, 3);

pub const E1: Square = Square::from_rank_file(0, 4);
pub const E2: Square = Square::from_rank_file(1, 4);
pub const E3: Square = Square::from_rank_file(2, 4);
pub const E4: Square = Square::from_rank_file(3, 4);
pub const E5: Square = Square::from_rank_file(4, 4);
pub const E6: Square = Square::from_rank_file(5, 4);
pub const E7: Square = Square::from_rank_file(6, 4);
pub const E8: Square = Square::from_rank_file(7, 4);

pub const F1: Square = Square::from_rank_file(0, 5);
pub const F2: Square = Square::from_rank_file(1, 5);
pub const F3: Square = Square::from_rank_file(2, 5);
pub const F4: Square = Square::from_rank_file(3, 5);
pub const F5: Square = Square::from_rank_file(4, 5);
pub const F6: Square = Square::from_rank_file(5, 5);
pub const F7: Square = Square::from_rank_file(6, 5);
pub const F8: Square = Square::from_rank_file(7, 5);

pub const G1: Square = Square::from_rank_file(0, 6);
pub const G2: Square = Square::from_rank_file(1, 6);
pub const G3: Square = Square::from_rank_file(2, 6);
pub const G4: Square = Square::from_rank_file(3, 6);
pub const G5: Square = Square::from_rank_file(4, 6);
pub const G6: Square = Square::from_rank_file(5, 6);
pub const G7: Square = Square::from_rank_file(6, 6);
pub const G8: Square = Square::from_rank_file(7, 6);

pub const H1: Square = Square::from_rank_file(0, 7);
pub const H2: Square = Square::from_rank_file(1, 7);
pub const H3: Square = Square::from_rank_file(2, 7);
pub const H4: Square = Square::from_rank_file(3, 7);
pub const H5: Square = Square::from_rank_file(4, 7);
pub const H6: Square = Square::from_rank_file(5, 7);
pub const H7: Square = Square::from_rank_file(6, 7);
pub const H8: Square = Square::from_rank_file(7, 7);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::a1("a1", 0, 0)]
    #[case::h1("a8", 7, 0)]
    #[case::h8("h8", 7, 7)]
    #[case::a8("h1", 0, 7)]
    fn test_square_from_algebraic(
        #[case] algebraic: &str,
        #[case] rank: u8,
        #[case] file: u8,
    ) -> Result<(), String> {
        let square = Square::from_algebraic(algebraic)
            .ok_or("Failed to convert algebraic notation to Square")?;
        assert_eq!(square.get_rank(), rank);
        assert_eq!(square.get_file(), file);

        Ok(())
    }

    #[rstest]
    #[case::a1("a1", 1 << 0)]
    #[case::h1("a8", 1 << 56)]
    #[case::h8("h8", 1 << 63)]
    #[case::a8("h1", 1 << 7)]
    fn test_get_mask(#[case] algebraic: &str, #[case] mask: u64) {
        let square = Square::from_algebraic(algebraic).unwrap();
        assert_eq!(square.get_mask(), mask);
    }

    #[test]
    fn test_constants() {
        assert_eq!(A1, Square::from_algebraic("a1").unwrap());
        assert_eq!(A8, Square::from_algebraic("a8").unwrap());
        assert_eq!(B1, Square::from_algebraic("b1").unwrap());
        assert_eq!(B8, Square::from_algebraic("b8").unwrap());
        assert_eq!(C1, Square::from_algebraic("c1").unwrap());
        assert_eq!(C8, Square::from_algebraic("c8").unwrap());
        assert_eq!(D1, Square::from_algebraic("d1").unwrap());
        assert_eq!(D8, Square::from_algebraic("d8").unwrap());
        assert_eq!(E1, Square::from_algebraic("e1").unwrap());
        assert_eq!(E8, Square::from_algebraic("e8").unwrap());
        assert_eq!(F1, Square::from_algebraic("f1").unwrap());
        assert_eq!(F8, Square::from_algebraic("f8").unwrap());
        assert_eq!(G1, Square::from_algebraic("g1").unwrap());
        assert_eq!(G8, Square::from_algebraic("g8").unwrap());
        assert_eq!(H1, Square::from_algebraic("h1").unwrap());
        assert_eq!(H8, Square::from_algebraic("h8").unwrap());
    }
}
