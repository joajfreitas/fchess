use priority_queue::PriorityQueue;
use std::cmp;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::board::Board;
use crate::book::Book;
use crate::move_generator::MoveGenerator;
use crate::moves::Move;
use crate::side::Side;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchResult {
    pub node_count: u64,
    pub time: Duration,
    pub nps: f32,
    pub best_move: Option<Move>,
    pub eval: i32,
    pub pv: Vec<Move>,
}

impl SearchResult {
    pub fn new(
        node_count: u64,
        time: Duration,
        nps: f32,
        best_move: Option<Move>,
        eval: i32,
        pv: &[Move],
    ) -> Self {
        Self {
            node_count,
            time,
            nps,
            best_move,
            eval,
            pv: pv.to_vec(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchLimits {
    depth: u32,
}

impl SearchLimits {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SearchConfig {
    depth: Option<u32>,
}

impl SearchConfig {
    pub fn new(depth: Option<u32>) -> Self {
        Self { depth }
    }

    pub fn resolve(&self) -> SearchLimits {
        SearchLimits::new(self.depth.unwrap_or(3))
    }
}

#[allow(unused)]
#[derive(Clone, Debug, Default)]
pub struct Search {
    move_generator: MoveGenerator,
    transposition_table: HashMap<u64, f32>,
    search_queue: PriorityQueue<Board, u32>,
    opening_book: Option<Book>,
    search_result: SearchResult,
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

impl Search {
    pub fn new() -> Self {
        Self {
            move_generator: MoveGenerator::new(),
            transposition_table: HashMap::default(),
            search_queue: PriorityQueue::new(),
            opening_book: None,
            search_result: SearchResult::default(),
        }
    }

    pub fn set_opening_book(&mut self, book: Book) {
        self.opening_book = Some(book);
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

    fn min_max(&mut self, node: &Node, max_depth: u32, max_min: bool) -> Option<(Node, i32)> {
        let mut max: i32 = -10000000;
        let mut best_node: Option<Node> = None;
        for mov in self
            .move_generator
            .generate_moves(&node.board)
            .iter()
            .flat_map(|moveset| moveset.into_iter())
        {
            self.search_result.node_count += 1;
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

            new_new_node.evaluation = Some(board_evaluation);

            max = cmp::max(new_new_node.evaluation?, max);
            if max == new_new_node.evaluation? {
                best_node = Some(new_new_node);
            }
        }

        Some((best_node?, max))
    }

    fn search(&mut self, board: &Board, search_limits: &SearchLimits) -> Option<SearchResult> {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let mut best: Option<Move> = None;
        let mut best_evaluation = -1000000000;
        let mut best_node: Option<Node> = None;
        for mov in self
            .move_generator
            .generate_moves(board)
            .iter()
            .flat_map(|moveset| moveset.into_iter())
        {
            let node = Node::new(board.apply(&mov).unwrap(), mov.clone(), 1, None);
            let r = self.min_max(
                &node,
                search_limits.depth,
                node.board.get_turn() == Side::White,
            );
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
        let mut pv: Vec<Move> = Vec::new();
        let evaluation = best_node.evaluation;
        loop {
            pv.insert(0, best_node.mov);
            //println!("{} {:?}", best_node.mov.to_algebraic(), best_node.evaluation);
            if best_node.parent.is_none() {
                break;
            }

            best_node = *best_node.parent.unwrap();
        }
        let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        self.search_result.time = end - start;
        self.search_result.nps =
            ((self.search_result.node_count as f64) / self.search_result.time.as_secs_f64()) as f32;
        self.search_result.best_move = best;
        self.search_result.pv = pv;
        self.search_result.eval = evaluation.unwrap();
        Some(self.search_result.clone())
    }

    pub fn best_move(
        &mut self,
        board: &Board,
        search_limits: &SearchLimits,
    ) -> Option<SearchResult> {
        self.search_result = SearchResult::default();
        if let Some(ref opening_book) = self.opening_book {
            opening_book
                .get_best_move(board)
                .map(|best_move| {
                    SearchResult::new(1, Duration::new(0, 0), 0.0, Some(best_move), 0, &Vec::new())
                })
                .or_else(|| self.search(board, search_limits))
        } else {
            self.search(board, search_limits)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use googletest::prelude::*;

    #[gtest]
    fn test_metrics_node_count_at_depth_2() -> Result<()> {
        let mut search = Search::new();

        let result = search.best_move(&Board::from_basic_board(), &SearchLimits::new(2));

        assert_that!(
            dbg!(result),
            some(field!(&SearchResult.node_count, eq(400)))
        );

        Ok(())
    }
}
