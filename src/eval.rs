use bit_iter::BitIter;

use crate::board::Board;
use crate::piece::{COLORED_PIECE_TYPES, ColoredPieceType, PieceType};
use crate::side::Side;

#[derive(Debug, Clone)]
pub struct Eval {
    piece_value: [i32; 14],
    pawn_table: [i32; 64],
    knight_table: [i32; 64],
    bishop_table: [i32; 64],
    rook_table: [i32; 64],
    queen_table: [i32; 64],
    king_middle_table: [i32; 64],
    //king_end_table: [i32; 64],
}

impl Eval {
    pub fn new() -> Self {
        Self {
            piece_value: [
                100, 500, 320, 330, 900, 20000, -100, -500, -320, -330, -900, -20000, 0, 0,
            ],
            pawn_table: [
                0, 0, 0, 0, 0, 0, 0, 0, //
                50, 50, 50, 50, 50, 50, 50, 50, //
                10, 10, 20, 30, 30, 20, 10, 10, //
                5, 5, 10, 25, 25, 10, 5, 5, //
                0, 0, 0, 20, 20, 0, 0, 0, //
                5, -5, -10, 0, 0, -10, -5, 5, //
                5, 10, 10, -20, -20, 10, 10, 5, //
                0, 0, 0, 0, 0, 0, 0, 0, //
            ],
            knight_table: [
                -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10,
                15, 15, 10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30,
                -30, 5, 10, 15, 15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30,
                -30, -30, -40, -50,
            ],
            bishop_table: [
                -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10,
                10, 5, 0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10,
                10, 10, 10, 10, 10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10,
                -10, -10, -20,
            ],
            rook_table: [
                0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5,
                0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
                0, 0, 0, 0, -5, 0, 0, 0, 5, 5, 0, 0, 0,
            ],
            queen_table: [
                -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5,
                5, 0, -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5,
                0, -10, -10, 0, 5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
            ],
            king_middle_table: [
                -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30,
                -20, -30, -30, -40, -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20,
                20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20,
            ],
            //king_end_table: [
            //    -50, -40, -30, -20, -20, -30, -40, -50, -30, -20, -10, 0, 0, -10, -20, -30, -30,
            //    -10, 20, 30, 30, 20, -10, -30, -30, -10, 30, 40, 40, 30, -10, -30, -30, -10, 30,
            //    40, 40, 30, -10, -30, -30, -10, 20, 30, 30, 20, -10, -30, -30, -30, 0, 0, 0, 0,
            //    -30, -30, -50, -30, -30, -30, -30, -30, -30, -50,
            //],
        }
    }

    fn piece_value(&self, piece_type: &ColoredPieceType, side: &Side) -> i32 {
        self.piece_value[*piece_type as usize] * if *side == Side::White { 1 } else { -1 }
    }

    fn piece_square_table(&self, index: usize, piece_type: &ColoredPieceType, side: &Side) -> i32 {
        let index = if piece_type.is_black() {
            index
        } else {
            63 - index
        };
        let s = match (*piece_type).try_into().unwrap() {
            PieceType::Pawn => self.pawn_table[index],
            PieceType::Rook => self.rook_table[index],
            PieceType::King => self.king_middle_table[index],
            PieceType::Queen => self.queen_table[index],
            PieceType::Bishop => self.bishop_table[index],
            PieceType::Knight => self.knight_table[index],
        };
        s * (if piece_type.side() == *side { 1 } else { -1 })
    }

    pub fn eval(&self, board: &Board, side: &Side) -> i32 {
        let mut s: i32 = 0;
        for piece_type in COLORED_PIECE_TYPES {
            for i in BitIter::from(board.get_piece_mask(piece_type)) {
                s += self.piece_value(&piece_type, side)
                    + self.piece_square_table(i, &piece_type, side);
            }
        }

        s
    }
}

impl Default for Eval {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use googletest::prelude::*;

    use crate::board_builder::BoardBuilder;
    use crate::mv;

    #[gtest]
    fn test_eval_initial_position_evals_to_zero() -> Result<()> {
        let eval = Eval::new();

        assert_that!(eval.eval(&Board::from_basic_board(), &Side::White), eq(0));

        Ok(())
    }

    #[gtest]
    fn test_eval_initial_white_pawn_moves() -> Result<()> {
        let basic_board_builder = BoardBuilder::new().with_basic_board();

        let e2e4 = basic_board_builder.with_move(&mv!("e2e4")?)?.build();
        let h2h4 = basic_board_builder.with_move(&mv!("h2h4")?)?.build();

        let eval = Eval::new();
        let e2e4_eval = eval.eval(&e2e4, &Side::White);
        let h2h4_eval = eval.eval(&h2h4, &Side::White);

        assert_that!(e2e4_eval, gt(0));
        assert_that!(e2e4_eval, gt(h2h4_eval));

        Ok(())
    }
    #[gtest]
    fn test_eval_initial_black_pawn_moves() -> Result<()> {
        let basic_board_builder = BoardBuilder::new()
            .with_basic_board()
            .with_move(&mv!("e2e4")?)?;

        let e7e5 = basic_board_builder.with_move(&mv!("e7e5")?)?.build();
        let h7h5 = basic_board_builder.with_move(&mv!("h7h5")?)?.build();

        let eval = Eval::new();
        let e7e5_eval = eval.eval(&e7e5, &Side::Black);
        let h7h5_eval = eval.eval(&h7h5, &Side::Black);

        assert_that!(e7e5_eval, eq(0));
        assert_that!(e7e5_eval, gt(h7h5_eval));

        Ok(())
    }
}
