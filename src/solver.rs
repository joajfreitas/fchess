use priority_queue::PriorityQueue;
use std::cmp;
use std::collections::HashMap;

use crate::board::Board;
use crate::move_generator::MoveGenerator;
use crate::moves::Move;
use crate::side::Side;

#[derive(Clone, Debug, Default)]
pub struct Solver {
    move_generator: MoveGenerator,
    transposition_table: HashMap<u64, f32>,
    search_queue: PriorityQueue<Board, u32>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    board: Board,
    mov: Move,
    depth: u32,
    parent: Option<Box<Node>>,
    evaluation: Option<i32>,
}

impl Node {
    fn new(board: Board, mov: Move, depth: u32, parent: Option<Node>) -> Node {
        Node {
            board,
            mov,
            depth,
            parent: parent.map(Box::new),
            evaluation: None,
        }
    }
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            move_generator: MoveGenerator::new(),
            transposition_table: HashMap::default(),
            search_queue: PriorityQueue::new(),
        }
    }

    fn generate_moves(&self, board: &Board) -> Vec<Move> {
        self.move_generator
            .generate_moves(board)
            .iter()
            .flat_map(|moveset| moveset.into_iter())
            .collect::<Vec<Move>>()
    }

    pub fn best_move(&mut self, board: &Board) -> Option<Move> {
        let mut best: Option<Move> = None;
        let mut best_evaluation = -1000000000;
        let mut best_node: Option<Node> = None;
        for mov in self.generate_moves(board) {
            let node = Node::new(board.apply(&mov).unwrap(), mov.clone(), 1, None);
            let r = self.min_max(&node, 3, node.board.get_turn() == Side::White);
            if r.is_none() {
                continue;
            }

            let (node, evaluation) = r.unwrap();
            best_evaluation = cmp::max(best_evaluation, evaluation);
            if best_evaluation == evaluation {
                best = Some(mov);
                best_node = Some(node);
            }
        }

        let mut best_node = best_node.unwrap();
        loop {
            //println!("{} {:?}", best_node.mov.to_algebraic(), best_node.evaluation);
            if best_node.parent.is_none() {
                break;
            }

            best_node = *best_node.parent.unwrap();
        }

        best
    }

    fn min_max(&mut self, node: &Node, max_depth: u32, max_min: bool) -> Option<(Node, i32)> {
        let mut max: i32 = -10000000;
        let mut best_node: Option<Node> = None;
        for mov in self.generate_moves(&node.board) {
            let board_result: Board = node.board.apply(&mov).unwrap();
            let new_node = Node::new(
                board_result.clone(),
                mov,
                node.depth + 1,
                Some(node.clone()),
            );

            let (mut new_new_node, board_evaluation) = if node.depth + 1 == max_depth {
                (
                    new_node,
                    self.eval(&board_result) * if max_min { 1 } else { -1 },
                )
            } else {
                let m = self.min_max(&new_node, max_depth, !max_min);
                if m.is_none() {
                    continue;
                }
                m.unwrap()
            };

            new_new_node.evaluation = Some(-board_evaluation);

            max = cmp::max(new_new_node.evaluation?, max);
            if max == new_new_node.evaluation? {
                //println!("{:?} {:?}", new_new_node.mov, new_new_node.evaluation);
                best_node = Some(new_new_node);
            }
        }

        Some((best_node?, max))
    }

    fn eval(&self, board: &Board) -> i32 {
        let pieces_values: [i32; 14] = [
            10, 50, 30, 30, 90, 1000, -10, -50, -30, -30, -90, -1000, 0, 0,
        ];

        let mut s: i32 = 0;
        for piece in board {
            s += pieces_values[piece.get_type() as usize];
        }

        s.clamp(-1000, 1000)
    }
}
