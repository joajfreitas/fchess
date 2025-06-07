use crate::board::Board;
use crate::piece::ColoredPieceType;
use crate::side::Side;
use crate::square::Square;
use anyhow::Result;

#[derive(Default, Clone)]
pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    #[allow(dead_code)]
    pub fn new() -> BoardBuilder {
        BoardBuilder {
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn with_piece(&self, coord: &str, piece_type: ColoredPieceType) -> BoardBuilder {
        let mut builder = self.clone();
        builder
            .board
            .set_piece(Square::from_algebraic(coord).unwrap(), piece_type);

        builder
    }

    #[allow(dead_code)]
    pub fn with_fen(&self, fen: &str) -> Result<BoardBuilder> {
        let mut builder = self.clone();
        builder.board = Board::from_fen(fen)?;
        Ok(builder)
    }

    #[allow(dead_code)]
    pub fn with_turn(&self, side: Side) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_turn(side);
        builder
    }

    #[allow(dead_code)]
    pub fn with_castling_white_short(&self, enabled: bool) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_castling_white_short(enabled);
        builder
    }

    #[allow(dead_code)]
    pub fn with_castling_white_long(&self, enabled: bool) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_castling_white_long(enabled);
        builder
    }

    #[allow(dead_code)]
    pub fn with_castling_black_short(&self, enabled: bool) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_castling_black_short(enabled);
        builder
    }

    #[allow(dead_code)]
    pub fn with_castling_black_long(&self, enabled: bool) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_castling_black_long(enabled);
        builder
    }

    #[allow(dead_code)]
    pub fn with_enpassant(&self, enpassant: Square) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_enpassant(Some(enpassant));
        builder
    }

    #[allow(dead_code)]
    pub fn with_half_move_clock(&self, half_move_clock: u8) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_half_move_clock(half_move_clock);
        builder
    }

    #[allow(dead_code)]
    pub fn with_full_move_clock(&self, full_move_clock: u8) -> BoardBuilder {
        let mut builder = self.clone();
        builder.board.set_full_move_clock(full_move_clock);
        builder
    }

    #[allow(dead_code)]
    pub fn with_basic_board(&self) -> Result<BoardBuilder> {
        self.with_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    #[allow(dead_code)]
    pub fn build(self) -> Board {
        self.board
    }
}
