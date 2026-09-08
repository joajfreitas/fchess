use serde::{Deserialize, Serialize};
use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TestCase {
    id: String,
    description: String,
    start_fen: String,
    expected_fen: String,
    san: String,
    lan: String,
}

pub struct ApplyMoveSuites {
    path: String,
}

impl ApplyMoveSuites {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn write_header(test_file: &mut File) {
        write!(
            test_file,
            r#"
                use fchess::Board;
                use fchess::Move;
            "#
        )
        .unwrap();
    }

    fn write_test(test_file: &mut File, test_name: &str, test_cases: &[TestCase]) {
        for (index, test_case) in test_cases.iter().enumerate() {
            write!(
                test_file,
                include_str!("templates/apply_move.rs"),
                name = format_args!("apply_move_{}_{}_{}", test_name, index, test_case.id),
                starting_fen = test_case.start_fen,
                lan = test_case.lan,
                expected_fen = test_case.expected_fen
            )
            .unwrap();
        }
    }

    pub fn generate(&self) {
        let out_dir = env::var("OUT_DIR").unwrap();

        let mut test_file = File::create(Path::new(&out_dir).join("apply_move_tests.rs")).unwrap();

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
