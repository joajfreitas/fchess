use std::env;
use std::fs;

mod test_common;
use crate::test_common::{TestResult, TestSuit};

use fchess::Epd;
use fchess::Move;
use fchess::Solver;

#[derive(Clone)]
struct TestCase {
    prompt: String,
    epd: Epd,
}

#[derive(Clone)]
struct BestMoveTestResult {
    testcase: TestCase,
    resulting_move: Move,
    result: bool,
}

impl BestMoveTestResult {
    fn new(testcase: TestCase, resulting_move: Move, result: bool) -> BestMoveTestResult {
        BestMoveTestResult {
            testcase,
            resulting_move,
            result,
        }
    }
}

impl TestResult for BestMoveTestResult {
    fn to_string(&self) -> String {
        format!(
            "{}\n{}\nexpected: {}, got: {}",
            self.testcase.prompt,
            self.testcase.epd.get_board(),
            self.testcase.epd.get_properties().get("bm").unwrap(),
            self.resulting_move.to_algebraic()
        )
    }

    fn result(&self) -> bool {
        self.result
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents =
        fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file");

    let mut solver = Solver::new();

    let mut testsuit_result: TestSuit<BestMoveTestResult> = TestSuit::new();

    for line in contents.lines() {
        let epd = Epd::from_string(line);
        let board = epd.get_board();
        let best_move = solver.best_move(&board).unwrap();

        let expected_move = Move::from_algebraic(epd.get_properties().get("bm").unwrap()).unwrap();

        testsuit_result.push_test(BestMoveTestResult::new(
            TestCase {
                prompt: line.to_string(),
                epd,
            },
            best_move.clone(),
            best_move == expected_move,
        ));
    }

    testsuit_result.finalize();

    //let epd = Epd::from_string(&epd);

    //if epd.is_some() {
    //    println!("{:?}", epd);
    //    std::process::exit(0);
    //}
    //else {
    //    std::process::exit(1);
    //}

    //println!("{:?}", epd);
    //std::process::exit(0);
}
