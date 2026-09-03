use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCase {
    id: String,
    description: String,
    fen: String,
    square: Option<String>,
    moves: Vec<String>,
}

pub struct MoveGenSuites {}

impl MoveGenSuites {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self) {
        let out_dir = env::var("OUT_DIR").unwrap();
        let contents = std::fs::read_to_string("testcases/movegen/standard.yaml").unwrap();
        let testsuit: Vec<TestCase> = serde_yaml::from_str(&contents).unwrap();

        let mut test_file = File::create(Path::new(&out_dir).join("movegen_tests.rs")).unwrap();

        write!(
            test_file,
            r#"
        use fchess::Move;
        use fchess::Square;
        use fchess::Board;
        use fchess::MoveGenerator;
        use fchess::MoveSet;
        "#
        )
        .unwrap();

        for (i, test_case) in testsuit.iter().enumerate() {
            write!(
                test_file,
                include_str!("templates/movegen.rs"),
                name = format_args!(
                    "{}{}",
                    test_case.description.to_lowercase().replace(" ", "_"),
                    i
                ),
                starting_fen = test_case.fen,
                square = test_case.square,
                moves = test_case.moves
            )
            .unwrap();
        }
        println!("Generate  movegen tests")
    }
}
