mod rampart;

use crate::rampart::RampartSuites;

fn main() {
    RampartSuites::new("./testcases/rampart/").generate();
}
