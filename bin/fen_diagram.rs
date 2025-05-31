use std::env;
use std::io;

use fchess::Board;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = if args.len() == 2 {
        args[1].to_string()
    } else {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(err) => println!("{err:?}"),
        };
        input
    };

    println!("{}", Board::from_fen(&input));
}
