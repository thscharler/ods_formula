use spreadsheet_ods::{CellRange, CellRef};
use std::fmt;
use std::fmt::Write;
use std::ops::{Add, BitAnd, Div, Mul, Sub};

pub mod prelude {
    pub use super::{
        Any, FormulaWriter, Integer, Logical, Number, NumberSequence, Reference, ReferenceList,
        Scalar, Text, Value,
    };
}

pub trait FormulaWriter {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result;
}

pub trait Integer: FormulaWriter {}

pub trait Number: FormulaWriter {
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

pub trait NumberSequence: FormulaWriter {}

pub trait Text: FormulaWriter {
    fn append<T: Text>(self, other: T) -> TextConcat<Self, T>
    where
        Self: Sized,
    {
        TextConcat { a: self, b: other }
    }
}

pub trait Scalar: FormulaWriter {}

pub trait Logical: FormulaWriter {
    fn eq<T: Logical>(self, other: T) -> EqOp<Self, T>
    where
        Self: Sized,
    {
        EqOp { a: self, b: other }
    }
}

pub trait Reference: FormulaWriter {}

pub trait ReferenceList: FormulaWriter {
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

pub trait Any: FormulaWriter {}

/// formula
pub fn formula<T: FormulaWriter>(f: T) -> Result<String, fmt::Error> {
    let mut buf = String::new();
    buf.write_str("of=")?;
    f.formula(&mut buf)?;
    Ok(buf)
}

pub trait Value
where
    Self: Sized,
{
    fn val(self) -> ValueFn<Self>;
}

impl<A: FormulaWriter> Value for A {
    fn val(self) -> ValueFn<A> {
        ValueFn { a: self }
    }
}

pub struct ValueFn<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for ValueFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        Ok(())
    }
}

impl<A: Any> Any for ValueFn<A> {}
impl<A: ReferenceList> ReferenceList for ValueFn<A> {}
impl<A: Reference> Reference for ValueFn<A> {}
impl<A: Logical> Logical for ValueFn<A> {}
impl<A: Scalar> Scalar for ValueFn<A> {}
impl<A: Text> Text for ValueFn<A> {}
impl<A: NumberSequence> NumberSequence for ValueFn<A> {}
impl<A: Number> Number for ValueFn<A> {}
impl<A: Integer> Integer for ValueFn<A> {}

pub fn val<A>(a: A) -> ValueFn<A> {
    ValueFn { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct Parentheses<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for Parentheses<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: Integer> Integer for Parentheses<A> {}
impl<A: Number> Number for Parentheses<A> {}
impl<A: NumberSequence> NumberSequence for Parentheses<A> {}
impl<A: Text> Text for Parentheses<A> {}
impl<A: Logical> Logical for Parentheses<A> {}
impl<A: Reference> Reference for Parentheses<A> {}
impl<A: ReferenceList> ReferenceList for Parentheses<A> {}
impl<A: Any> Any for Parentheses<A> {}

pub fn par<A>(a: A) -> Parentheses<A> {
    Parentheses { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

impl FormulaWriter for CellRef {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "{}", self.to_formula())
    }
}
impl Integer for CellRef {}
impl Number for CellRef {}
impl NumberSequence for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl Reference for CellRef {}
impl ReferenceList for CellRef {}
impl Any for CellRef {}

impl FormulaWriter for CellRange {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "{}", self.to_formula())
    }
}
impl Integer for CellRange {}
impl Number for CellRange {}
impl NumberSequence for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl Reference for CellRange {}
impl ReferenceList for CellRange {}
impl Any for CellRange {}

impl<T: FormulaWriter, const N: usize> FormulaWriter for [T; N] {
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
impl<T: NumberSequence, const N: usize> NumberSequence for [T; N] {}

macro_rules! value_int {
    ($t:ty) => {
        impl FormulaWriter for $t {
            fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                write!(buf, "{}", self)
            }
        }

        impl Integer for $t {}
        impl Number for $t {}
        impl Logical for $t {}
        impl Any for $t {}
    };
}
macro_rules! value_number {
    ($t:ty) => {
        impl FormulaWriter for $t {
            fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                write!(buf, "{}", self)
            }
        }

        impl Number for $t {}
        impl Logical for $t {}
        impl Any for $t {}
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

impl FormulaWriter for &str {
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
impl Any for &str {}

impl FormulaWriter for String {
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
impl Any for String {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct AddOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for AddOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "+")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> NumberSequence for AddOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Number for AddOp<A, B> {}
impl<A: FormulaWriter + Integer, B: FormulaWriter + Integer> Integer for AddOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for AddOp<A, B> {}

pub fn add<A: Number, B: Number>(a: A, b: B) -> AddOp<A, B> {
    AddOp { a, b }
}

pub struct SubOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for SubOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "-")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> NumberSequence for SubOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Number for SubOp<A, B> {}
impl<A: FormulaWriter + Integer, B: FormulaWriter + Integer> Integer for SubOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for SubOp<A, B> {}

pub fn sub<A: Number, B: Number>(a: A, b: B) -> SubOp<A, B> {
    SubOp { a, b }
}

pub struct MulOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for MulOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "*")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> NumberSequence for MulOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Number for MulOp<A, B> {}
impl<A: FormulaWriter + Integer, B: FormulaWriter + Integer> Integer for MulOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for MulOp<A, B> {}

pub fn mul<A: Number, B: Number>(a: A, b: B) -> MulOp<A, B> {
    MulOp { a, b }
}

pub struct DivOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for DivOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "/")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> NumberSequence for DivOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Number for DivOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for DivOp<A, B> {}

pub fn div<A: Number, B: Number>(a: A, b: B) -> DivOp<A, B> {
    DivOp { a, b }
}

pub struct EqOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for EqOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "=")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> Logical for EqOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for EqOp<A, B> {}

pub fn eq<A: Logical, B: Logical>(a: A, b: B) -> EqOp<A, B> {
    EqOp { a, b }
}

pub struct TextConcat<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for TextConcat<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "&")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> Text for TextConcat<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for TextConcat<A, B> {}

pub fn txt_concat<A: Text, B: Text>(a: A, b: B) -> TextConcat<A, B> {
    TextConcat { a, b }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct IntersectOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for IntersectOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "!")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> ReferenceList for IntersectOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for IntersectOp<A, B> {}

pub fn intersect<A: ReferenceList, B: ReferenceList>(a: A, b: B) -> IntersectOp<A, B> {
    IntersectOp { a, b }
}

pub struct ConcatOp<A, B> {
    a: A,
    b: B,
}

impl<A: FormulaWriter, B: FormulaWriter> FormulaWriter for ConcatOp<A, B> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        self.a.formula(buf)?;
        write!(buf, "~")?;
        self.b.formula(buf)?;
        Ok(())
    }
}

impl<A: FormulaWriter, B: FormulaWriter> ReferenceList for ConcatOp<A, B> {}
impl<A: FormulaWriter, B: FormulaWriter> Any for ConcatOp<A, B> {}

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

impl<D: FormulaWriter, M: FormulaWriter, Y: FormulaWriter> FormulaWriter for DateFn<D, M, Y> {
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

impl<D: FormulaWriter, M: FormulaWriter, Y: FormulaWriter> Integer for DateFn<D, M, Y> {}
impl<D: FormulaWriter, M: FormulaWriter, Y: FormulaWriter> Number for DateFn<D, M, Y> {}
impl<D: FormulaWriter, M: FormulaWriter, Y: FormulaWriter> NumberSequence for DateFn<D, M, Y> {}
impl<D: FormulaWriter, M: FormulaWriter, Y: FormulaWriter> Any for DateFn<D, M, Y> {}

pub fn date<D: Integer, M: Integer, Y: Integer>(day: D, month: M, year: Y) -> DateFn<D, M, Y> {
    DateFn { day, month, year }
}

pub struct DateValueFn<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for DateValueFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "DATEVALUE(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: FormulaWriter> Integer for DateValueFn<A> {}
impl<A: FormulaWriter> Number for DateValueFn<A> {}
impl<A: FormulaWriter> NumberSequence for DateValueFn<A> {}
impl<A: FormulaWriter> Any for DateValueFn<A> {}

pub fn date_value<A: Text>(a: A) -> DateValueFn<A> {
    DateValueFn { a }
}

pub struct CountFn<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for CountFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "COUNT(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: FormulaWriter> Integer for CountFn<A> {}
impl<A: FormulaWriter> Number for CountFn<A> {}
impl<A: FormulaWriter> NumberSequence for CountFn<A> {}
impl<A: FormulaWriter> Any for CountFn<A> {}

pub fn count<A: NumberSequence>(a: A) -> CountFn<A> {
    CountFn { a }
}

pub struct CosFn<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for CosFn<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "COS(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: FormulaWriter> Integer for CosFn<A> {}
impl<A: FormulaWriter> Number for CosFn<A> {}
impl<A: FormulaWriter> NumberSequence for CosFn<A> {}
impl<A: FormulaWriter> Any for CosFn<A> {}

pub fn cos<A: Number>(a: A) -> CosFn<A> {
    CosFn { a }
}

pub struct Sum<A> {
    a: A,
}

impl<A: FormulaWriter> FormulaWriter for Sum<A> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "SUM(")?;
        self.a.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

impl<A: FormulaWriter + Integer> Integer for Sum<A> {}
impl<A: FormulaWriter> Number for Sum<A> {}
impl<A: FormulaWriter> NumberSequence for Sum<A> {}
impl<A: FormulaWriter> Any for Sum<A> {}

pub fn sum<A: NumberSequence>(a: A) -> Sum<A> {
    Sum { a }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------
