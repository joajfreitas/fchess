extern crate num;
#[macro_use]
extern crate num_derive;

pub mod bitboard;
pub mod bitwise;
pub mod board;
pub mod book;
pub mod common;
pub mod dumb7fill;
pub mod moves;
pub mod piece;
pub mod side;
pub mod solver;
pub mod square;

#[cfg(test)]
mod tests {
    use super::board::Board;
    use super::moves::MoveGenerator;
    use super::piece::PieceType;
    use super::square::Square;

    fn create_mov_from_coords(moves: Vec<(u8, u8)>) -> u64 {
        let mut aux: u64 = 0;
        for (rank, file) in moves {
            aux |= 1 << (rank + (file * 8))
        }

        aux
    }

    #[test]
    fn test_rook_move() {
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

        let board = Board::from_fen("8/3r4/8/8/8/8/8 w KQkq - 2 3");

        let move_generator = MoveGenerator::new();
        println!("{:?}", board);
        let mov = move_generator
            .generate_moves_for_piece(&board, Square::from_rank_file(6, 3))
            .unwrap();
        println!("{:?}", mov);
        let moves = create_mov_from_coords(vec![
            (3, 7),
            (3, 5),
            (3, 4),
            (3, 3),
            (3, 2),
            (3, 1),
            (3, 0),
            (0, 6),
            (1, 6),
            (2, 6),
            (4, 6),
            (5, 6),
            (6, 6),
            (7, 6),
        ]);
        assert_eq!(mov.mov, moves);
    }

    #[test]
    fn test_rok_move_with_friend() {
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

        let board = Board::from_fen("3R4/3r4/8/8/3q4/8/8 w KQkq - 2 3");

        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackRook {
                assert_eq!(piece.mov.count_ones(), 10);
            }
        }
    }

    #[test]
    fn test_bishop_move() {
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

        let board = Board::from_fen("8/3b4/8/8/8/8/8 w KQkq - 2 3");

        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 9);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_enemy() {
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

        let board = Board::from_fen("8/3b4/4B3/8/8/8/8 w KQkq - 2 3");
        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 6);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_friend() {
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

        let board = Board::from_fen("8/3b4/2r1B3/8/8/8/8 w KQkq - 2 3");

        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 3);
            }
        }
    }

    #[test]
    fn test_knight_move() {
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

        let board = Board::from_fen("8/8/8/3n4/8/8/8 w KQkq - 2 3");
        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_enemy() {
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

        let board = Board::from_fen("8/8/8/3n4/1N6/8/8 w KQkq - 2 3");

        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_friend() {
        //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
        // 8 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 7 │   │   │   │   │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
        // 6 │   │   │   │   │   │   │   │   │
        // 5 │   │   │   │ ♘ │   │   │   │   │
        //   ├───┼───┼───┼───┼───┼───┼───┼───┤
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

        let board = Board::from_fen("8/8/8/3n4/1b6/8/8 w KQkq - 2 3");
        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 7);
            }
        }
    }

    #[test]
    fn test_pawn_move() {
        let mut board = Board::new();
        board.set_piece(Square::from_rank_file(3, 3), PieceType::WhitePawn);
        let move_generator = MoveGenerator::new();
        for mov in move_generator.generate_moves(&board) {
            if mov.piece == PieceType::WhitePawn {
                assert_eq!(mov.mov.count_ones(), 1);
            }
        }
    }
    #[test]
    fn test_pawn_move_with_friend() {
        let mut board = Board::new();
        board.set_piece(Square::from_rank_file(3, 3), PieceType::WhitePawn);
        board.set_piece(Square::from_rank_file(4, 3), PieceType::WhiteKnight);
        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 0);
            }
        }
    }

    //#[test]
    //fn test_pawn_move_with_enemy() {
    //    let mut board = Board::new();
    //    board.set(&PieceType::WhitePawn, Square::from_rank_file(3, 3));
    //    board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 3));
    //    board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 4));
    //    board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 2));

    //    println!("{:?}", board);
    //    let move_generator = MoveGenerator::new();

    //    for piece in move_generator.generate_moves(&board) {
    //        println!("{:?}", piece);
    //        if piece.piece == PieceType::WhitePawn {
    //            assert_eq!(piece.mov.count_ones(), 2);
    //        }
    //    }
    //}

    #[test]
    fn test_pawn_move_with_enemy_edges() {
        let mut board = Board::new();
        board.set_piece(Square::from_algebraic("h4").unwrap(), PieceType::WhitePawn);
        board.set_piece(Square::from_algebraic("a4").unwrap(), PieceType::WhitePawn);
        board.set_piece(
            Square::from_algebraic("g5").unwrap(),
            PieceType::BlackKnight,
        );
        board.set_piece(
            Square::from_algebraic("h5").unwrap(),
            PieceType::BlackKnight,
        );
        board.set_piece(
            Square::from_algebraic("a5").unwrap(),
            PieceType::BlackKnight,
        );
        board.set_piece(
            Square::from_algebraic("b5").unwrap(),
            PieceType::BlackKnight,
        );

        let move_generator = MoveGenerator::new();
        for piece in move_generator.generate_moves(&board) {
            if piece.piece == PieceType::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 1);
            }
        }
    }
}
