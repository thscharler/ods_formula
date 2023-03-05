use crate::{
    func, func0, func1, func2, func3, func4, Any, Array, Criterion, FNumber, Number, Param,
    Reference, Sequence,
};
use std::borrow::Cow;
use std::fmt::Write;

/// Return the absolute (nonnegative) value.
#[inline]
pub fn abs(n: impl Number) -> FNumber {
    FNumber(func1("ABS", &n))
}

/// Returns the principal value of the arc cosine of a number. The angle is returned in
/// radians.
#[inline]
pub fn acos(n: impl Number) -> FNumber {
    FNumber(func1("ACOS", &n))
}

/// Return the principal value of the inverse hyperbolic cosine.
#[inline]
pub fn acosh(n: impl Number) -> FNumber {
    FNumber(func1("ACOSH", &n))
}

/// Return the principal value of the arc cotangent of a number. The angle is returned in
/// radians.
#[inline]
pub fn acot(n: impl Number) -> FNumber {
    FNumber(func1("ACOT", &n))
}

/// Return the hyperbolic arc cotangent
#[inline]
pub fn acoth(n: impl Number) -> FNumber {
    FNumber(func1("ACOTH", &n))
}

/// Return the principal value of the arc sine of a number. The angle is returned in
/// radians.
#[inline]
pub fn asin(n: impl Number) -> FNumber {
    FNumber(func1("ASIN", &n))
}

///  Return the principal value of the inverse hyperbolic sine
#[inline]
pub fn asinh(n: impl Number) -> FNumber {
    FNumber(func1("ASINH", &n))
}

/// Return the principal value of the arc tangent of a number. The angle is returned in
/// radians.
#[inline]
pub fn atan(n: impl Number) -> FNumber {
    FNumber(func1("ATAN", &n))
}

/// Returns the principal value of the arc tangent given a coordinate of two numbers.
/// The angle is returned in radians.
#[inline]
pub fn atan2(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func2("ATAN2", &x, &y))
}

/// Return the principal value of the inverse hyperbolic tangent
#[inline]
pub fn atanh(n: impl Number) -> FNumber {
    FNumber(func1("ATANH", &n))
}

/// Returns the modified Bessel function of integer order In(X).
#[inline]
pub fn besseli(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BESSELI", &x, &n))
}

/// Returns the Bessel function of integer order Jn(X) (cylinder function)
#[inline]
pub fn besselj(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BESSELJ", &x, &n))
}

/// Returns the modified Bessel function of integer order Kn(x)
#[inline]
pub fn besselk(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BESSELK", &x, &n))
}

/// Returns the Bessel function of integer order Yn(X), also known as the Neumann
/// function.
#[inline]
pub fn bessely(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func2("BESSELY", &x, &n))
}

/// Returns the number of different R-length sets that can be selected from N items.
#[inline]
pub fn combin(n: impl Number, r: impl Number) -> FNumber {
    FNumber(func2("COMBIN", &n, &r))
}

/// Returns the number of combinations with repetitions.
#[inline]
pub fn combina(n: impl Number, m: impl Number) -> FNumber {
    FNumber(func2("COMBINA", &n, &m))
}

pub enum BaseUnit {
    UKAcre,
    USAcre,
    SqAngstrom,
    Ar,
    SqFoot,
    Hectare,
    SqInch,
    SqMeter,
    Morgen,
    SqMile,
    SqNauticalMile,
    SqPica,
    SqYard,
    Angstrom,
    Ell,
    Foot,
    Inch,
    LightYear,
    Meter,
    Mile,
    NauticalMile,
    Parsec,
    Pica,
    SurveyMile,
    Yard,
    BTU,
    ThermCalorie,
    ITCalorie,
    Erg,
    ElectronVolt,
    Flb,
    HPh,
    Joule,
    WattHour,
    Dyne,
    Newton,
    Lbf,
    Pond,
    Bit,
    Byte,
    Gauss,
    Tesla,
    Gram,
    Grain,
    Cwt,
    UKCwt,
    Lbm,
    Stone,
    Ton,
    Ozm,
    Sg,
    AtomicMassUnit,
    UKTon,
    HP,
    PS,
    Watt,
    Atm,
    MmHg,
    Pascal,
    Psi,
    Torr,
    AdmiralityKnot,
    Knot,
    MetersPerHour,
    MetersPerSecond,
    MilesPerHour,
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
    Day,
    Hour,
    Minute,
    Second,
    Year,
    CbAngstrom,
    Barrel,
    Bushel,
    Cup,
    CbFoot,
    Gallon,
    GRT,
    CbInch,
    Liter,
    CbLightYear,
    CbMeter,
    CbMile,
    MTon,
    CbNauticalMile,
    FluidOunce,
    CbPica,
    Pint,
    Quart,
    Tablespoon,
    Teaspoon,
    ModernTeaspoon,
    UKGallon,
    UKPint,
    UKQuart,
    CbYard,
}
impl Param for BaseUnit {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            BaseUnit::UKAcre => "uk_acre",
            BaseUnit::USAcre => "us_acre",
            BaseUnit::SqAngstrom => "ang2",
            BaseUnit::Ar => "ar",
            BaseUnit::SqFoot => "ft2",
            BaseUnit::Hectare => "ha",
            BaseUnit::SqInch => "in2",
            BaseUnit::SqMeter => "ly2",
            BaseUnit::Morgen => "Morgen",
            BaseUnit::SqMile => "mi2",
            BaseUnit::SqNauticalMile => "Nmi2",
            BaseUnit::SqPica => "Pica2",
            BaseUnit::SqYard => "yd2",
            BaseUnit::Angstrom => "ang",
            BaseUnit::Ell => "ell",
            BaseUnit::Foot => "ft",
            BaseUnit::Inch => "in",
            BaseUnit::LightYear => "ly",
            BaseUnit::Meter => "m",
            BaseUnit::Mile => "mi",
            BaseUnit::NauticalMile => "Nmi",
            BaseUnit::Parsec => "pc",
            BaseUnit::Pica => "Pica",
            BaseUnit::SurveyMile => "survey_mi",
            BaseUnit::Yard => "yd",
            BaseUnit::BTU => "BTU",
            BaseUnit::ThermCalorie => "c",
            BaseUnit::ITCalorie => "cal",
            BaseUnit::Erg => "e",
            BaseUnit::ElectronVolt => "eV",
            BaseUnit::Flb => "flb",
            BaseUnit::HPh => "HPh",
            BaseUnit::Joule => "J",
            BaseUnit::WattHour => "Wh",
            BaseUnit::Dyne => "dyn",
            BaseUnit::Newton => "N",
            BaseUnit::Lbf => "lbf",
            BaseUnit::Pond => "pond",
            BaseUnit::Bit => "bit",
            BaseUnit::Byte => "byte",
            BaseUnit::Gauss => "ga",
            BaseUnit::Tesla => "T",
            BaseUnit::Gram => "g",
            BaseUnit::Grain => "grain",
            BaseUnit::Cwt => "cwt",
            BaseUnit::UKCwt => "uk_cwt",
            BaseUnit::Lbm => "lbm",
            BaseUnit::Stone => "stone",
            BaseUnit::Ton => "ton",
            BaseUnit::Ozm => "ozm",
            BaseUnit::Sg => "sg",
            BaseUnit::AtomicMassUnit => "u",
            BaseUnit::UKTon => "uk_ton",
            BaseUnit::HP => "HP",
            BaseUnit::PS => "PS",
            BaseUnit::Watt => "W",
            BaseUnit::Atm => "atm",
            BaseUnit::MmHg => "mmHg",
            BaseUnit::Pascal => "Pa",
            BaseUnit::Psi => "psi",
            BaseUnit::Torr => "Torr",
            BaseUnit::AdmiralityKnot => "admkn",
            BaseUnit::Knot => "kn",
            BaseUnit::MetersPerHour => "m/h",
            BaseUnit::MetersPerSecond => "m/s",
            BaseUnit::MilesPerHour => "mph",
            BaseUnit::Celsius => "C",
            BaseUnit::Fahrenheit => "F",
            BaseUnit::Kelvin => "K",
            BaseUnit::Rankine => "Rank",
            BaseUnit::Reaumur => "Reau",
            BaseUnit::Day => "day",
            BaseUnit::Hour => "hr",
            BaseUnit::Minute => "min",
            BaseUnit::Second => "sec",
            BaseUnit::Year => "yr",
            BaseUnit::CbAngstrom => "ang3",
            BaseUnit::Barrel => "barrel",
            BaseUnit::Bushel => "bushel",
            BaseUnit::Cup => "cup",
            BaseUnit::CbFoot => "ft3",
            BaseUnit::Gallon => "gal",
            BaseUnit::GRT => "GRT",
            BaseUnit::CbInch => "in3",
            BaseUnit::Liter => "l",
            BaseUnit::CbLightYear => "ly3",
            BaseUnit::CbMeter => "m3",
            BaseUnit::CbMile => "mi3",
            BaseUnit::MTon => "MTON",
            BaseUnit::CbNauticalMile => "Mmi3",
            BaseUnit::FluidOunce => "oz",
            BaseUnit::CbPica => "Pica3",
            BaseUnit::Pint => "pt",
            BaseUnit::Quart => "qt",
            BaseUnit::Tablespoon => "tbs",
            BaseUnit::Teaspoon => "tsp",
            BaseUnit::ModernTeaspoon => "tspm",
            BaseUnit::UKGallon => "uk_gal",
            BaseUnit::UKPint => "gu_pt",
            BaseUnit::UKQuart => "uk_qt",
            BaseUnit::CbYard => "yd3",
        }
    }
}
pub enum DecimalPrefix {
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
}
impl Param for DecimalPrefix {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            DecimalPrefix::Yotta => "Y",
            DecimalPrefix::Zetta => "Z",
            DecimalPrefix::Exa => "E",
            DecimalPrefix::Peta => "P",
            DecimalPrefix::Tera => "T",
            DecimalPrefix::Giga => "G",
            DecimalPrefix::Mega => "M",
            DecimalPrefix::Kilo => "k",
            DecimalPrefix::Hecto => "h",
            DecimalPrefix::Deka => "da",
            DecimalPrefix::Deci => "d",
            DecimalPrefix::Centi => "c",
            DecimalPrefix::Milli => "m",
            DecimalPrefix::Micro => "u",
            DecimalPrefix::Nano => "n",
            DecimalPrefix::Pico => "p",
            DecimalPrefix::Femto => "f",
            DecimalPrefix::Atto => "a",
            DecimalPrefix::Zepto => "z",
        }
    }
}
pub enum BinaryPrefix {
    Yobi,
    Zebi,
    Exbi,
    Pebi,
    Tebi,
    Gibi,
    Mebi,
    Kibi,
}
impl Param for BinaryPrefix {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            BinaryPrefix::Yobi => "Yi",
            BinaryPrefix::Zebi => "Zi",
            BinaryPrefix::Exbi => "Ei",
            BinaryPrefix::Pebi => "Pi",
            BinaryPrefix::Tebi => "Ti",
            BinaryPrefix::Gibi => "Gi",
            BinaryPrefix::Mebi => "Mi",
            BinaryPrefix::Kibi => "Ki",
        }
    }
}

pub enum ConvertUnit {
    Unit(BaseUnit),
    SI(DecimalPrefix, BaseUnit),
    Bin(BinaryPrefix, BaseUnit),
}
impl Param for ConvertUnit {
    type ParamType<'a> = Cow<'a, str>;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            ConvertUnit::Unit(v) => Cow::Borrowed(v.as_param()),
            ConvertUnit::SI(p, v) => {
                let mut buf = String::new();
                let _ = write!(buf, "{}{}", p.as_param(), v.as_param());
                Cow::Owned(buf)
            }
            ConvertUnit::Bin(p, v) => {
                let mut buf = String::new();
                let _ = write!(buf, "{}{}", p.as_param(), v.as_param());
                Cow::Owned(buf)
            }
        }
    }
}

/// Returns a number converted from one unit system into another.
#[inline]
pub fn convert(n: impl Number, from: ConvertUnit, to: ConvertUnit) -> FNumber {
    FNumber(func3("CONVERT", &n, &from.as_param(), &to.as_param()))
}

#[inline]
pub fn cos(n: impl Number) -> FNumber {
    FNumber(func1("COS", &n))
}

#[inline]
pub fn cosh(n: impl Number) -> FNumber {
    FNumber(func1("COSH", &n))
}

#[inline]
pub fn cot(n: impl Number) -> FNumber {
    FNumber(func1("COT", &n))
}

#[inline]
pub fn coth(n: impl Number) -> FNumber {
    FNumber(func1("COTH", &n))
}

#[inline]
pub fn csc(n: impl Number) -> FNumber {
    FNumber(func1("CSC", &n))
}

#[inline]
pub fn csch(n: impl Number) -> FNumber {
    FNumber(func1("CSCH", &n))
}

#[inline]
pub fn degresse(n: impl Number) -> FNumber {
    FNumber(func1("DEGREES", &n))
}

#[inline]
pub fn delta(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func2("DELTA", &x, &y))
}

#[inline]
pub fn erf(z0: impl Number, z1: impl Number) -> FNumber {
    FNumber(func2("ERF", &z0, &z1))
}

#[inline]
pub fn erfc(z: impl Number) -> FNumber {
    FNumber(func1("ERFC", &z))
}

#[inline]
pub fn even(n: impl Number) -> FNumber {
    FNumber(func1("EVEN", &n))
}

#[inline]
pub fn exp(n: impl Number) -> FNumber {
    FNumber(func1("EXP", &n))
}

#[inline]
pub fn fact(n: impl Number) -> FNumber {
    FNumber(func1("FACT", &n))
}

#[inline]
pub fn factdouble(n: impl Number) -> FNumber {
    FNumber(func1("FACTDOUBLE", &n))
}

#[inline]
pub fn gamma(n: impl Number) -> FNumber {
    FNumber(func1("GAMMA", &n))
}

#[inline]
pub fn gammaln(n: impl Number) -> FNumber {
    FNumber(func1("GAMMALN", &n))
}

#[inline]
pub fn gcd(n: impl Sequence) -> FNumber {
    FNumber(func1("GCD", &n))
}

#[inline]
pub fn gestep(n: impl Number) -> FNumber {
    FNumber(func1("GESTEP", &n))
}

#[inline]
pub fn gestep2(n: impl Number, step: impl Number) -> FNumber {
    FNumber(func2("GESTEP", &n, &step))
}

#[inline]
pub fn lcm(n: impl Sequence) -> FNumber {
    FNumber(func1("LCM", &n))
}

#[inline]
pub fn ln(n: impl Number) -> FNumber {
    FNumber(func1("LN", &n))
}

#[inline]
pub fn log(n: impl Number) -> FNumber {
    FNumber(func1("LOG", &n))
}

#[inline]
pub fn log_b(n: impl Number, base: impl Number) -> FNumber {
    FNumber(func2("LOG", &n, &base))
}

#[inline]
pub fn log10(n: impl Number) -> FNumber {
    FNumber(func1("LOG10", &n))
}

#[inline]
pub fn modulo(a: impl Number, b: impl Number) -> FNumber {
    FNumber(func2("MOD", &a, &b))
}

#[inline]
pub fn multinomial(n: impl Sequence) -> FNumber {
    FNumber(func1("MULTINOMIAL", &n))
}

#[inline]
pub fn odd(n: impl Number) -> FNumber {
    FNumber(func1("ODD", &n))
}

#[inline]
pub fn pi() -> FNumber {
    FNumber(func0("PI"))
}

#[inline]
pub fn power(a: impl Number, b: impl Number) -> FNumber {
    FNumber(func2("POWER", &a, &b))
}

#[inline]
pub fn product(n: impl Sequence) -> FNumber {
    FNumber(func1("PRODUCT", &n))
}

#[inline]
pub fn quotient(a: impl Number, b: impl Number) -> FNumber {
    FNumber(func2("QUOTIENT", &a, &b))
}

#[inline]
pub fn radians(n: impl Number) -> FNumber {
    FNumber(func1("RADIANS", &n))
}

#[inline]
pub fn rand() -> FNumber {
    FNumber(func0("RAND"))
}

#[inline]
pub fn randbetween(a: impl Number, b: impl Number) -> FNumber {
    FNumber(func2("RANDBETWEEN", &a, &b))
}

#[inline]
pub fn sec(n: impl Number) -> FNumber {
    FNumber(func1("SEC", &n))
}

#[inline]
pub fn seriessum(
    x: impl Number,
    n: impl Number,
    m: impl Number,
    coefficients: impl Array,
) -> FNumber {
    FNumber(func4("SERIESSUM", &x, &n, &m, &coefficients))
}

#[inline]
pub fn sign(n: impl Number) -> FNumber {
    FNumber(func1("SIGN", &n))
}

#[inline]
pub fn sin(n: impl Number) -> FNumber {
    FNumber(func1("SIN", &n))
}

#[inline]
pub fn sinh(n: impl Number) -> FNumber {
    FNumber(func1("SINH", &n))
}

#[inline]
pub fn sech(n: impl Number) -> FNumber {
    FNumber(func1("SECH", &n))
}

#[inline]
pub fn sqrt(n: impl Number) -> FNumber {
    FNumber(func1("SQRT", &n))
}

#[inline]
pub fn sqrtpi(n: impl Number) -> FNumber {
    FNumber(func1("SQRTPI", &n))
}

pub enum SubTotalFunction {
    Average,
    Count,
    CountA,
    Max,
    Min,
    Product,
    StDev,
    StDevP,
    Sum,
    Var,
    VarP,
}
impl SubTotalFunction {
    fn param_ext(&self, exclude_collapsed: bool) -> u32 {
        if exclude_collapsed {
            self.as_param() + 100
        } else {
            self.as_param()
        }
    }
}
impl Param for SubTotalFunction {
    type ParamType<'a> = u32;

    fn as_param(&self) -> u32 {
        match self {
            SubTotalFunction::Average => 1,
            SubTotalFunction::Count => 2,
            SubTotalFunction::CountA => 3,
            SubTotalFunction::Max => 4,
            SubTotalFunction::Min => 5,
            SubTotalFunction::Product => 6,
            SubTotalFunction::StDev => 7,
            SubTotalFunction::StDevP => 8,
            SubTotalFunction::Sum => 9,
            SubTotalFunction::Var => 10,
            SubTotalFunction::VarP => 11,
        }
    }
}

#[inline]
pub fn subtotal(f: SubTotalFunction, exclude_collapsed: bool, seq: impl Sequence) -> FNumber {
    FNumber(func2("SUBTOTAL", &f.param_ext(exclude_collapsed), &seq))
}

#[inline]
pub fn sum(n: impl Sequence) -> FNumber {
    FNumber(func1("SUM", &n))
}

#[inline]
pub fn sumif(range: impl Reference, criterion: impl Criterion) -> FNumber {
    FNumber(func2("SUMIF", &range, &criterion))
}

#[inline]
pub fn sumif2(range: impl Reference, criterion: impl Criterion, sum: impl Reference) -> FNumber {
    FNumber(func3("SUMIF", &range, &criterion, &sum))
}

#[inline]
pub fn sumifs(range: impl Reference, criterion: &[(impl Reference, impl Criterion)]) -> FNumber {
    let mut param: Vec<&dyn Any> = Vec::new();
    param.push(&range);
    for (r, c) in criterion {
        param.push(r);
        param.push(c)
    }
    FNumber(func("SUMIFS", param.as_slice()))
}

#[inline]
pub fn sumproduct(n: impl Sequence) -> FNumber {
    FNumber(func1("SUMPRODUCT", &n))
}

#[inline]
pub fn sumx2my2(n: impl Sequence) -> FNumber {
    FNumber(func1("SUMX2MY2", &n))
}

#[inline]
pub fn sumx2py2(n: impl Sequence) -> FNumber {
    FNumber(func1("SUMX2PY2", &n))
}

#[inline]
pub fn sumxmy2(n: impl Sequence) -> FNumber {
    FNumber(func1("SUMXMY2", &n))
}

#[inline]
pub fn tan(n: impl Number) -> FNumber {
    FNumber(func1("TAN", &n))
}

#[inline]
pub fn tanh(n: impl Number) -> FNumber {
    FNumber(func1("TANH", &n))
}
