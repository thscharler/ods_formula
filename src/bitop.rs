use crate::{func, FNumber, Number};

/// Returns bitwise “and” of its parameters
#[inline]
pub fn bitand(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func("BITAND", &[&x, &y]))
}

/// Returns bitwise “or” of its parameters
#[inline]
pub fn bitor(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func("BITOR", &[&x, &y]))
}

/// Returns bitwise “exclusive or” of its parameters
#[inline]
pub fn bitxor(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func("BITXOR", &[&x, &y]))
}

/// Returns left shift of value X by N bits (“<<”)
#[inline]
pub fn bitlshift(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BITLSHIFT", &[&x, &n]))
}

/// Returns right shift of value X by N bits (“>>”)
#[inline]
pub fn bitrshift(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BITRSHIFT", &[&x, &n]))
}
