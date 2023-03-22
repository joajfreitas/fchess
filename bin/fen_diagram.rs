use std::io;

use fchess::board::Board;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => println!("{:?}", err),
    };

    println!("{:?}", Board::read_fen(&input));
}
