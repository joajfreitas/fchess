use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use fchess::board::Board;
use fchess::moves::Move;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct StartCondition {
    description: String,
    fen: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExpectedCondition {
    #[serde(alias = "move")]
    mov: String,
    san: Option<String>,
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

#[derive(Serialize, Deserialize, Debug)]
struct Test {
    id: String,
    description: String,
    start_fen: String,
    expected_fen: String,
    san: String,
    lan: String,
}

fn main() -> Result<()> {
    let mut tests: Vec<Test> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");

    let mut test_suit: TestSuit = serde_json::from_str(&contents)?;

    for testcase in &mut test_suit.testcases {
        let start_board = Board::from_fen(&testcase.start.fen);
        for mut expected in &mut testcase.expected {
            let mov = match Move::from_san(&expected.mov, &start_board) {
                Some(mov) => {
                    tests.push(Test {
                        id: Uuid::new_v4().simple().to_string(),
                        description: testcase.start.description.clone(),
                        start_fen: testcase.start.fen.clone(),
                        expected_fen: expected.fen.clone(),
                        san: expected.mov.clone(),
                        lan: mov.to_algebraic(),
                    });
                }
                None => {}
            };
        }
    }

    println!("{}", serde_yaml::to_string(&tests).unwrap());

    Ok(())
}
