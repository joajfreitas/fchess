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
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Square {
    square: u8,
}

impl Square {
    pub fn from_rank_file(rank: u8, file: u8) -> Square {
        assert!(rank < 8, "Rank must be between 0 and 7");
        assert!(file < 8, "File must be between 0 and 7");
        Square {
            square: (rank << 3) + file,
        }
    }

    pub fn from_index(square: u8) -> Square {
        Square { square }
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

    pub fn get_index(&self) -> u8 {
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

#[cfg(test)]
mod tests {
    use super::Square;
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
}
