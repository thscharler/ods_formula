use crate::{func1, func2, Matrix};
use crate::{FMatrix, FNumber};

/// Calculates the determinant of a matrix.
#[inline]
pub fn mdeterm(a: impl Matrix) -> FNumber {
    FNumber(func1("MDETERM", &a))
}

/// Returns the inverse of a matrix
#[inline]
pub fn minverse(a: impl Matrix) -> FMatrix {
    FMatrix(func1("MINVERSE", &a))
}

/// Multiplies the matrices A and B.
#[inline]
pub fn mmult(a: impl Matrix, b: impl Matrix) -> FMatrix {
    FMatrix(func2("MMUL", &a, &b))
}

/// Creates a unit matrix of a specified dimension N.
#[inline]
pub fn munit(a: impl Matrix) -> FMatrix {
    FMatrix(func1("MUNIT", &a))
}

/// Returns the transpose of a matrix.
#[inline]
pub fn transpose<A: Matrix>(a: A) -> FMatrix {
    FMatrix(func1("TRANSPOSE", &a))
}
