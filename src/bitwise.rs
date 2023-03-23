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
