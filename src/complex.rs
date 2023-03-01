use crate::{func, FNumber, Number, Sequence};

pub fn complex<R: Number, I: Number>(real: R, imag: I) -> FNumber {
    FNumber(func("COMPLEX", &[&real, &imag]))
}

pub fn imabs<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMABS", &[&complex]))
}

pub fn imaginary<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMAGINARY", &[&complex]))
}

pub fn imargument<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMARGUMENT", &[&complex]))
}

pub fn imconjugate<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCONJUGATE", &[&complex]))
}

pub fn imcos<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCOS", &[&complex]))
}

pub fn imcosh<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCOSH", &[&complex]))
}

pub fn imcot<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCOT", &[&complex]))
}

pub fn imcsc<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCSC", &[&complex]))
}

pub fn imcsch<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMCSCH", &[&complex]))
}

pub fn imdiv<X: Number, Y: Number>(complex_x: X, complex_y: Y) -> FNumber {
    FNumber(func("IMDIV", &[&complex_x, &complex_y]))
}

pub fn imexp<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMEXP", &[&complex]))
}

pub fn imln<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMLN", &[&complex]))
}

pub fn imlog10<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMLOG10", &[&complex]))
}

pub fn imlog2<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMLOG2", &[&complex]))
}

pub fn impower<X: Number, Y: Number>(complex_x: X, complex_y: Y) -> FNumber {
    FNumber(func("IMPOWER", &[&complex_x, &complex_y]))
}

pub fn improduct<X: Number, Y: Number>(complex_x: X, complex_y: Y) -> FNumber {
    FNumber(func("IMPRODUCT", &[&complex_x, &complex_y]))
}

pub fn imreal<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMREAL", &[&complex]))
}

pub fn imsin<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMSIN", &[&complex]))
}

pub fn imsinh<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMSINH", &[&complex]))
}

pub fn imsec<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMSEC", &[&complex]))
}

pub fn imsech<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMSECH", &[&complex]))
}

pub fn imsqrt<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMSQRT", &[&complex]))
}

pub fn imsub<X: Number, Y: Number>(complex_x: X, complex_y: Y) -> FNumber {
    FNumber(func("IMSUB", &[&complex_x, &complex_y]))
}

pub fn imsum<S: Sequence>(complex_sequence: S) -> FNumber {
    FNumber(func("IMSUM", &[&complex_sequence]))
}

pub fn imtan<C: Number>(complex: C) -> FNumber {
    FNumber(func("IMTAN", &[&complex]))
}
