/// Set bit in bitfield according to value
pub fn set_bit<T>(
    bitfield: T,
    position: u8,
    value: T,
) -> <<T as std::ops::BitAnd<<T as std::ops::Not>::Output>>::Output as std::ops::BitOr<T>>::Output
where
    T: num::Unsigned
        + std::ops::Not
        + std::ops::Shl<u8>
        + std::ops::BitAnd<<T as std::ops::Not>::Output>
        + std::ops::Shl<u8, Output = T>,
    <T as std::ops::BitAnd<<T as std::ops::Not>::Output>>::Output:
        std::ops::BitOr<<T as std::ops::Shl<u8>>::Output>,
{
    let mask: T = T::one() << position;
    (bitfield & !mask) | (value << position)
}

/// Set bit in bitfield to true
pub fn enable_bit<T>(
    bitfield: T,
    position: u8,
) -> <<T as std::ops::BitAnd<<T as std::ops::Not>::Output>>::Output as std::ops::BitOr<T>>::Output

where
    T: num::Unsigned
        + std::ops::Not
        + std::ops::Shl<u8>
        + std::ops::BitAnd<<T as std::ops::Not>::Output>
        + std::ops::BitOr<
            <T as std::ops::Shl<u8>>::Output,
            Output = <<T as std::ops::BitAnd<<T as std::ops::Not>::Output>>::Output as std::ops::BitOr<T>>::Output,
        >,
<T as std::ops::BitAnd<<T as std::ops::Not>::Output>>::Output: std::ops::BitOr<T>
{
    bitfield | (T::one() << position)
}

#[cfg(test)]
mod tests {
    use super::{enable_bit, set_bit};

    #[test]
    fn test_set_bit() {
        assert_eq!(1_u32, set_bit(0_u32, 0, 1_u32))
    }

    #[test]
    fn test_enable_bit() {
        assert_eq!(1_u32, enable_bit(0_u32, 0))
    }
}
