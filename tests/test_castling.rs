use anyhow::Result;
use googletest::prelude::*;

use fchess::{Board, Move};

#[test]
fn test_castling_capture_black_long_queen_revokes_rights() -> Result<()> {
    let board = Board::from_fen("r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1")?
        .apply(&Move::from_full_algebraic("b7a8")?)?;

    assert_that!(board.get_castling_black_long(), eq(false));

    Ok(())
}

#[test]
fn test_castling_capture_black_short_queen_revokes_rights() -> Result<()> {
    let board = Board::from_fen("4k2r/6P1/8/8/8/8/8/4K3 w k - 0 1")?
        .apply(&Move::from_full_algebraic("g7h8")?)?;

    assert_that!(board.get_castling_black_short(), eq(false));

    Ok(())
}

#[test]
fn test_castling_capture_white_long_queen_revokes_rights() -> Result<()> {
    let board = Board::from_fen("4k3/8/8/8/8/8/1p6/R3K3 b Q - 0 1")?
        .apply(&Move::from_full_algebraic("b2a1")?)?;

    assert_that!(board.get_castling_white_long(), eq(false));

    Ok(())
}

#[test]
fn test_castling_capture_white_short_queen_revokes_rights() -> Result<()> {
    let board = Board::from_fen("4k3/8/8/8/8/8/6p1/4K2R b K - 0 1")?
        .apply(&Move::from_full_algebraic("g2h1")?)?;

    assert_that!(board.get_castling_white_short(), eq(false));

    Ok(())
}
