use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use fchess::Board;
use fchess::Move;

#[derive(Serialize, Deserialize, Debug)]
struct TestCase {
    id: String,
    description: String,
    start_fen: String,
    expected_fen: String,
    san: String,
    lan: String,
}

struct SuitResult {
    total_tests: u32,
    failed_tests: u32,
    successful_tests: u32,
    tests: Vec<TestResult>,
}

struct TestResult {
    id: String,
    starting_board: Board,
    expected_board: Board,
    result_board: Board,
    san: String,
    lan: String,
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
        id: &str,
        starting_board: &Board,
        expected_board: &Board,
        result_board: &Board,
        san: &str,
        lan: &str,
        mov: Move,
    ) -> TestResult {
        TestResult {
            id: id.to_string(),
            starting_board: starting_board.clone(),
            expected_board: expected_board.clone(),
            result_board: result_board.clone(),
            san: san.to_string(),
            lan: lan.to_string(),
            mov,
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
    let testsuit: Vec<TestCase> = serde_yaml::from_str(&contents).unwrap();

    let mut testsuit_results = SuitResult::new();

    for testcase in testsuit {
        let board = Board::from_fen(&testcase.start_fen);
        let mov = Move::from_algebraic(&testcase.lan);
        if mov.is_none() {
            println!("Failed to parse move {}", &testcase.lan);
            println!("{}", board);
            continue;
        }
        let mov = mov.unwrap();
        let resulting_board = board.apply(&mov.clone());
        if let Some(resulting_board) = resulting_board {
            let resulting_board = resulting_board;
            let expected_board = Board::from_fen(&testcase.expected_fen);
            let test_result = TestResult::new(
                &testcase.id,
                &board,
                &expected_board,
                &resulting_board,
                &testcase.san,
                &testcase.lan,
                mov,
            );
            testsuit_results.push_test(test_result);
        } else {
            println!("Failed to apply move to board {}", mov);
        }
    }

    for test_case in testsuit_results.tests {
        if !test_case.result {
            println!("====================================");
            println!("{}", test_case.id);
            println!("{}", test_case.starting_board);
            println!("{:?}", test_case.starting_board);
            println!("{}", test_case.san);
            println!("{}", test_case.lan);
            println!("{:?}", test_case.mov);
            println!("expected:\n{}", test_case.expected_board);
            println!("expected:\n{:?}", test_case.expected_board);
            println!("resulting:\n{}", test_case.result_board);
            println!("resulting:\n{:?}", test_case.result_board);
        }
    }
    println!(
        "{}/{}",
        testsuit_results.successful_tests, testsuit_results.total_tests
    );
    std::process::exit(testsuit_results.failed_tests as i32);
}
