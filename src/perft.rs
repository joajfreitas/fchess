use crate::board::Board;
use crate::move_generator::MoveGenerator;
use crate::moves::Move;

fn _perft(board: &Board, search_depth: u32) -> Option<u32> {
    if search_depth == 0 {
        return Some(1);
    }

    let move_gen = MoveGenerator::new();

    let mut stack = Vec::<(Board, u32)>::new();
    let mut acc: u32 = 0;

    stack.push((board.clone(), 0));

    while !stack.is_empty() {
        let (board, depth) = stack.pop().unwrap();

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

    Some(acc)
}

pub fn perft(board: &Board, search_depth: u32) -> Option<(u32, Vec<(Move, u32)>)> {
    if search_depth == 0 {
        return None;
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
        .collect::<Option<Vec<(Move, u32)>>>();

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
        expect_that!(perft(&board, 1), some((points_to(eq(20)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_2() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 2), some((points_to(eq(400)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_3() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 3), some((points_to(eq(8902)), anything())));
    }

    #[gtest]
    fn test_perft_starting_position_depth_4() {
        let board = BoardBuilder::new().with_basic_board().build();
        expect_that!(perft(&board, 4), some((points_to(eq(197281)), anything())));
    }
}
