use crate::board::Board;
use crate::moves::{Move, MoveGenerator};

#[derive(Clone, Copy, Debug, Default)]
pub struct Solver {
    move_generator: MoveGenerator,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            move_generator: MoveGenerator::new(),
        }
    }

    pub fn best_move(&self, board: &Board) -> Option<Move> {
        let mut best = None;
        let mut score = -500.0;

        let mut evals = 0;
        for piece in self.move_generator.generate_moves(board) {
            for mov in piece.into_iter() {
                let b = board.apply(mov.clone())?;
                let (sc, min_max_evals) = self.min_max(&b, 3).unwrap();
                if score < sc {
                    best = Some(mov);
                    score = sc;
                }
                evals += min_max_evals;
            }
        }

        println!("evaluations: {}", evals);
        best
    }

    fn min_max(&self, board: &Board, depth: u8) -> Option<(f32, u32)> {
        //let mut best = None;
        let mut score = -500.0;
        let mut evals = 1;

        if depth == 0 || board.checkmate() {
            //return Some(board.eval(), evals));
            return Some((1.0, evals));
        }

        for piece in self.move_generator.generate_moves(board) {
            for mov in piece.into_iter() {
                let b = board.apply(mov.clone())?;
                let sc = self.min_max(&b, depth - 1);
                if sc.unwrap().0 > score {
                    score = sc.unwrap().0;
                }
                evals += sc.unwrap().1;
            }
        }

        Some((score, evals))
    }
}
