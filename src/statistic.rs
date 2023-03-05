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

/// Returns the confidence interval for a population mean.
pub fn confidence(alpha: impl Number, stddev: impl Number, size: impl Number) -> FNumber {
    FNumber(func3("CONFIDENCE", &alpha, &stddev, &size))
}

/// Calculates the correlation coefficient of values in N1 and N2
pub fn correl(n1: impl Array, n2: impl Array) -> FNumber {
    FNumber(func2("CORREL", &n1, &n2))
}

/// Calculates covariance of two cell ranges.
pub fn covar(n1: impl Array, n2: impl Array) -> FNumber {
    FNumber(func2("COVAR", &n1, &n2))
}

/// Returns the smallest value for which the cumulative binomial distribution is greater
/// than or equal to a criterion value.
pub fn critbinom(trials: impl Number, sp: impl Number, alpha: impl Number) -> FNumber {
    FNumber(func3("CRITBINOM", &trials, &sp, &alpha))
}

/// Calculates sum of squares of deviations
pub fn devsq(n: impl Sequence) -> FNumber {
    FNumber(func1("DEVSQ", &n))
}

///  returns the value of the probability density function or the cumulative distribution
/// function for the exponential distribution.
pub fn expondist(x: impl Number, lambda: impl Number) -> FNumber {
    FNumber(func2("EXPONDIST", &x, &lambda))
}

///  returns the value of the probability density function or the cumulative distribution
/// function for the exponential distribution.
pub fn expondist_cu(x: impl Number, lambda: impl Number, cumulative: impl Logical) -> FNumber {
    FNumber(func3("EXPONDIST", &x, &lambda, &cumulative))
}

///  returns the value of the probability density function or the cumulative distribution
/// function for the F-distribution.
pub fn fdist(x: impl Number, r1: impl Number, r2: impl Number) -> FNumber {
    FNumber(func3("FDIST", &x, &r1, &r2))
}

///  returns the value of the probability density function or the cumulative distribution
/// function for the F-distribution.
pub fn fdist_cu(
    x: impl Number,
    r1: impl Number,
    r2: impl Number,
    cumulative: impl Logical,
) -> FNumber {
    FNumber(func4("FDIST", &x, &r1, &r2, &cumulative))
}

/// returns the area of the right tail of the probability density function for the F-distribution.
pub fn legacy_fdist(x: impl Number, r1: impl Number, r2: impl Number) -> FNumber {
    FNumber(func3("LEGACY.FDIST", &x, &r1, &r2))
}

/// returns the inverse of FDIST(x;R1;R2;TRUE()).
pub fn finv(p: impl Number, r1: impl Number, r2: impl Number) -> FNumber {
    FNumber(func3("FINV", &p, &r1, &r2))
}

///  returns the inverse of LEGACY.FDIST(x;R1;R2)
pub fn legacy_finv(p: impl Number, r1: impl Number, r2: impl Number) -> FNumber {
    FNumber(func3("LEGACY.FINV", &p, &r1, &r2))
}

/// returns the Fisher transformation.
pub fn fisher(r: impl Number) -> FNumber {
    FNumber(func1("FISHER", &r))
}

/// returns the inverse Fisher transformation.
pub fn fisherinv(r: impl Number) -> FNumber {
    FNumber(func1("FISHERINV", &r))
}

/// Extrapolates future values based on existing x and y values.
pub fn forecast(value: impl Number, data_y: impl Array, data_x: impl Array) -> FNumber {
    FNumber(func3("FORECAST", &value, &data_y, &data_x))
}

///
pub fn frequency(data: impl Sequence, bins: impl Sequence) -> FArray {}
