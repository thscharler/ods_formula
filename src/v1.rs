mod logical;

use spreadsheet_ods::{CellRange, CellRef};
use std::fmt;
use std::fmt::Write;

pub use logical::*;

pub mod prelude {
    pub use super::{
        Any, Integer, Logical, Number, NumberSequence, Reference, ReferenceList, Scalar, Text,
    };
}

pub trait Any {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result;
}

#[inline]
fn func(name: &str, args: &[&dyn Any], buf: &mut dyn fmt::Write) -> fmt::Result {
    buf.write_str(name)?;
    buf.write_char('(')?;
    for (i, v) in args.iter().enumerate() {
        if i > 0 {
            buf.write_char(';')?;
        }
        v.formula(buf)?;
    }
    buf.write_char(')')?;
    Ok(())
}

pub trait Number: Any {
    fn add<T: Number>(self, other: T) -> AddOp<Self, T>
    where
        Self: Sized,
    {
        AddOp { a: self, b: other }
    }

    fn sub<T: Number>(self, other: T) -> SubOp<Self, T>
    where
        Self: Sized,
    {
        SubOp { a: self, b: other }
    }

    fn mul<T: Number>(self, other: T) -> MulOp<Self, T>
    where
        Self: Sized,
    {
        MulOp { a: self, b: other }
    }

    fn div<T: Number>(self, other: T) -> DivOp<Self, T>
    where
        Self: Sized,
    {
        DivOp { a: self, b: other }
    }
}
impl<A: Number> Scalar for A {}
impl<A: Number> DateParam for A {}
impl<A: Number> TimeParam for A {}

pub trait Complex: Any {}

pub trait Text: Any {
    fn append<T: Text>(self, other: T) -> TextConcat<Self, T>
    where
        Self: Sized,
    {
        TextConcat { a: self, b: other }
    }
}

pub trait Logical: Any {
    fn eq<T: Logical>(self, other: T) -> EqOp<Self, T>
    where
        Self: Sized,
    {
        EqOp { a: self, b: other }
    }
}

pub trait Reference: Any {}
impl<T: Reference> Array for T {}
impl<T: Reference> Database for T {}
impl<T: Reference> Criteria for T {}

pub trait ReferenceList: Any {
    fn intersect<T: ReferenceList>(self, other: T) -> IntersectOp<Self, T>
    where
        Self: Sized,
    {
        IntersectOp { a: self, b: other }
    }

    fn concat<T: ReferenceList>(self, other: T) -> ConcatOp<Self, T>
    where
        Self: Sized,
    {
        ConcatOp { a: self, b: other }
    }
}

pub trait Time: Any {}
pub trait Date: Any {}
pub trait DateTime: Any {}
pub trait Percentage: Any {}
pub trait Currency: Any {}
pub trait Array: Any {}

pub trait Scalar: Any {}
pub trait DateParam: Any {}
pub trait TimeParam: Any {}
pub trait Integer: Any {}
pub trait TextOrNumber: Any {}
pub trait Field: Any {}
pub trait NumberSequence: Any {}
pub trait NumberSequenceList: Any {}
pub trait DateSequence: Any {}
pub trait LogicalSequence: Any {}
pub trait ComplexSequence: Any {}
pub trait Database: Any {}
pub trait Criteria: Any {}

pub enum Criterion<F> {
    V(F),
    Eq(F),
    Ne(F),
    Lt(F),
    Gt(F),
    LtEq(F),
    GtEq(F),
}

impl<F: Any> Any for Criterion<F> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        match self {
            Criterion::V(f) => {
                f.formula(buf)?;
            }
            Criterion::Eq(f) => {
                write!(buf, "\"=\"&")?;
                f.formula(buf)?;
            }
            Criterion::Ne(f) => {
                write!(buf, "\"<>\"&")?;
                f.formula(buf)?;
            }
            Criterion::Lt(f) => {
                write!(buf, "\"<\"&")?;
                f.formula(buf)?;
            }
            Criterion::Gt(f) => {
                write!(buf, "\">\"&")?;
                f.formula(buf)?;
            }
            Criterion::LtEq(f) => {
                write!(buf, "\"<=\"&")?;
                f.formula(buf)?;
            }
            Criterion::GtEq(f) => {
                write!(buf, "\">=\"&")?;
                f.formula(buf)?;
            }
        }
        Ok(())
    }
}

/// formula
pub fn formula<T: Any>(f: T) -> Result<String, fmt::Error> {
    let mut buf = String::new();
    buf.write_str("of=")?;
    f.formula(&mut buf)?;
    Ok(buf)
}

// pub trait Value
// where
//     Self: Sized,
// {
//     fn val(self) -> ValueFn<Self>;
// }
//
// pub struct ValueFn<A> {
//     a: A,
// }
//
// impl<A: FormulaWriter> FormulaWriter for ValueFn<A> {
//     fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
//         self.a.formula(buf)?;
//         Ok(())
//     }
// }
//
// impl<A: Integer> Integer for ValueFn<A> {}
// impl<A: Number> Number for ValueFn<A> {}
// impl<A: NumberSequence> NumberSequence for ValueFn<A> {}
// impl<A: Any> Any for ValueFn<A> {}
// impl<A: ReferenceList> ReferenceList for ValueFn<A> {}
// impl<A: Reference> Reference for ValueFn<A> {}
// impl<A: Logical> Logical for ValueFn<A> {}
// impl<A: Scalar> Scalar for ValueFn<A> {}
// impl<A: Text> Text for ValueFn<A> {}
//
// pub fn val<A>(a: A) -> ValueFn<A> {
//     ValueFn { a }
// }

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct Parentheses<A> {
    a: A,
}

impl<A: Any> Any for Parentheses<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: Number> Number for Parentheses<A> {}
impl<A: Complex> Complex for Parentheses<A> {}
impl<A: Logical> Logical for Parentheses<A> {}
impl<A: Time> Time for Parentheses<A> {}
impl<A: Date> Date for Parentheses<A> {}
impl<A: DateTime> DateTime for Parentheses<A> {}
impl<A: Percentage> Percentage for Parentheses<A> {}
impl<A: Currency> Currency for Parentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for Parentheses<A> {}
impl<A: NumberSequence> NumberSequence for Parentheses<A> {}
impl<A: NumberSequenceList> NumberSequenceList for Parentheses<A> {}
impl<A: DateSequence> DateSequence for Parentheses<A> {}
impl<A: LogicalSequence> LogicalSequence for Parentheses<A> {}
impl<A: ComplexSequence> ComplexSequence for Parentheses<A> {}
impl<A: Text> Text for Parentheses<A> {}
impl<A: Field> Field for Parentheses<A> {}
impl<A: Reference> Reference for Parentheses<A> {}
impl<A: ReferenceList> ReferenceList for Parentheses<A> {}
impl<A: Integer> Integer for Parentheses<A> {}

pub fn par<A>(a: A) -> Parentheses<A> {
    Parentheses { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

impl Any for CellRef {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "{}", self.to_formula())
    }
}
impl Number for CellRef {}
impl Complex for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl Reference for CellRef {}
impl ReferenceList for CellRef {}
impl Time for CellRef {}
impl Date for CellRef {}
impl DateTime for CellRef {}
impl Percentage for CellRef {}
impl Currency for CellRef {}
impl Integer for CellRef {}
impl NumberSequence for CellRef {}
impl NumberSequenceList for CellRef {}
impl DateSequence for CellRef {}
impl LogicalSequence for CellRef {}
impl ComplexSequence for CellRef {}

impl Any for CellRange {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "{}", self.to_formula())
    }
}
impl Number for CellRange {}
impl Complex for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl Reference for CellRange {}
impl ReferenceList for CellRange {}
impl Time for CellRange {}
impl Date for CellRange {}
impl DateTime for CellRange {}
impl Percentage for CellRange {}
impl Currency for CellRange {}
impl Integer for CellRange {}
impl NumberSequence for CellRange {}
impl NumberSequenceList for CellRange {}
impl DateSequence for CellRange {}
impl LogicalSequence for CellRange {}
impl ComplexSequence for CellRange {}

impl<T: Any, const N: usize> Any for [T; N] {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        for (i, r) in self.iter().enumerate() {
            if i > 0 {
                write!(buf, ";")?;
            }
            r.formula(buf)?;
        }
        Ok(())
    }
}
impl<T: ReferenceList, const N: usize> Number for [T; N] {}
impl<T: ReferenceList, const N: usize> Complex for [T; N] {}
impl<T: ReferenceList, const N: usize> Text for [T; N] {}
impl<T: ReferenceList, const N: usize> Logical for [T; N] {}
impl<T: ReferenceList, const N: usize> ReferenceList for [T; N] {}
impl<T: ReferenceList, const N: usize> Time for [T; N] {}
impl<T: ReferenceList, const N: usize> Date for [T; N] {}
impl<T: ReferenceList, const N: usize> DateTime for [T; N] {}
impl<T: ReferenceList, const N: usize> Percentage for [T; N] {}
impl<T: ReferenceList, const N: usize> Currency for [T; N] {}
impl<T: ReferenceList, const N: usize> Integer for [T; N] {}
impl<T: ReferenceList, const N: usize> NumberSequenceList for [T; N] {}
impl<T: ReferenceList, const N: usize> DateSequence for [T; N] {}
impl<T: ReferenceList, const N: usize> LogicalSequence for [T; N] {}
impl<T: ReferenceList, const N: usize> ComplexSequence for [T; N] {}

macro_rules! value_int {
    ($t:ty) => {
        impl Any for $t {
            fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                write!(buf, "{}", self)
            }
        }

        impl Integer for $t {}
        impl Number for $t {}
        impl Complex for $t {}
        impl Logical for $t {}
        impl Date for $t {}
        impl Percentage for $t {}
        impl Currency for $t {}
        impl NumberSequence for $t {}
        impl NumberSequenceList for $t {}
        impl DateSequence for $t {}
        impl LogicalSequence for $t {}
        impl ComplexSequence for $t {}
        impl Scalar for $t {}
        impl DateParam for $t {}
        impl TextOrNumber for $t {}
        impl Field for $t {}
    };
}
macro_rules! value_number {
    ($t:ty) => {
        impl Any for $t {
            fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                write!(buf, "{}", self)
            }
        }

        impl Number for $t {}
        impl Complex for $t {}
        impl Logical for $t {}
        impl Time for $t {}
        impl Date for $t {}
        impl DateTime for $t {}
        impl Percentage for $t {}
        impl Currency for $t {}
        impl TextOrNumber for $t {}
        impl NumberSequence for $t {}
        impl NumberSequenceList for $t {}
        impl DateSequence for $t {}
        impl LogicalSequence for $t {}
        impl ComplexSequence for $t {}
    };
}

value_int!(i8);
value_int!(i16);
value_int!(i32);
value_int!(i64);
value_int!(i128);
value_int!(isize);
value_int!(u8);
value_int!(u16);
value_int!(u32);
value_int!(u64);
value_int!(u128);
value_int!(usize);
value_number!(f32);
value_number!(f64);

impl Any for bool {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        match self {
            true => buf.write_str("TRUE()"),
            false => buf.write_str("FALSE()"),
        }
    }
}

impl Logical for bool {}
impl Scalar for bool {}
impl LogicalSequence for bool {}

impl Any for &str {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        buf.write_char('"')?;
        for (i, s) in self.split('"').enumerate() {
            if i > 0 {
                buf.write_str("\"\"")?;
            }
            buf.write_str(s)?;
        }
        buf.write_char('"')?;
        Ok(())
    }
}
impl Text for &str {}
impl Scalar for &str {}
impl DateParam for &str {}
impl TimeParam for &str {}
impl TextOrNumber for &str {}
impl Field for &str {}

impl Any for String {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        buf.write_char('"')?;
        for (i, s) in self.split('"').enumerate() {
            if i > 0 {
                buf.write_str("\"\"")?;
            }
            buf.write_str(s)?;
        }
        buf.write_char('"')?;
        Ok(())
    }
}
impl Text for String {}
impl Scalar for String {}
impl DateParam for String {}
impl TimeParam for String {}
impl TextOrNumber for String {}
impl Field for String {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct AddOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for AddOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "+")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

//TODO????
impl<A: Number, B: Number> Number for AddOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for AddOp<A, B> {}
impl<A: Integer, B: Integer> Integer for AddOp<A, B> {}
impl<A: Integer, B: Integer> Field for AddOp<A, B> {}

pub fn add<A: Number, B: Number>(a: A, b: B) -> AddOp<A, B> {
    AddOp { a, b }
}

pub struct SubOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for SubOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "-")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: Number, B: Number> Number for SubOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for SubOp<A, B> {}
impl<A: Integer, B: Integer> Integer for SubOp<A, B> {}
impl<A: Integer, B: Integer> Field for SubOp<A, B> {}

pub fn sub<A: Number, B: Number>(a: A, b: B) -> SubOp<A, B> {
    SubOp { a, b }
}

pub struct MulOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for MulOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "*")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: Number, B: Number> Number for MulOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for MulOp<A, B> {}
impl<A: Integer, B: Integer> Integer for MulOp<A, B> {}
impl<A: Integer, B: Integer> Field for MulOp<A, B> {}

pub fn mul<A: Number, B: Number>(a: A, b: B) -> MulOp<A, B> {
    MulOp { a, b }
}

pub struct DivOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for DivOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "/")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: Number, B: Number> Number for DivOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for DivOp<A, B> {}

pub fn div<A: Number, B: Number>(a: A, b: B) -> DivOp<A, B> {
    DivOp { a, b }
}

pub struct EqOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for EqOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "=")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: Any, B: Any> Logical for EqOp<A, B> {}

pub fn eq<A: Logical, B: Logical>(a: A, b: B) -> EqOp<A, B> {
    EqOp { a, b }
}

pub struct TextConcat<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for TextConcat<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "&")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: Any, B: Any> Text for TextConcat<A, B> {}

pub fn txt_concat<A: Text, B: Text>(a: A, b: B) -> TextConcat<A, B> {
    TextConcat { a, b }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct IntersectOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for IntersectOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "!")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: ReferenceList, B: ReferenceList> ReferenceList for IntersectOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Number for IntersectOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Integer for IntersectOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Text for IntersectOp<A, B> {}

pub fn intersect<A: ReferenceList, B: ReferenceList>(a: A, b: B) -> IntersectOp<A, B> {
    IntersectOp { a, b }
}

pub struct ConcatOp<A, B> {
    a: A,
    b: B,
}

impl<A: Any, B: Any> Any for ConcatOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "~")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: ReferenceList, B: ReferenceList> ReferenceList for ConcatOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Number for ConcatOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Integer for ConcatOp<A, B> {}
impl<A: ReferenceList, B: ReferenceList> Text for ConcatOp<A, B> {}

pub fn concat<A: ReferenceList, B: ReferenceList>(a: A, b: B) -> ConcatOp<A, B> {
    ConcatOp { a, b }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct DateFn<D, M, Y> {
    day: D,
    month: M,
    year: Y,
}

impl<D: Any, M: Any, Y: Any> Any for DateFn<D, M, Y> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "DATE(")?;
        self.day.formula(buf)?;
        write!(buf, ";")?;
        self.month.formula(buf)?;
        write!(buf, ";")?;
        self.year.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<D: Integer, M: Integer, Y: Integer> Integer for DateFn<D, M, Y> {}
impl<D: Integer, M: Integer, Y: Integer> Number for DateFn<D, M, Y> {}

pub fn date<D: Integer, M: Integer, Y: Integer>(day: D, month: M, year: Y) -> DateFn<D, M, Y> {
    DateFn { day, month, year }
}

pub struct DateValueFn<A> {
    a: A,
}

impl<A: Any> Any for DateValueFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "DATEVALUE(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: Text> Integer for DateValueFn<A> {}
impl<A: Text> Number for DateValueFn<A> {}

pub fn date_value<A: Text>(a: A) -> DateValueFn<A> {
    DateValueFn { a }
}

pub struct CountFn<A> {
    a: A,
}

impl<A: Any> Any for CountFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "COUNT(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: NumberSequence> Integer for CountFn<A> {}
impl<A: NumberSequence> Field for CountFn<A> {}
impl<A: NumberSequence> Number for CountFn<A> {}

pub fn count<A: NumberSequence>(a: A) -> CountFn<A> {
    CountFn { a }
}

pub struct CosFn<A> {
    a: A,
}

impl<A: Any> Any for CosFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "COS(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: Any> Integer for CosFn<A> {}
impl<A: Any> Number for CosFn<A> {}

pub fn cos<A: Number>(a: A) -> CosFn<A> {
    CosFn { a }
}

pub struct Sum<A> {
    a: A,
}

impl<A: Any> Any for Sum<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "SUM(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: NumberSequenceList> Number for Sum<A> {}

pub fn sum<A: NumberSequenceList>(a: A) -> Sum<A> {
    Sum { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------
