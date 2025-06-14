use std::env;
use std::fs::read_dir;
use std::fs::DirEntry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct RampartTestPrelude {
    fen: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct RampartTestCase {
    #[serde(rename(deserialize = "move"))]
    mov: String,
    fen: String,
}

#[derive(Serialize, Deserialize)]
struct RampartTest {
    start: RampartTestPrelude,
    expected: Vec<RampartTestCase>,
}

#[derive(Serialize, Deserialize)]
struct RampartSuite {
    description: Option<String>,
    testCases: Vec<RampartTest>,
}

fn remove_invalid_function_name_chars(name: &str) -> String {
    name.replace(
        [
            ' ', '/', '-', '.', '#', ',', '\'', '+', ';', ':', '?', '=', '&', ')', '(', '!',
        ],
        "_",
    )
    .to_lowercase()
    .replace("______", "_")
    .replace("_____", "_")
    .replace("____", "_")
    .replace("___", "_")
    .replace("__", "_")
    .trim_matches('_')
    .to_string()
}

// build script's entry point
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut test_file = File::create(Path::new(&out_dir).join("tests.rs")).unwrap();

    // write test file header, put `use`, `const` etc there
    write_header(&mut test_file);

    for directory in read_dir("./testcases/rampart/").unwrap() {
        write_test(&mut test_file, directory.unwrap());
    }
}

fn write_test(test_file: &mut File, dir_entry: DirEntry) {
    let dir_entry = dir_entry.path().canonicalize().unwrap();

    let source = std::fs::read_to_string(&dir_entry).unwrap();
    println!("{:?}", dir_entry);
    let suite: RampartSuite = serde_json::from_str(&source).unwrap();

    for (test_id, test) in suite.testCases.iter().enumerate() {
        for (expected_id, expected) in test.expected.iter().enumerate() {
            let test_name = format!(
                "{}_{}_{}_{}_{}",
                dir_entry.file_stem().unwrap().to_str().unwrap(),
                remove_invalid_function_name_chars(&test.start.description),
                test_id,
                remove_invalid_function_name_chars(&expected.mov),
                expected_id
            );

            write!(
                test_file,
                include_str!("./tests/templates/move_generation.rs"),
                name = test_name,
                starting_fen = test.start.fen,
                san = expected.mov,
            )
            .unwrap();
        }
    }
}

fn write_header(test_file: &mut File) {
    write!(
        test_file,
        r#"
use fchess::Board;
use fchess::MoveGenerator;
use fchess::Move;
"#
    )
    .unwrap();
}
