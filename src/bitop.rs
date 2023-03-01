use crate::{func, FNumber, Number};

pub fn bitand<X: Number, Y: Number>(x: X, y: Y) -> FNumber {
    FNumber(func("BITAND", &[&x, &y]))
}

pub fn bitor<X: Number, Y: Number>(x: X, y: Y) -> FNumber {
    FNumber(func("BITOR", &[&x, &y]))
}

pub fn bitxor<X: Number, Y: Number>(x: X, y: Y) -> FNumber {
    FNumber(func("BITXOR", &[&x, &y]))
}

pub fn bitlshift<X: Number, N: Number>(x: X, n: N) -> FNumber {
    FNumber(func("BITLSHIFT", &[&x, &n]))
}

pub fn bitrshift<X: Number, N: Number>(x: X, n: N) -> FNumber {
    FNumber(func("BITRSHIFT", &[&x, &n]))
}
