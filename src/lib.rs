/*!
This crate provides a library for chess board representation, move generation and an engine.
*/
extern crate num;
#[macro_use]
extern crate num_derive;

pub use crate::board::Board;
pub use crate::book::Book;
pub use crate::move_generator::MoveGenerator;
pub use crate::moves::Move;
pub use crate::moveset::MoveSet;
pub use crate::side::Side;
pub use crate::solver::Solver;
pub use crate::square::Square;

mod bitboard;
mod bitwise;
mod board;
mod book;
mod common;
mod dumb7fill;
mod move_generator;
mod moves;
mod moveset;
mod piece;
mod side;
mod solver;
mod square;

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::board::Board;
    use super::move_generator::MoveGenerator;
    use super::moves::Move;
    use super::moveset::MoveSet;
    use super::piece::PieceType;
    use super::square::Square;

    fn create_mov_from_coords(moves: Vec<&str>) -> u64 {
        let mut aux: u64 = 0;
        for mov in moves {
            aux |= 1 << Square::from_algebraic(mov).unwrap().get_index()
        }

        aux
    }

    #[rstest]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♖ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/3r4/8/8/8/8/8 w KQkq - 2 3",
           ("d7", PieceType::BlackRook),
            vec!["d8", "d6", "d5", "d4", "d3", "d2", "d1", "a7", "b7", "c7", "e7", "f7", "g7", "h7"]
                )]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │ ♜ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♖ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │ ♕ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("3R4/3r4/8/8/3q4/8/8 b KQkq - 2 3",
           ("d7", PieceType::BlackRook),
                    vec![
                    "d6", "d5", "d8", "a7", "b7", "c7", "e7", "f7", "g7", "h7"
                ])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♗ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/3b4/8/8/8/8/8 b KQkq - 2 3",
           ("d7", PieceType::BlackBishop), vec!["c8", "e6", "f5", "g4", "h3", "e8", "c6", "b5", "a4"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♗ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │ ♝ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/3b4/4B3/8/8/8/8 b KQkq - 2 3",
           ("d7", PieceType::BlackBishop), vec!["c8", "e6", "e8", "c6", "b5", "a4"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │ ♗ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │ ♖ │   │ ♝ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/3b4/2r1B3/8/8/8/8 b KQkq - 2 3",
           ("d7", PieceType::BlackBishop), vec!["c8", "e6", "e8"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │ ♘ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/3n4/8/8/8 b KQkq - 2 3",
           ("d5", PieceType::BlackKnight), vec!["b6", "c7", "e7", "f6", "f4", "e3", "c3", "b4"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │ ♘ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │ ♞ │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/3n4/1N6/8/8 b KQkq - 2 3",
           ("d5", PieceType::BlackKnight), vec!["b6", "c7", "e7", "f6", "f4", "e3", "c3", "b4"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │ ♘ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │ ♗ │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/3n4/1b6/8/8 b KQkq - 2 3",
           ("d5", PieceType::BlackKnight), vec!["b6", "c7", "e7", "f6", "f4", "e3", "c3"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │ ♟︎ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/4P3/8/8/8 b KQkq - 2 3",
           ("e5", PieceType::WhitePawn), vec!["e6"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │ ♞ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │ ♟︎ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/3N4/3P4/8/8/8"
           , ("d4", PieceType::WhitePawn), vec![])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │ ♘ │ ♘ │ ♘ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │ ♟︎ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/2nnn3/3P4/8/8/8"
           , ("d4", PieceType::WhitePawn), vec!["c5", "e5"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │ ♘ │ ♘ │   │   │   │   │ ♘ │ ♘ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │ ♟︎ │   │   │   │   │   │   │ ♟︎ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("N7/8/8/8/8/8/8/8"
           , ("a8", PieceType::WhiteKnight), vec!["b6", "c7"])]
    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │ ♘ │ ♘ │   │   │   │   │ ♘ │ ♘ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │ ♟︎ │   │   │   │   │   │   │ ♟︎ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │   │   │   │   │   │   │   │   │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[case("8/8/8/nn4nn/P6P/8/8/8", ("a4", PieceType::WhitePawn), vec!["b5"])]
    fn test_generate_move_for_piece(
        #[case] initial_fen: &str,
        #[case] source_piece: (&str, PieceType),
        #[case] generated_moves: Vec<&str>,
    ) {
        let unit = Board::from_fen(initial_fen);
        let move_generator = MoveGenerator::new();
        let source_piece_square = Square::from_algebraic(source_piece.0).unwrap();
        assert_eq!(
            move_generator
                .generate_moves_for_piece(&unit, source_piece_square)
                .unwrap(),
            MoveSet::new(
                source_piece_square,
                source_piece.1,
                create_mov_from_coords(generated_moves)
            )
        )
    }

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
        // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
        //   └───┴───┴───┴───┴───┴───┴───┴───┘
        //     a   b   c   d   e   f   g   h  

        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "e2e4",
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
    )]
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
    #[case(
        //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
        // 8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │   │   │ ♖ │
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
        // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
        //   └───┴───┴───┴───┴───┴───┴───┴───┘
        //     a   b   c   d   e   f   g   h  

        "rnbqk2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "e8g8",
        "rnbq1rk1/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQ - 1 1"
    )]
    #[case("8/3P4/8/8/8/8/3p4/8", "d7d8Q", "3Q4/8/8/8/8/8/3p4/8 b")]
    fn test_apply_move(
        #[case] initial_fen: &str,
        #[case] algebraic_move: &str,
        #[case] resulting_fen: &str,
    ) {
        let unit = Board::from_fen(initial_fen);
        let unit = unit
            .apply(Move::from_full_algebraic(algebraic_move).unwrap())
            .unwrap();

        assert_eq!(unit, Board::from_fen(resulting_fen))
    }

    #[test]
    fn test_enpassant_move() {
        //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
        // 8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 7 │ ♙ │ ♙ │ ♙ │   │ ♙ │ ♙ │ ♙ │ ♙ │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 6 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 5 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 4 │   │   │   │ ♙ │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 3 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 2 │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
        //   └───┴───┴───┴───┴───┴───┴───┴───┘
        //     a   b   c   d   e   f   g   h

        let board = Board::from_fen("rnbqkbnr/ppp1pppp/8/8/3p4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        // apply enpassant move
        let result = board
            .apply(Move::from_full_algebraic("e2e4").unwrap())
            .unwrap();

        // check enpassant target square is set
        assert_eq!(
            result,
            Board::from_fen("rnbqkbnr/ppp1pppp/8/8/3pP3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1")
        );

        // apply capture during enpassant
        let result = result
            .apply(Move::from_full_algebraic("d4e3").unwrap())
            .unwrap();

        //  ┌───┬───┬───┬───┬───┬───┬───┬───┐
        //8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //7 │ ♙ │ ♙ │ ♙ │   │ ♙ │ ♙ │ ♙ │ ♙ │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //6 │   │   │   │   │   │   │   │   │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //5 │   │   │   │   │   │   │   │   │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //4 │   │   │   │   │   │   │   │   │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //3 │   │   │   │   │ ♙ │   │   │   │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //2 │ ♟︎ │ ♟︎ │ ♟︎ │ ♟︎ │   │ ♟︎ │ ♟︎ │ ♟︎ │
        //  ├───┼───┼───┼───┼───┼───┼───┼───┤
        //1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │ ♝ │ ♞ │ ♜ │
        //  └───┴───┴───┴───┴───┴───┴───┴───┘
        //    a   b   c   d   e   f   g   h

        // check that pawn is enpassant captured
        //
        println!("{}", result);
        assert_eq!(
            result,
            Board::from_fen("rnbqkbnr/ppp1pppp/8/8/8/4p3/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
        );
    }
}
