use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use fchess::board::Board;
use fchess::moves::{Move, MoveGenerator};

#[derive(Serialize, Deserialize, Debug)]
struct StartCondition {
    description: String,
    fen: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExpectedCondition {
    #[serde(alias = "move")]
    mov: String,
    fen: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestCase {
    start: StartCondition,
    expected: Vec<ExpectedCondition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TestSuit {
    description: String,
    #[serde(alias = "testCases")]
    testcases: Vec<TestCase>,
}

fn check_result(mov: &str, resulting_board: &Board, expected_board: &Board) {
    if resulting_board == expected_board {
        println!("passed")
    } else {
        println!("{}", mov);
        println!("{}", resulting_board);
        println!("{:?}", resulting_board);
        println!("{}", expected_board);
        println!("{:?}", expected_board);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");
    let test_suit: TestSuit = serde_json::from_str(&contents)?;

    for testcase in test_suit.testcases {
        let start_board = Board::from_fen(&testcase.start.fen);
        for expected in testcase.expected {
            let mov = Move::from_san(&expected.mov, &start_board);
            let result = match mov {
                Some(mov) => {
                    let resulting_board = start_board.apply(mov).unwrap();
                    let expected_board = Board::from_fen(&expected.fen);
                    //dbg!(resulting_board) == dbg!(expected_board)
                    check_result(&expected.mov, &resulting_board, &expected_board);
                }
                None => {
                    println!("Cannot decode san move");
                    continue;
                }
            };
        }
    }
    Ok(())
}
