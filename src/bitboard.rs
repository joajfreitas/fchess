use crate::common::*;

pub type Bitboard = u64;

pub trait BitboardExt {
    fn shift(self, s: Shift) -> Bitboard;
    fn shift_p(self, s: Shift, columns: u64) -> Bitboard;
}

impl BitboardExt for Bitboard {
    #[inline]
    fn shift(self, s: Shift) -> Bitboard {
        if s > 0 {
            self << s
        } else {
            self >> -s
        }
    }

    fn shift_p(self, s: Shift, columns: u64) -> Bitboard {
        self.shift(s) & columns
    }
}
