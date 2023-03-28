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

struct SuitResult {
    total_tests: u32,
    failed_tests: u32,
    successful_tests: u32,
    tests: Vec<TestResult>,
}

struct TestResult {
    starting_board: Board,
    expected_board: Board,
    result_board: Board,
    mov: Move,
    result: bool,
}

impl SuitResult {
    fn new() -> SuitResult {
        SuitResult {
            total_tests: 0,
            failed_tests: 0,
            successful_tests: 0,
            tests: Vec::new(),
        }
    }

    fn push_test(&mut self, test_result: TestResult) {
        self.total_tests += 1;
        if test_result.result {
            self.successful_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        self.tests.push(test_result)
    }
}

impl TestResult {
    pub fn new(
        starting_board: &Board,
        expected_board: &Board,
        result_board: &Board,
        mov: Move,
    ) -> TestResult {
        TestResult {
            starting_board: starting_board.clone(),
            expected_board: expected_board.clone(),
            result_board: result_board.clone(),
            mov: mov,
            result: TestResult::check(expected_board, result_board),
        }
    }

    pub fn check(expected_board: &Board, result_board: &Board) -> bool {
        expected_board == result_board
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");
    let test_suit: TestSuit = serde_json::from_str(&contents)?;

    let mut testsuit_results = SuitResult::new();

    for testcase in test_suit.testcases {
        let start_board = Board::from_fen(&testcase.start.fen);
        for expected in testcase.expected {
            let mov = Move::from_san(&expected.mov, &start_board);
            match mov {
                Some(mov) => {
                    let resulting_board = start_board.apply(mov.clone());
                    if resulting_board.is_none() {
                        println!("Failed to apply move to board {}", mov);
                    } else {
                        let resulting_board = resulting_board.unwrap();
                        let expected_board = Board::from_fen(&expected.fen);
                        let test_result =
                            TestResult::new(&start_board, &expected_board, &resulting_board, mov);
                        testsuit_results.push_test(test_result);
                    }
                }
                None => {
                    println!("Failed to parse move {}", &expected.mov);
                    println!("{}", start_board);
                    continue;
                }
            };
        }
    }

    for test_case in testsuit_results.tests {
        if !test_case.result {
            println!("====================================");
            println!("{}", test_case.starting_board);
            println!("{:?}", test_case.mov);
            println!("expected:\n{}", test_case.expected_board);
            println!("resulting:\n{}", test_case.result_board);
        }
    }
    println!(
        "{}/{}",
        testsuit_results.successful_tests, testsuit_results.total_tests
    );
    std::process::exit(testsuit_results.failed_tests as i32);
}
