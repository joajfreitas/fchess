/*!
This crate provides a library for chess board representation, move generation and an engine.
*/

#![feature(iter_intersperse)]

extern crate num;
#[macro_use]
extern crate num_derive;

pub use crate::board::Board;
pub use crate::book::Book;
pub use crate::epd::Epd;
pub use crate::move_generator::MoveGenerator;
pub use crate::moves::Move;
pub use crate::moveset::MoveSet;
pub use crate::side::Side;
pub use crate::solver::Solver;
pub use crate::square::Square;

mod bitboard;
mod bitwise;
mod board;
mod board_builder;
mod book;
mod common;
mod dumb7fill;
mod epd;
mod fen;
mod move_generator;
mod moves;
mod moveset;
mod piece;
mod side;
mod solver;
mod square;
mod zobrist_hash;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::board::Board;
    use super::moves::Move;

    use anyhow::Result;

    #[rstest]
    #[case(
         //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
         // 8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 7 │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │ ♙ │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 6 │   │   │   │   │   │   │   │   │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 5 │   │   │   │   │   │   │   │   │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 4 │   │   │   │   │   │   │   │   │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 3 │   │   │   │   │   │   │   │   │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 2 │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │
         //   ├───┼───┼───┼───┼───┼───┼───┼───┤
         // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │   │   │ ♜ │
         //   └───┴───┴───┴───┴───┴───┴───┴───┘
         //     a   b   c   d   e   f   g   h

        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1",
        "e1g1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQ1RK1 b kq - 1 1"
    )]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♟︎ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │ ♙ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/3P4/8/8/8/8/3p4/8", "d7d8Q", "3Q4/8/8/8/8/8/3p4/8 b")]
    fn test_apply_move(
        #[case] initial_fen: &str,
        #[case] algebraic_move: &str,
        #[case] resulting_fen: &str,
    ) -> Result<()> {
        let unit = Board::from_fen(initial_fen)?;
        let unit = unit
            .apply(&Move::from_full_algebraic(algebraic_move).unwrap())
            .unwrap();

        assert_eq!(unit, Board::from_fen(resulting_fen)?);

        Ok(())
    }
}
