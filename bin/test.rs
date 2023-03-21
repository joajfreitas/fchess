use std::io;

use fchess::board::Board;
use fchess::moves::Scope;

fn main() -> io::Result<()> {
    //let board = Board::read_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    //let board = Board::read_fen("rnbqkb1r/pppp1ppp/5n2/4p3/Q7/2P5/PP1PPPPP/RNB1KBNR w KQkq - 2 3".to_string());
    let board = Board::read_fen(
        "r3kbnr/pp3ppp/1qn1b3/4p3/P2p4/1PPP4/4PPPP/RN1QKBNR w KQkq - 0 8".to_string(),
    );

    println!("{:?}", board);
    for mov in board.generate_moves(&Scope::White) {
        println!("{:?}", mov.piece);
        println!("{:?}", mov);
    }
    Ok(())
}
