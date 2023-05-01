use std::ops::Not;

/// Represents the board side.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum Side {
    #[default]
    White,
    Black,
}

/**
Negation implementation for Side:
 * Black -> White
 * White -> Black
*/
impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::White => Side::Black,
            Side::Black => Side::White,
        }
    }
}
