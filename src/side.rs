use std::ops::Not;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum Side {
    #[default]
    White,
    Black,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
