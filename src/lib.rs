extern crate num;
#[macro_use]
extern crate num_derive;

pub mod bitboard;
pub mod board;
pub mod book;
pub mod common;
pub mod dumb7fill;
pub mod moves;
pub mod piece;
pub mod square;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::board::Board;
    use super::moves::Scope;
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
        let board = Board::read_fen("8/3r4/8/8/8/8/8 w KQkq - 2 3");
        println!("{:?}", board);
        let mov = board
            .generate_moves_for_piece(&Scope::Black, Square::from_rank_file(6, 3))
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
    fn test_rook_move_with_friend() {
        let board = Board::read_fen("3R4/3r4/8/8/3q4/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackRook {
                assert_eq!(piece.mov.count_ones(), 10);
            }
        }
    }

    #[test]
    fn test_bishop_move() {
        let board = Board::read_fen("8/3b4/8/8/8/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 9);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_enemy() {
        let board = Board::read_fen("8/3b4/4B3/8/8/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 6);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_friend() {
        let board = Board::read_fen("8/3b4/2r1B3/8/8/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 3);
            }
        }
    }

    #[test]
    fn test_knight_move() {
        let board = Board::read_fen("8/8/8/3n4/8/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_enemy() {
        let board = Board::read_fen("8/8/8/3n4/1N6/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_friend() {
        let board = Board::read_fen("8/8/8/3n4/1b6/8/8 w KQkq - 2 3");
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == PieceType::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 7);
            }
        }
    }

    #[test]
    fn test_pawn_move() {
        let mut board = Board::new();
        board.set(&PieceType::WhitePawn, Square::from_rank_file(3, 3));
        for mov in board.generate_moves(&Scope::White) {
            if mov.piece == PieceType::WhitePawn {
                assert_eq!(mov.mov.count_ones(), 1);
            }
        }
    }
    #[test]
    fn test_pawn_move_with_friend() {
        let mut board = Board::new();
        board.set(&PieceType::WhitePawn, Square::from_rank_file(3, 3));
        board.set(&PieceType::WhiteKnight, Square::from_rank_file(4, 3));
        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == PieceType::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 0);
            }
        }
    }

    #[test]
    fn test_pawn_move_with_enemy() {
        let mut board = Board::new();
        board.set(&PieceType::WhitePawn, Square::from_rank_file(3, 3));
        board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 3));
        board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 4));
        board.set(&PieceType::BlackKnight, Square::from_rank_file(4, 2));

        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == PieceType::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 2);
            }
        }
    }

    #[test]
    fn test_pawn_move_with_enemy_edges() {
        let mut board = Board::new();
        board.set(&PieceType::WhitePawn, Square::from_algebraic("h4"));
        board.set(&PieceType::WhitePawn, Square::from_algebraic("a4"));
        board.set(&PieceType::BlackKnight, Square::from_algebraic("g5"));
        board.set(&PieceType::BlackKnight, Square::from_algebraic("h5"));
        board.set(&PieceType::BlackKnight, Square::from_algebraic("a5"));
        board.set(&PieceType::BlackKnight, Square::from_algebraic("b5"));

        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == PieceType::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 1);
            }
        }
    }
}
