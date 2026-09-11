use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::fs::read_dir;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TestCase {
    id: String,
    description: String,
    fen: String,
    square: Option<String>,
    moves: Vec<String>,
    disabled: Option<bool>,
}

pub struct MoveGenSuites {
    path: String,
}

impl MoveGenSuites {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn write_header(test_file: &mut File) {
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
    }

    fn write_test(test_file: &mut File, test_name: &str, test_cases: &[TestCase]) {
        for (index, test_case) in test_cases.iter().enumerate() {
            if test_case.disabled.unwrap_or(false) {
                continue;
            }
            write!(
                test_file,
                include_str!("templates/movegen.rs"),
                name = format_args!(
                    "movegen_{}_{}_{}",
                    test_name,
                    index,
                    test_case.description.to_lowercase().replace(" ", "_")
                ),
                starting_fen = test_case.fen,
                square = test_case.square,
                moves = test_case.moves
            )
            .unwrap();
        }
    }

    pub fn generate(&self) {
        let out_dir = env::var("OUT_DIR").unwrap();

        let mut test_file = File::create(Path::new(&out_dir).join("movegen_tests.rs")).unwrap();

        Self::write_header(&mut test_file);

        for directory in read_dir(&self.path).unwrap() {
            let mut dir_entry = directory.unwrap().path().canonicalize().unwrap();
            let contents = std::fs::read_to_string(dir_entry.clone()).unwrap();
            let testsuit: Vec<TestCase> = serde_yaml::from_str(&contents).unwrap();
            dir_entry.set_extension("");
            Self::write_test(
                &mut test_file,
                dir_entry.file_name().unwrap().to_str().unwrap(),
                &testsuit,
            );
        }
    }
}
