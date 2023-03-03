use crate::{func, FNumber, Number};
use std::fmt::{Display, Formatter};

pub fn abs(n: impl Number) -> FNumber {
    FNumber(func("ABS", &[&n]))
}

pub fn acos(n: impl Number) -> FNumber {
    FNumber(func("ACOS", &[&n]))
}

pub fn acosh(n: impl Number) -> FNumber {
    FNumber(func("ACOSH", &[&n]))
}

pub fn acot(n: impl Number) -> FNumber {
    FNumber(func("ACOT", &[&n]))
}

pub fn acoth(n: impl Number) -> FNumber {
    FNumber(func("ACOTH", &[&n]))
}

pub fn asin(n: impl Number) -> FNumber {
    FNumber(func("ASIN", &[&n]))
}

pub fn asinh(n: impl Number) -> FNumber {
    FNumber(func("ASINH", &[&n]))
}

pub fn atan(n: impl Number) -> FNumber {
    FNumber(func("ATAN", &[&n]))
}

pub fn atan2(x: impl Number, y: impl Number) -> FNumber {
    FNumber(func("ATAN2", &[&x, &y]))
}

pub fn atanh(n: impl Number) -> FNumber {
    FNumber(func("ATANH", &[&n]))
}

/// Returns the modified Bessel function of integer order In(X).
pub fn besseli(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BESSELI", &[&x, &n]))
}

/// Returns the Bessel function of integer order Jn(X) (cylinder function)
pub fn besselj(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BESSELJ", &[&x, &n]))
}

/// Returns the modified Bessel function of integer order Kn(x)
pub fn besselk(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BESSELK", &[&x, &n]))
}

/// Returns the Bessel function of integer order Yn(X), also known as the Neumann
/// function.
pub fn bessely(x: impl Number, n: impl Number) -> FNumber {
    FNumber(func("BESSELY", &[&x, &n]))
}

/// Returns the number of different R-length sets that can be selected from N items.
pub fn combin(n: impl Number, r: impl Number) -> FNumber {
    FNumber(func("COMBIN", &[&n, &r]))
}

/// Returns the number of combinations with repetitions.
pub fn combina(n: impl Number, m: impl Number) -> FNumber {
    FNumber(func("COMBINA", &[&n, &m]))
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
impl Display for BaseUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
        };
        write!(f, "{}", s)
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
impl Display for DecimalPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
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
        };
        write!(f, "{}", s)
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
impl Display for BinaryPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinaryPrefix::Yobi => "Yi",
            BinaryPrefix::Zebi => "Zi",
            BinaryPrefix::Exbi => "Ei",
            BinaryPrefix::Pebi => "Pi",
            BinaryPrefix::Tebi => "Ti",
            BinaryPrefix::Gibi => "Gi",
            BinaryPrefix::Mebi => "Mi",
            BinaryPrefix::Kibi => "Ki",
        };
        write!(f, "{}", s)
    }
}

pub enum ConvertUnit {
    Unit(BaseUnit),
    SI(DecimalPrefix, BaseUnit),
    Bin(BinaryPrefix, BaseUnit),
}
impl Display for ConvertUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertUnit::Unit(v) => {
                write!(f, "{}", v)
            }
            ConvertUnit::SI(p, v) => {
                write!(f, "{}{}", p, v)
            }
            ConvertUnit::Bin(p, v) => {
                write!(f, "{}{}", p, v)
            }
        }
    }
}

/// Returns a number converted from one unit system into another.
pub fn convert(n: impl Number, from: ConvertUnit, to: ConvertUnit) -> FNumber {
    FNumber(func("CONVERT", &[&n, &from.to_string(), &to.to_string()]))
}
