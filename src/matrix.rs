use crate::Matrix;
use crate::{func, FMatrix, FNumber, Number};

pub fn mdeterm<A: Matrix>(a: A) -> FNumber {
    FNumber(func("MDETERM", &[&a]))
}

pub fn minverse<A: Matrix>(a: A) -> FMatrix {
    FMatrix(func("MINVERSE", &[&a]))
}

pub fn mmult<A: Matrix, B: Matrix>(a: A, b: B) -> FMatrix {
    FMatrix(func("MMUL", &[&a, &b]))
}

pub fn munit<A: Number>(a: A) -> FMatrix {
    FMatrix(func("MUNIT", &[&a]))
}

pub fn transpose<A: Matrix>(a: A) -> FMatrix {
    FMatrix(func("TRANSPOSE", &[&a]))
}
