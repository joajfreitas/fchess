use std::io;

use fchess::board::Board;
use fchess::moves::MoveGenerator;

fn main() -> io::Result<()> {
    let board = Board::from_fen("r3kbnr/pp3ppp/1qn1b3/4p3/P2p4/1PPP4/4PPPP/RN1QKBNR w KQkq - 0 8");
    let move_generator = MoveGenerator::new();

    println!("{:?}", board);
    for mov in move_generator.generate_moves(&board) {
        println!("{:?}", mov.piece);
        println!("{:?}", mov);
    }
    Ok(())
}
