use crate::{func1, func2, func3, FNumber, Number, Param};

pub enum RoundingMode {
    AwayFrom0,
    TowardsPlusInf,
}
impl Param for RoundingMode {
    type ParamType<'a> = u32;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            RoundingMode::AwayFrom0 => 1,
            RoundingMode::TowardsPlusInf => 0,
        }
    }
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
#[inline]
pub fn ceiling(n: impl Number) -> FNumber {
    FNumber(func1("CEILIING", &n))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
#[inline]
pub fn ceiling2(n: impl Number, significance: impl Number) -> FNumber {
    FNumber(func2("CEILIING", &n, &significance))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
#[inline]
pub fn ceiling_mode(n: impl Number, mode: RoundingMode) -> FNumber {
    FNumber(func3("CEILIING", &n, &(), &mode.as_param()))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
#[inline]
pub fn ceiling2_mode(n: impl Number, significance: impl Number, mode: RoundingMode) -> FNumber {
    FNumber(func3("CEILIING", &n, &significance, &mode.as_param()))
}

/// Rounds a number down to the nearest integer
#[inline]
pub fn round_int(n: impl Number) -> FNumber {
    FNumber(func1("INT", &n))
}

/// Round a number N down to the nearest multiple of the second parameter,
/// significance.
#[inline]
pub fn floor(n: impl Number) -> FNumber {
    FNumber(func1("FLOOR", &n))
}

/// Round a number N down to the nearest multiple of the second parameter,
/// significance.
#[inline]
pub fn floor2(n: impl Number, significance: impl Number) -> FNumber {
    FNumber(func2("FLOOR", &n, &significance))
}

/// Round a number N down to the nearest multiple of the second parameter,
/// significance.
#[inline]
pub fn floor_mode(n: impl Number, mode: RoundingMode) -> FNumber {
    FNumber(func3("FLOOR", &n, &(), &mode.as_param()))
}

/// Round a number N down to the nearest multiple of the second parameter,
/// significance.
#[inline]
pub fn floor2_mode(n: impl Number, significance: impl Number, mode: RoundingMode) -> FNumber {
    FNumber(func3("FLOOR", &n, &significance, &mode.as_param()))
}

///  Rounds the number to given multiple.
#[inline]
pub fn mround(a: impl Number, b: impl Number) -> FNumber {
    FNumber(func2("MROUND", &a, &b))
}

/// Rounds the value X to the nearest multiple of the power of 10 specified by Digits.
#[inline]
pub fn round(x: impl Number) -> FNumber {
    FNumber(func1("ROUND", &x))
}

/// Rounds the value X to the nearest multiple of the power of 10 specified by Digits.
#[inline]
pub fn round_digits(x: impl Number, digits: impl Number) -> FNumber {
    FNumber(func2("ROUND", &x, &digits))
}

/// Rounds the value X towards zero to the number of digits specified by Digits.
#[inline]
pub fn rounddown(x: impl Number) -> FNumber {
    FNumber(func1("ROUNDDOWN", &x))
}

/// Rounds the value X towards zero to the number of digits specified by Digits.
#[inline]
pub fn rounddown_digits(x: impl Number, digits: impl Number) -> FNumber {
    FNumber(func2("ROUNDDOWN", &x, &digits))
}

/// Rounds the value X away from zero to the number of digits specified by Digits
#[inline]
pub fn roundup(x: impl Number) -> FNumber {
    FNumber(func1("ROUNDUP", &x))
}

/// Rounds the value X away from zero to the number of digits specified by Digits
#[inline]
pub fn roundup_digits(x: impl Number, digits: impl Number) -> FNumber {
    FNumber(func2("ROUNDUP", &x, &digits))
}

/// Truncate a number to a specified number of digits
#[inline]
pub fn trunc(x: impl Number) -> FNumber {
    FNumber(func1("TRUNC", &x))
}

/// Truncate a number to a specified number of digits
#[inline]
pub fn trunc_digits(x: impl Number, digits: impl Number) -> FNumber {
    FNumber(func2("TRUNC", &x, &digits))
}
