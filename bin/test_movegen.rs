#![feature(iter_intersperse)]

use std::collections::HashSet;
use std::env;
use std::fs;

use serde::{Deserialize, Serialize};

use anyhow::Result;
use fchess::{Board, Move, MoveGenerator, MoveSet, Square};

mod test_common;
use crate::test_common::{TestResult, TestSuit};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCase {
    id: String,
    description: String,
    fen: String,
    square: Option<String>,
    moves: Vec<String>,
}

#[derive(Clone)]
struct MovegenTestResult {
    testcase: TestCase,
    resulting_moves: Vec<Move>,
    result: bool,
}

impl MovegenTestResult {
    fn new(testcase: TestCase, resulting_moves: Vec<Move>, result: bool) -> MovegenTestResult {
        MovegenTestResult {
            testcase,
            resulting_moves,
            result,
        }
    }
}

impl TestResult for MovegenTestResult {
    fn to_string(&self) -> String {
        let expected_set: HashSet<String> =
            self.testcase.moves.iter().map(|x| x.to_string()).collect();
        let resulting_set: HashSet<String> = self
            .resulting_moves
            .iter()
            .map(|mov| mov.to_algebraic())
            .collect();

        let d1: HashSet<String> = expected_set
            .difference(&resulting_set)
            .map(|x| x.to_string())
            .collect();
        let d2: HashSet<String> = resulting_set
            .difference(&expected_set)
            .map(|x| x.to_string())
            .collect();
        let diffs = d2
            .union(&d1)
            .map(|x| x.to_string())
            .intersperse(",".to_string())
            .collect::<String>();

        format!(
            "id:{}\n{}\nexpected: {}\nresult: {}\ndiff: {}",
            self.testcase.id,
            Board::from_fen(&self.testcase.fen).expect("Invalid FEN"),
            self.testcase
                .moves
                .clone()
                .into_iter()
                .intersperse(",".to_string())
                .collect::<String>(),
            &self
                .resulting_moves
                .iter()
                .map(|mov| mov.to_algebraic())
                .intersperse(",".to_string())
                .collect::<String>(),
            diffs
        )
    }

    fn result(&self) -> bool {
        self.result
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");
    let testsuit: Vec<TestCase> = serde_yaml::from_str(&contents).unwrap();

    let mut testsuit_result: TestSuit<MovegenTestResult> = TestSuit::new();

    let move_generator = MoveGenerator::new();
    for testcase in testsuit {
        let board = Board::from_fen(&testcase.fen)?;
        let movesets: Vec<MoveSet> = if testcase.square.is_none() {
            move_generator.generate_moves(&board)
        } else {
            vec![move_generator
                .generate_moves_for_piece(
                    &board,
                    Square::from_algebraic(&testcase.clone().square.unwrap()).unwrap(),
                )
                .unwrap()]
        };
        let mut moves: Vec<Move> = movesets
            .iter()
            .flat_map(|moveset| moveset.into_iter())
            .collect::<Vec<Move>>();
        let mut expected_moves = testcase
            .clone()
            .moves
            .iter()
            .map(|mov| Move::from_full_algebraic(mov).unwrap())
            .collect::<Vec<Move>>();

        moves.sort();
        expected_moves.sort();

        let test_result = MovegenTestResult::new(testcase, moves.clone(), moves == expected_moves);
        testsuit_result.push_test(test_result);
    }

    testsuit_result.finalize();
    Ok(())
}
