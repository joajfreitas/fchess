mod apply_move;
mod movegen;
mod rampart;

use crate::apply_move::ApplyMoveSuites;
use crate::movegen::MoveGenSuites;
use crate::rampart::RampartSuites;

fn main() {
    RampartSuites::new("./testcases/rampart").generate();
    MoveGenSuites::new("./testcases/movegen").generate();
    ApplyMoveSuites::new("./testcases/apply_move").generate();
}
