mod movegen;
mod rampart;

use crate::movegen::MoveGenSuites;
use crate::rampart::RampartSuites;

fn main() {
    RampartSuites::new("./testcases/rampart/").generate();
    MoveGenSuites::new().generate()
}
