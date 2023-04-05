use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use fchess::board::Board;
use fchess::moves::Move;

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

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");
    let test_suit: TestSuit = serde_json::from_str(&contents)?;

    for testcase in test_suit.testcases {
        let start_board = Board::from_fen(&testcase.start.fen);
        for expected in testcase.expected {
            let mov = match Move::from_san(&expected.mov, &start_board) {
                Some(mov) => mov.to_algebraic(),
                None => {
                    println!("Failed");
                    expected.mov
                }
            };

            println!("{}", mov);
        }
    }

    Ok(())
}
