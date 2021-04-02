extern crate num;
#[macro_use]
extern crate num_derive;

pub mod bitboard;
pub mod common;
pub mod moves;
pub mod dumb7fill;


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::moves::{Board, Scope, Piece};

    #[test]
    fn test_rook_move() {
        let board = Board::read_fen("8/3r4/8/8/8/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackRook {
                assert_eq!(piece.mov.count_ones(), 14);
            }
        }
    }

    #[test]
    fn test_rook_move_with_enemy() {
        let board = Board::read_fen("3R4/3r2R1/8/8/3R4/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackRook {
                assert_eq!(piece.mov.count_ones(), 10);
            }
        }
    }

    #[test]
    fn test_rook_move_with_friend() {
        let board = Board::read_fen("3R4/3r4/8/8/3q4/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackRook {
                assert_eq!(piece.mov.count_ones(), 10);
            }
        }
    }

    #[test]
    fn test_bishop_move() {
        let board = Board::read_fen("8/3b4/8/8/8/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 9);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_enemy() {
        let board = Board::read_fen("8/3b4/4B3/8/8/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 6);
            }
        }
    }

    #[test]
    fn test_bishop_move_with_friend() {
        let board = Board::read_fen("8/3b4/2r1B3/8/8/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackBishop {
                assert_eq!(piece.mov.count_ones(), 3);
            }
        }
    }


    #[test]
    fn test_knight_move() {
        let board = Board::read_fen("8/8/8/3n4/8/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_enemy() {
        let board = Board::read_fen("8/8/8/3n4/1N6/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 8);
            }
        }
    }

    #[test]
    fn test_knight_move_with_friend() {
        let board = Board::read_fen("8/8/8/3n4/1b6/8/8 w KQkq - 2 3".to_string());
        for piece in board.generate_moves(&Scope::Black) {
            if piece.piece == Piece::BlackKnight {
                assert_eq!(piece.mov.count_ones(), 7);
            }
        }
    }

    #[test]
    fn test_pawn_move() {
        let mut board = Board::new();
        board.set(&Piece::WhitePawn, (3,3));
        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == Piece::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 1);
            }
        }
    }
    #[test]
    fn test_pawn_move_with_friend() {
        let mut board = Board::new();
        board.set(&Piece::WhitePawn, (3,3));
        board.set(&Piece::WhiteKnight, (4,3));
        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == Piece::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 0);
            }
        }
    }

    #[test]
    fn test_pawn_move_with_enemy() {
        let mut board = Board::new();
        board.set(&Piece::WhitePawn, (3,3));
        board.set(&Piece::BlackKnight, (4,3));
        board.set(&Piece::BlackKnight, (4,4));
        board.set(&Piece::BlackKnight, (4,2));
        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == Piece::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 2);
            }
        }
    }

    #[test]
    fn test_pawn_move_with_enemy_edges() {
        let mut board = Board::new();
        board.set(&Piece::WhitePawn, (3,7));
        board.set(&Piece::WhitePawn, (3,0));
        board.set(&Piece::BlackKnight, (4,6));
        board.set(&Piece::BlackKnight, (4,7));
        board.set(&Piece::BlackKnight, (4,0));
        board.set(&Piece::BlackKnight, (4,1));
        for piece in board.generate_moves(&Scope::White) {
            if piece.piece == Piece::WhitePawn {
                assert_eq!(piece.mov.count_ones(), 1);
            }
        }
    }
}
