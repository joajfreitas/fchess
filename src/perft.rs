use anyhow::{Result, anyhow};

use crate::board::Board;
use crate::move_generator::MoveGenerator;
use crate::moves::Move;

fn _perft(board: &Board, search_depth: u32) -> Result<u32> {
    if search_depth == 0 {
        return Ok(1);
    }

    let move_gen = MoveGenerator::new();

    let mut stack = Vec::<(Board, u32)>::new();
    let mut acc: u32 = 0;

    stack.push((board.clone(), 0));

    while let Some((board, depth)) = stack.pop() {
        if depth == search_depth - 1 {
            acc += move_gen
                .generate_moves(&board)
                .iter()
                .map(|mov| mov.count())
                .sum::<u32>();
        } else if depth < search_depth - 1 {
            for mov in move_gen
                .generate_moves(&board)
                .iter()
                .flat_map(|mov| mov.into_iter())
            {
                stack.push((board.apply(&mov).unwrap(), depth + 1));
            }
        } else {
            panic!();
        }
    }

    Ok(acc)
}

pub fn perft(board: &Board, search_depth: u32) -> Result<(u32, Vec<(Move, u32)>)> {
    if search_depth == 0 {
        return Err(anyhow!("0 depth doesn't make sense for perft"));
    }

    let move_gen = MoveGenerator::new();
    let moves = move_gen
        .generate_moves(board)
        .iter()
        .flat_map(|mov| mov.into_iter().collect::<Vec<Move>>())
        .collect::<Vec<Move>>();

    let counts = moves
        .iter()
        .map(|mov| {
            _perft(&board.apply(mov).unwrap(), search_depth - 1).map(|count| (mov.clone(), count))
        })
        .collect::<Result<Vec<(Move, u32)>>>();

    counts.map(|counts| (counts.iter().map(|(_, count)| count).sum(), counts))
}

#[cfg(test)]
mod tests {
    use googletest::prelude::*;

    use super::*;
    use crate::board_builder::BoardBuilder;

    #[gtest]
    fn test_perft_starting_position_depth_1() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 1), ok((points_to(eq(20)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_2() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 2), ok((points_to(eq(400)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_3() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 3), ok((points_to(eq(8902)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_4() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 4), ok((points_to(eq(197281)), anything())));
    }
}
