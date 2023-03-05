use crate::{func1, func2, func3, FNumber, Number, Param};

pub enum CeilingMode {
    AwayFrom0,
    TowardsPlusInf,
}
impl Param for CeilingMode {
    type ParamType<'a> = u32;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            CeilingMode::AwayFrom0 => 1,
            CeilingMode::TowardsPlusInf => 0,
        }
    }
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
pub fn ceiling(n: impl Number) -> FNumber {
    FNumber(func1("CEILIING", &n))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
pub fn ceiling2(n: impl Number, significance: impl Number) -> FNumber {
    FNumber(func2("CEILIING", &n, &significance))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
pub fn ceiling_mode(n: impl Number, mode: CeilingMode) -> FNumber {
    FNumber(func3("CEILIING", &n, &(), &mode.as_param()))
}

///  Round a number N up to the nearest multiple of the second parameter, significance.
pub fn ceiling2_mode(n: impl Number, significance: impl Number, mode: CeilingMode) -> FNumber {
    FNumber(func3("CEILIING", &n, &significance, &mode.as_param()))
}

/// Rounds a number down to the nearest integer
pub fn round_int(n: impl Number) -> FNumber {
    FNumber(func1("INT", &n))
}
