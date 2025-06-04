use crate::board::Board;
use crate::piece::ColoredPieceType;
use crate::side::Side;
use crate::square::Square;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

use regex::Regex;
use std::collections::VecDeque;

fn read_piece_placement(fen: &str) -> Result<Board> {
    let mut board = Board::new();
    let vec = ['P', 'R', 'N', 'B', 'Q', 'K', 'p', 'r', 'n', 'b', 'q', 'k'];
    let mut chars = fen.chars().collect::<VecDeque<char>>();

    let mut rank: u8 = 0;
    let mut file: u8 = 0;

    while let Some(c) = chars.pop_front() {
        let pos = vec.iter().position(|&r| r == c);

        match (pos, c) {
            (Some(o), _) => {
                let piece_type: ColoredPieceType = num::FromPrimitive::from_usize(o).unwrap();
                board.set_piece(Square::from_rank_file(7 - rank, file), piece_type);
                file += 1;
            }
            (_, '0'..='9') => file += c.to_digit(10).unwrap() as u8,
            (_, '/') => {
                if file != 8 {
                    return Err(anyhow!("Invalid file in FEN string, expected 8 files"));
                }
                rank += 1;
                file = 0;
            }
            _ => {
                panic!(); // Regex ensures that only valid characters are present
            }
        }
    }

    if rank != 7 {
        return Err(anyhow!("Invalid rank in FEN string, expected 8 ranks"));
    }

    Ok(board)
}

pub fn read_fen(fen: &str) -> Result<Board> {
    lazy_static! {
        static ref TAIL_RE: Regex = Regex::new(r"^(?<placement>[1-8\/PRNBQKprnbqk]+)\s?(?<side>[wb])? ?(?<white_short_castling>K?)(?<white_long_castling>Q?)(?<black_short_castling>k?)(?<black_long_castling>q?)-? ?(?<en_passant>-?([a-h][1-8])?) ?(?<half_clock>\d{1,2})? ?(?<full_clock>\d{1,2})?").unwrap();
    }

    let captures = TAIL_RE.captures(fen).ok_or(anyhow!("Invalid fen"))?;

    let mut board = read_piece_placement(
        captures
            .name("placement")
            .ok_or(anyhow!("Piece placement not found in FEN string"))?
            .as_str(),
    )?;

    board.set_turn(match captures.name("side").map(|side| side.as_str()) {
        None | Some("w") => Ok(Side::White),
        Some("b") => Ok(Side::Black),
        _ => Err(anyhow!("Invalid side in FEN string")),
    }?);

    board.set_castling_white_short(
        captures
            .name("white_short_castling")
            .is_some_and(|key| key.as_str() == "K"),
    );
    board.set_castling_white_long(
        captures
            .name("white_long_castling")
            .is_some_and(|key| key.as_str() == "Q"),
    );
    board.set_castling_black_short(
        captures
            .name("black_short_castling")
            .is_some_and(|key| key.as_str() == "k"),
    );
    board.set_castling_black_long(
        captures
            .name("black_long_castling")
            .is_some_and(|key| key.as_str() == "q"),
    );

    board.set_enpassant(
        captures
            .name("en_passant")
            .and_then(|key| Square::from_algebraic(key.as_str())),
    );

    let parse_int = |key: regex::Match| key.as_str().parse::<u8>();

    board.set_half_move_clock(captures.name("half_clock").map_or(Ok(0), parse_int)?);
    board.set_full_move_clock(captures.name("full_clock").map_or(Ok(1), parse_int)?);

    Ok(board.clone())
}

#[cfg(test)]
mod tests {
    use super::read_fen;
    use crate::board::Board;
    use crate::board_builder::BoardBuilder;
    use crate::piece::ColoredPieceType;
    use crate::side::Side;
    use crate::square::Square;

    use anyhow::Result;
    use googletest::prelude::*;

    #[gtest]
    fn test_read_simple_fen() {
        assert_that!(read_fen("8/8/8/8/8/8/8/8"), ok(eq(&Board::new())));
    }

    #[test]
    fn test_read_starting_position_fen() {
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
        assert_that!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            ok(eq(&BoardBuilder::new()
                .with_piece("a1", ColoredPieceType::WhiteRook)
                .with_piece("b1", ColoredPieceType::WhiteKnight)
                .with_piece("c1", ColoredPieceType::WhiteBishop)
                .with_piece("d1", ColoredPieceType::WhiteQueen)
                .with_piece("e1", ColoredPieceType::WhiteKing)
                .with_piece("f1", ColoredPieceType::WhiteBishop)
                .with_piece("g1", ColoredPieceType::WhiteKnight)
                .with_piece("h1", ColoredPieceType::WhiteRook)
                .with_piece("a2", ColoredPieceType::WhitePawn)
                .with_piece("b2", ColoredPieceType::WhitePawn)
                .with_piece("c2", ColoredPieceType::WhitePawn)
                .with_piece("d2", ColoredPieceType::WhitePawn)
                .with_piece("e2", ColoredPieceType::WhitePawn)
                .with_piece("f2", ColoredPieceType::WhitePawn)
                .with_piece("g2", ColoredPieceType::WhitePawn)
                .with_piece("h2", ColoredPieceType::WhitePawn)
                .with_piece("a7", ColoredPieceType::BlackPawn)
                .with_piece("b7", ColoredPieceType::BlackPawn)
                .with_piece("c7", ColoredPieceType::BlackPawn)
                .with_piece("d7", ColoredPieceType::BlackPawn)
                .with_piece("e7", ColoredPieceType::BlackPawn)
                .with_piece("f7", ColoredPieceType::BlackPawn)
                .with_piece("g7", ColoredPieceType::BlackPawn)
                .with_piece("h7", ColoredPieceType::BlackPawn)
                .with_piece("a8", ColoredPieceType::BlackRook)
                .with_piece("b8", ColoredPieceType::BlackKnight)
                .with_piece("c8", ColoredPieceType::BlackBishop)
                .with_piece("d8", ColoredPieceType::BlackQueen)
                .with_piece("e8", ColoredPieceType::BlackKing)
                .with_piece("f8", ColoredPieceType::BlackBishop)
                .with_piece("g8", ColoredPieceType::BlackKnight)
                .with_piece("h8", ColoredPieceType::BlackRook)
                .with_turn(Side::White)
                .with_enpassant(None)
                .with_half_move_clock(0)
                .with_full_move_clock(1)
                .build()))
        );
    }

    #[test]
    fn test_read_fen_with_black_turn() -> Result<()> {
        assert_eq!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b")?.get_turn(),
            Side::Black
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_castling_rights_white_short() -> Result<()> {
        assert!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w K")?.get_castling_white_short()
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_castling_rights_white_long() -> Result<()> {
        assert!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Q")?.get_castling_white_long()
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_castling_rights_black_short() -> Result<()> {
        assert!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w k")?.get_castling_black_short()
        );
        Ok(())
    }

    #[test]
    fn test_read_fen_with_castling_rights_black_long() -> Result<()> {
        assert!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w q")?.get_castling_black_long()
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_enpassant() -> Result<()> {
        assert_eq!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w e3 0 1")?.get_enpassant(),
            Square::from_algebraic("e3")
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_half_move_clock() -> Result<()> {
        assert_eq!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 10 1")?
                .get_half_move_clock(),
            10
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_full_move_clock() -> Result<()> {
        assert_eq!(
            read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - 0 10")?.get_full_move_clock(),
            10
        );

        Ok(())
    }

    #[test]
    fn test_read_fen_with_invalid_piece() {
        let board = read_fen("Znbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(
            format!("{}", board.err().unwrap()),
            "Invalid fen".to_string()
        );
    }

    #[test]
    fn test_read_fen_without_full_ranks() {
        let board = read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP");
        assert_eq!(
            format!("{}", board.err().unwrap()),
            "Invalid rank in FEN string, expected 8 ranks".to_string()
        );
    }

    #[test]
    fn test_read_fen_without_full_files() {
        let board = read_fen("rnbqkbn/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        assert_eq!(
            format!("{}", board.err().unwrap()),
            "Invalid file in FEN string, expected 8 files".to_string()
        );
    }
}
