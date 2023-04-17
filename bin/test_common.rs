pub trait TestResult: Clone {
    fn to_string(&self) -> String;
    fn result(&self) -> bool;
}

pub struct TestSuit<T> {
    total_tests: u32,
    failed_tests: u32,
    successfull_tests: u32,
    tests: Vec<T>,
}

impl<T: TestResult> TestSuit<T> {
    pub fn new() -> TestSuit<T> {
        TestSuit {
            total_tests: 0,
            failed_tests: 0,
            successfull_tests: 0,
            tests: Vec::new(),
        }
    }

    pub fn get_total_tests(&self) -> u32 {
        self.total_tests
    }

    pub fn get_failed_tests(&self) -> u32 {
        self.failed_tests
    }

    pub fn get_successful_tests(&self) -> u32 {
        self.successfull_tests
    }

    pub fn push_test(&mut self, test_result: T) {
        self.total_tests += 1;
        if test_result.result() {
            self.successfull_tests += 1;
        } else {
            self.failed_tests += 1;
        }
        self.tests.push(test_result);
    }

    pub fn finalize(&self) {
        for test in self.tests.iter() {
            if test.result() {
                println!("✓");
            } else {
                println!("{}", test.to_string());
            }
        }

        println!("{}/{}", self.get_successful_tests(), self.get_total_tests());
        std::process::exit(self.get_failed_tests() as i32);
    }
}
