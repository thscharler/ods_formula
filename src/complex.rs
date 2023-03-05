use crate::{func1, func2, FNumber, Number, Sequence};

/// Creates a complex number from a given real coefficient and imaginary coefficient.
#[inline]
pub fn complex(real: impl Number, imag: impl Number) -> FNumber {
    FNumber(func2("COMPLEX", &real, &imag))
}

/// Returns the absolute value of a complex number.
#[inline]
pub fn imabs(complex: impl Number) -> FNumber {
    FNumber(func1("IMABS", &complex))
}

///Returns the imaginary coefficient of a complex number.
#[inline]
pub fn imaginary(complex: impl Number) -> FNumber {
    FNumber(func1("IMAGINARY", &complex))
}

///Returns the complex argument of a complex number
#[inline]
pub fn imargument(complex: impl Number) -> FNumber {
    FNumber(func1("IMARGUMENT", &complex))
}

/// Returns the complex conjugate of a complex number.
#[inline]
pub fn imconjugate(complex: impl Number) -> FNumber {
    FNumber(func1("IMCONJUGATE", &complex))
}

/// Returns the cosine of a complex number.
#[inline]
pub fn imcos(complex: impl Number) -> FNumber {
    FNumber(func1("IMCOS", &complex))
}

///Returns the hyperbolic cosine of a complex number.
#[inline]
pub fn imcosh(complex: impl Number) -> FNumber {
    FNumber(func1("IMCOSH", &complex))
}

///Returns the cotangent of a complex number.
#[inline]
pub fn imcot(complex: impl Number) -> FNumber {
    FNumber(func1("IMCOT", &complex))
}

///Returns the cosecant of a complex number
#[inline]
pub fn imcsc(complex: impl Number) -> FNumber {
    FNumber(func1("IMCSC", &complex))
}

///Returns the hyperbolic cosecant of a complex number.
#[inline]
pub fn imcsch(complex: impl Number) -> FNumber {
    FNumber(func1("IMCSCH", &complex))
}

/// Divides the first number by the second.
#[inline]
pub fn imdiv(complex_x: impl Number, complex_y: impl Number) -> FNumber {
    FNumber(func2("IMDIV", &complex_x, &complex_y))
}

///Returns the exponent of e and a complex number.
#[inline]
pub fn imexp(complex: impl Number) -> FNumber {
    FNumber(func1("IMEXP", &complex))
}

///Returns the natural logarithm of a complex number
#[inline]
pub fn imln(complex: impl Number) -> FNumber {
    FNumber(func1("IMLN", &complex))
}

/// Returns the common logarithm of a comp
#[inline]
pub fn imlog10(complex: impl Number) -> FNumber {
    FNumber(func1("IMLOG10", &complex))
}

///Returns the binary logarithm of a complex number.
#[inline]
pub fn imlog2(complex: impl Number) -> FNumber {
    FNumber(func1("IMLOG2", &complex))
}

///Returns the complex number X raised to the Yth power.
#[inline]
pub fn impower(complex_x: impl Number, complex_y: impl Number) -> FNumber {
    FNumber(func2("IMPOWER", &complex_x, &complex_y))
}

///Returns the product of complex numbers.
#[inline]
pub fn improduct(complex_x: impl Number, complex_y: impl Number) -> FNumber {
    FNumber(func2("IMPRODUCT", &complex_x, &complex_y))
}

/// Returns the real coefficient of a complex number.
#[inline]
pub fn imreal(complex: impl Number) -> FNumber {
    FNumber(func1("IMREAL", &complex))
}

/// Returns the sine of a complex number.
#[inline]
pub fn imsin(complex: impl Number) -> FNumber {
    FNumber(func1("IMSIN", &complex))
}

///Returns the hyperbolic sine of a comp
#[inline]
pub fn imsinh(complex: impl Number) -> FNumber {
    FNumber(func1("IMSINH", &complex))
}

///Returns the secant of a complex number.
#[inline]
pub fn imsec(complex: impl Number) -> FNumber {
    FNumber(func1("IMSEC", &complex))
}

///Returns the hyperbolic secant of a complex number.
#[inline]
pub fn imsech(complex: impl Number) -> FNumber {
    FNumber(func1("IMSECH", &complex))
}

/// Returns the square root of a complex number
#[inline]
pub fn imsqrt(complex: impl Number) -> FNumber {
    FNumber(func1("IMSQRT", &complex))
}

/// Subtracts the second complex number from the first
#[inline]
pub fn imsub(complex_x: impl Number, complex_y: impl Number) -> FNumber {
    FNumber(func2("IMSUB", &complex_x, &complex_y))
}

///Sums (add) a set of complex numbers, including all numbers in ranges.
#[inline]
pub fn imsum(complex_sequence: impl Sequence) -> FNumber {
    FNumber(func1("IMSUM", &complex_sequence))
}

///Returns the tangent of a complex number
#[inline]
pub fn imtan(complex: impl Number) -> FNumber {
    FNumber(func1("IMTAN", &complex))
}
