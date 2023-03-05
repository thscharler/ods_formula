use crate::{func2, FNumber, Number};

/// Returns bitwise “and” of its parameters
#[inline]
pub fn bitand(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func2("BITAND", &x, &y))
}

/// Returns bitwise “or” of its parameters
#[inline]
pub fn bitor(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func2("BITOR", &x, &y))
}

/// Returns bitwise “exclusive or” of its parameters
#[inline]
pub fn bitxor(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func2("BITXOR", &x, &y))
}

/// Returns left shift of value X by N bits (“<<”)
#[inline]
pub fn bitlshift(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BITLSHIFT", &x, &n))
}

/// Returns right shift of value X by N bits (“>>”)
#[inline]
pub fn bitrshift(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BITRSHIFT", &x, &n))
}
