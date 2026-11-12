use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct PerftSuites {
    path: String,
}

impl PerftSuites {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    fn write_header(&self, test_file: &mut File) {
        write!(
            test_file,
            r#"
            use fchess::Epd;
            use fchess::perft::perft;
            "#
        )
        .unwrap();
    }

    fn write_test(&self, test_file: &mut File, test_name: &str, epd: &str) {
        write!(
            test_file,
            include_str!("templates/perft.rs"),
            test_name = test_name,
            epd = epd,
        )
        .unwrap();
    }

    pub fn generate(&self) {
        let out_dir = env::var("OUT_DIR").unwrap();
        let mut test_file = File::create(Path::new(&out_dir).join("perft_test.rs")).unwrap();

        self.write_header(&mut test_file);
        for (index, epd) in std::fs::read_to_string(self.path.clone())
            .unwrap()
            .lines()
            .into_iter()
            .enumerate()
        {
            self.write_test(&mut test_file, &format!("perft_{:03}", index), epd);
        }
    }
}
