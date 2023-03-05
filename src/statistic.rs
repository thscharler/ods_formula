use crate::{
    create_param, func, func1, func2, func3, func4, func5, param_assume_init, Any, Array,
    Criterion, FNumber, Logical, Number, Reference, Sequence,
};

/// Calculates the average of the absolute deviations of the values in list
#[inline]
pub fn avedev(n: impl Sequence) -> FNumber {
    FNumber(func1("AVEDEV", &n))
}

/// Average the set of numbers.
#[inline]
pub fn average(n: impl Sequence) -> FNumber {
    FNumber(func1("AVERAGE", &n))
}

/// Average values, including values of type Text and Logical.
#[inline]
pub fn averagea(n: impl Sequence) -> FNumber {
    FNumber(func1("AVERAGEA", &n))
}

/// Average the values of cells in a range that meet a criteria.
#[inline]
pub fn averageif(range: impl Reference, criterion: impl Criterion) -> FNumber {
    FNumber(func2("AVERAGEIF", &range, &criterion))
}

/// Average the values of cells in a range that meet a criteria.
#[inline]
pub fn averageif2(
    range: impl Reference,
    criterion: impl Criterion,
    avg: impl Reference,
) -> FNumber {
    FNumber(func3("AVERAGEIF", &range, &criterion, &avg))
}

///  Average the values of cells that meet multiple criteria in multiple ranges
#[inline]
pub fn averageifs(
    range: impl Reference,
    criterion: &[(impl Reference, impl Criterion)],
) -> FNumber {
    let mut param = create_param(criterion.len() * 2 + 1);
    param[0].write(&range);
    for (i, (r, c)) in criterion.iter().enumerate() {
        param[1 + 2 * i].write(r);
        param[1 + 2 * i + 1].write(c);
    }
    let param = unsafe { param_assume_init(param) };

    FNumber(func("AVERAGEIFS", param.as_ref()))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the beta distribution.
pub fn betadist(x: impl Number, alpha: impl Number, beta: impl Number) -> FNumber {
    FNumber(func3("BETADIST", &x, &alpha, &beta))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the beta distribution.
pub fn betadist_from(
    x: impl Number,
    alpha: impl Number,
    beta: impl Number,
    a: impl Number,
) -> FNumber {
    FNumber(func4("BETADIST", &x, &alpha, &beta, &a))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the beta distribution.
pub fn betadist_range(
    x: impl Number,
    alpha: impl Number,
    beta: impl Number,
    a: impl Number,
    b: impl Number,
) -> FNumber {
    FNumber(func5("BETADIST", &x, &alpha, &beta, &a, &b))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the beta distribution.
pub fn betadist_cu(
    x: impl Number,
    alpha: impl Number,
    beta: impl Number,
    a: impl Number,
    b: impl Number,
    cumulative: impl Logical,
) -> FNumber {
    FNumber(func("BETADIST", &[&x, &alpha, &beta, &a, &b, &cumulative]))
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
pub fn betainv(p: impl Number, alpha: impl Number, beta: impl Number) -> FNumber {
    FNumber(func3("BETAINV", &p, &alpha, &beta))
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
pub fn betainv_from(
    p: impl Number,
    alpha: impl Number,
    beta: impl Number,
    a: impl Number,
) -> FNumber {
    FNumber(func4("BETAINV", &p, &alpha, &beta, &a))
}

/// returns the inverse of BETADIST(x;α;β;A;B;TRUE()).
pub fn betainv_range(
    p: impl Number,
    alpha: impl Number,
    beta: impl Number,
    a: impl Number,
    b: impl Number,
) -> FNumber {
    FNumber(func5("BETAINV", &p, &alpha, &beta, &a, &b))
}

/// Returns the probability of a trial result using binomial distribution.
pub fn binom_dist_range(n: impl Number, p: impl Number, s: impl Number) -> FNumber {
    FNumber(func3("BINOM.DIST.RANGE", &n, &p, &s))
}

/// Returns the probability of a trial result using binomial distribution.
pub fn binom_dist_range2(
    n: impl Number,
    p: impl Number,
    s: impl Number,
    s2: impl Number,
) -> FNumber {
    FNumber(func4("BINOM.DIST.RANGE", &n, &p, &s, &s2))
}

/// Returns the binomial distribution.
pub fn binomdist(
    s: impl Number,
    n: impl Number,
    p: impl Number,
    cumulative: impl Logical,
) -> FNumber {
    FNumber(func4("BINOMDIST", &s, &n, &p, &cumulative))
}

/// returns the right-tail probability for the χ2-distribution
pub fn legacy_chidist(x: impl Number, degrees_of_freedom: impl Number) -> FNumber {
    FNumber(func2("LEGACY.CHIDIST", &x, &degrees_of_freedom))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the χ2-distribution.
pub fn chisqdist(x: impl Number, degrees_of_freedom: impl Number) -> FNumber {
    FNumber(func2("CHISQDIST", &x, &degrees_of_freedom))
}

/// returns the value of the probability density function or the cumulative distribution
/// function for the χ2-distribution.
pub fn chisqdist_cu(
    x: impl Number,
    degrees_of_freedom: impl Number,
    cumulative: impl Logical,
) -> FNumber {
    FNumber(func3("CHISQDIST", &x, &degrees_of_freedom, &cumulative))
}

/// returns the inverse of LEGACY.CHIDIST(x; DegreesOfFreedom)
pub fn legacy_chiinv(p: impl Number, degrees_of_freedom: impl Number) -> FNumber {
    FNumber(func2("LEGACY.CHIINV", &p, &degrees_of_freedom))
}

/// returns the inverse of CHISQDIST(x; DegreesOfFreedom; TRUE())
pub fn chisqinv(p: impl Number, degrees_of_freedom: impl Number) -> FNumber {
    FNumber(func2("CHISQINV", &p, &degrees_of_freedom))
}

/// Returns some Chi square goodness-for-fit test.
pub fn legacy_chitest(a: impl Array, e: impl Array) -> FNumber {
    FNumber(func2("LEGACY.CHITEST", &a, &e))
}
