mod logical;

use spreadsheet_ods::{CellRange, CellRef};
use std::fmt;
use std::fmt::Write;

pub use logical::*;

pub mod prelude {
    pub use super::{Any, Logical, Number, Reference, Text};
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

pub trait Text: Any {
    fn concat<T: Text>(self, other: T) -> TextConcat<Self, T>
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

pub trait Reference: Any {
    fn intersect<T: Reference>(self, other: T) -> IntersectOp<Self, T>
    where
        Self: Sized,
    {
        IntersectOp { a: self, b: other }
    }

    fn concat<T: Reference>(self, other: T) -> ConcatOp<Self, T>
    where
        Self: Sized,
    {
        ConcatOp { a: self, b: other }
    }
}

pub trait TextOrNumber: Any {}

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
impl<A: Text> Text for Parentheses<A> {}
impl<A: Logical> Logical for Parentheses<A> {}
impl<A: Reference> Reference for Parentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for Parentheses<A> {}

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
impl Text for CellRef {}
impl Logical for CellRef {}
impl Reference for CellRef {}

impl Any for CellRange {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "{}", self.to_formula())
    }
}
impl Number for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl Reference for CellRange {}

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
impl<T: Reference, const N: usize> Number for [T; N] {}
impl<T: Reference, const N: usize> Text for [T; N] {}
impl<T: Reference, const N: usize> Logical for [T; N] {}
impl<T: Reference, const N: usize> Reference for [T; N] {}

macro_rules! value_int {
    ($t:ty) => {
        impl Any for $t {
            fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                write!(buf, "{}", self)
            }
        }

        impl Number for $t {}
        impl Logical for $t {}
        impl TextOrNumber for $t {}
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
        impl Logical for $t {}
        impl TextOrNumber for $t {}
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
impl TextOrNumber for &str {}

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
impl TextOrNumber for String {}

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

impl<A: Number, B: Number> Number for AddOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for AddOp<A, B> {}
impl<A: Number, B: Number> Logical for AddOp<A, B> {}

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
impl<A: Number, B: Number> Logical for SubOp<A, B> {}

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
impl<A: Number, B: Number> Logical for MulOp<A, B> {}
impl<A: Number, B: Number> TextOrNumber for MulOp<A, B> {}

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
impl<A: Number, B: Number> Logical for DivOp<A, B> {}

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

pub fn eq<A: Any, B: Any>(a: A, b: B) -> EqOp<A, B> {
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

impl<A: Text, B: Text> Text for TextConcat<A, B> {}

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

impl<A: Reference, B: Reference> Reference for IntersectOp<A, B> {}
impl<A: Reference, B: Reference> Number for IntersectOp<A, B> {}
impl<A: Reference, B: Reference> Logical for IntersectOp<A, B> {}
impl<A: Reference, B: Reference> Text for IntersectOp<A, B> {}

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

impl<A: Reference, B: Reference> Reference for ConcatOp<A, B> {}
impl<A: Reference, B: Reference> Number for ConcatOp<A, B> {}
impl<A: Reference, B: Reference> Logical for ConcatOp<A, B> {}
impl<A: Reference, B: Reference> Text for ConcatOp<A, B> {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct DateFn<D, M, Y> {
    day: D,
    month: M,
    year: Y,
}

impl<D: Any, M: Any, Y: Any> Any for DateFn<D, M, Y> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        func("DATE", &[&self.day, &self.month, &self.year], buf)
    }
}

impl<D: Number, M: Number, Y: Number> Number for DateFn<D, M, Y> {}

pub fn date<D: Number, M: Number, Y: Number>(day: D, month: M, year: Y) -> DateFn<D, M, Y> {
    DateFn { day, month, year }
}

pub struct DateValueFn<A> {
    a: A,
}

impl<A: Any> Any for DateValueFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        func("DATEVALUE", &[&self.a], buf)
    }
}

impl<A: Text> Number for DateValueFn<A> {}

pub fn date_value<A: Text>(a: A) -> DateValueFn<A> {
    DateValueFn { a }
}

pub struct CountFn<A> {
    a: A,
}

impl<A: Any> Any for CountFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        func("COUNT", &[&self.a], buf)
    }
}

impl<A: Number> Number for CountFn<A> {}

pub fn count<A: Number>(a: A) -> CountFn<A> {
    CountFn { a }
}

pub struct CosFn<A> {
    a: A,
}

impl<A: Any> Any for CosFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        func("COS", &[&self.a], buf)
    }
}

impl<A: Number> Number for CosFn<A> {}

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

impl<A: Number> Number for Sum<A> {}

pub fn sum<A: Number>(a: A) -> Sum<A> {
    Sum { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------
