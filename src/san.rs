use anyhow::{anyhow, Result};
use regex::{Match, Regex};

use crate::board::Board;
use crate::move_generator::MoveGenerator;
use crate::moves::{Move, Scope};
use crate::piece::ColoredPieceType;
use crate::piece::PieceType;
use crate::side::Side;
use crate::square::Square;

fn san_match_type(piece_type: ColoredPieceType, scope: Scope) -> bool {
    matches!(
        (piece_type, scope),
        (_, Scope::All)
            | (ColoredPieceType::WhitePawn, Scope::WhitePawn)
            | (ColoredPieceType::BlackPawn, Scope::BlackPawn)
            | (ColoredPieceType::WhiteRook, Scope::WhiteRook)
            | (ColoredPieceType::BlackRook, Scope::BlackRook)
            | (ColoredPieceType::WhiteKnight, Scope::WhiteKnight)
            | (ColoredPieceType::BlackKnight, Scope::BlackKnight)
            | (ColoredPieceType::WhiteBishop, Scope::WhiteBishop)
            | (ColoredPieceType::BlackBishop, Scope::BlackBishop)
            | (ColoredPieceType::WhiteQueen, Scope::WhiteQueen)
            | (ColoredPieceType::BlackQueen, Scope::BlackQueen)
            | (ColoredPieceType::WhiteKing, Scope::WhiteKing)
            | (ColoredPieceType::BlackKing, Scope::BlackKing)
    )
}
fn from_san_queen_side_castle(board: &Board) -> Result<Move> {
    if board.get_turn() == Side::White {
        Ok(Move::new(
            Square::from_algebraic("e1").unwrap(),
            Square::from_algebraic("c1").unwrap(),
        ))
    } else {
        Ok(Move::new(
            Square::from_algebraic("e8").unwrap(),
            Square::from_algebraic("c8").unwrap(),
        ))
    }
}

fn from_san_king_side_castle(board: &Board) -> Result<Move> {
    if board.get_turn() == Side::White {
        Ok(Move::new(
            Square::from_algebraic("e1").unwrap(),
            Square::from_algebraic("g1").unwrap(),
        ))
    } else {
        Ok(Move::new(
            Square::from_algebraic("e8").unwrap(),
            Square::from_algebraic("g8").unwrap(),
        ))
    }
}

pub fn read_san(algebra: &str, board: &Board) -> Result<Move> {
    if algebra == "O-O-O" {
        return from_san_queen_side_castle(board);
    } else if algebra == "O-O" {
        return from_san_king_side_castle(board);
    }

    fn set_empty_string_to_none(m: Match) -> Option<Match> {
        if m.as_str() == "" {
            None
        } else {
            Some(m)
        }
    }

    let handle_piece_type = |m: Match| -> ColoredPieceType {
        let piece_type =
            ColoredPieceType::from_string(&m.as_str().chars().next().unwrap()).unwrap();
        if board.get_turn() == Side::Black {
            !piece_type
        } else {
            piece_type
        }
    };

    let handle_rank = |rank: Match| rank.as_str().chars().next().unwrap() as u8 - b'a';
    let handle_file = |file: Match| file.as_str().chars().next().unwrap() as u8 - b'1';

    let re = Regex::new(r"([BNRQK]?)([a-h]?)([1-8]?)x?([a-h])([1-8])=?([BNRQK]?)").unwrap();
    let captures = re.captures(algebra).unwrap();
    let scope = captures
        .get(1)
        .and_then(set_empty_string_to_none)
        .map(handle_piece_type)
        .map(Scope::from)
        .unwrap_or(Scope::All);
    let src_file = captures
        .get(2)
        .and_then(set_empty_string_to_none)
        .map(handle_rank);
    let src_rank = captures
        .get(3)
        .and_then(set_empty_string_to_none)
        .map(handle_file);
    let dst_rank = captures
        .get(4)
        .and_then(set_empty_string_to_none)
        .map(handle_rank);
    let dst_file = captures
        .get(5)
        .and_then(set_empty_string_to_none)
        .map(handle_file);
    let promotion = captures
        .get(6)
        .and_then(set_empty_string_to_none)
        .map(handle_piece_type);

    let dst = Square::from_rank_file(dst_file.unwrap(), dst_rank.unwrap());
    let move_generator = MoveGenerator::new(); // TODO: should not do this here if expensive
    let mut matches: Vec<(Move, ColoredPieceType)> = Vec::new();
    let moves = move_generator.generate_moves(board);
    for moveset in moves {
        for mov in moveset.into_iter() {
            let piece_type = board.piece_at(mov.get_src()).unwrap();
            if (src_rank.is_none() || Some(mov.get_src().get_rank()) == src_rank)
                && (src_file.is_none() || Some(mov.get_src().get_file()) == src_file)
                && mov.get_dst() == dst
                && san_match_type(piece_type, scope)
            {
                matches.push((mov, piece_type));
            }
        }
    }

    matches.sort_by_key(|(_, piece_type)| {
        if *piece_type == ColoredPieceType::WhitePawn || *piece_type == ColoredPieceType::BlackPawn
        {
            0
        } else {
            1
        }
    });
    let mut resulting_move = matches.into_iter().next().map(|(mov, _)| mov);
    resulting_move
        .as_mut()
        .ok_or(anyhow!(
            "Couldn't find a move that matches the specified SAN"
        ))?
        .set_promotion(promotion);
    resulting_move.ok_or(anyhow!("No move found"))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use rstest::*;

    use crate::board_builder::BoardBuilder;

    use super::read_san;
    use super::Board;

    // SAN

    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │ ♖ │ ♘ │ ♗ │ ♕ │ ♔ │ ♗ │ ♘ │ ♖ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │ ♙ │ ♙ │ ♙ │   │ ♙ │ ♙ │ ♙ │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │ ♙ │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │ ♟︎ │   │   │ ♟︎ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │ ♟︎ │ ♟︎ │   │ ♟︎ │ ♟︎ │ ♟︎ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │   │   │ ♜ │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[fixture]
    fn san_white_to_move() -> Board {
        BoardBuilder::new()
            .with_fen("rnbqkbnr/ppp1ppp1/3p4/8/8/1P2P3/2PP1PPP/RNBQK2R w KQkq - 0 3")
            .unwrap()
            .build()
    }


    #[rstest]
    #[case("d3", "d2d3")]  // Pawn move
    #[case("Ra2", "a1a2")] // Rook move
    #[case("Nc3", "b1c3")] // Knight move
    #[case("Ba3", "c1a3")] // Bishop move
    #[case("Qg4", "d1g4")] // Queen move
    #[case("Ke2", "e1e2")] // King move
    #[case("O-O", "e1g1")] // King side castling
    fn test_san_white_move(
        san_white_to_move: Board,
        #[case] san: &str,
        #[case] expected: &str,
    ) -> Result<()> {
        let mov = read_san(san, &san_white_to_move).unwrap();

        assert_eq!(mov.to_algebraic(), expected);

        Ok(())
    }

    //   ┌───┬───┬───┬───┬───┬───┬───┬───┐
    // 8 │ ♖ │   │   │   │ ♔ │ ♗ │ ♘ │ ♖ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 7 │ ♙ │ ♙ │ ♙ │ ♕ │ ♙ │ ♙ │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 6 │   │   │   │ ♙ │   │   │ ♙ │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 5 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 4 │   │   │   │   │   │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 3 │   │ ♟︎ │   │   │ ♟︎ │   │   │   │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 2 │   │   │ ♟︎ │ ♟︎ │   │ ♟︎ │ ♟︎ │ ♟︎ │
    //   ├───┼───┼───┼───┼───┼───┼───┼───┤
    // 1 │ ♜ │ ♞ │ ♝ │ ♛ │ ♚ │   │   │ ♜ │
    //   └───┴───┴───┴───┴───┴───┴───┴───┘
    //     a   b   c   d   e   f   g   h
    #[fixture]
    fn san_black_to_move() -> Board {
        BoardBuilder::new()
            .with_fen("r111kbnr/pppqpp2/3p2p1/8/8/1P2P3/2PP1PPP/RNBQK2R b KQkq - 0 3")
            .unwrap()
            .build()
    }

    #[rstest]
    #[case("e6", "e7e6")]  // Pawn move
    #[case("Rh7", "h8h7")] // Rook move
    #[case("Nf6", "g8f6")] // Knight move
    #[case("Bh6", "f8h6")] // Bishop move
    #[case("Qb5", "d7b5")] // Queen move
    #[case("Kd8", "e8d8")] // King move
    #[case("O-O-O", "e8c8")] // King side castling
    fn test_san_black_move(
        san_black_to_move: Board,
        #[case] san: &str,
        #[case] expected: &str,
    ) -> Result<()> {
        let mov = read_san(san, &san_black_to_move)?;

        assert_eq!(mov.to_algebraic(), expected);

        Ok(())
    }
}
