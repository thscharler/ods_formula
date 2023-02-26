use std::fmt;
use std::fmt::Write;
use std::ops::{Add, Div, Mul, Sub};

pub use args::*;
pub use values::*;

///
pub trait Formula {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result;

    fn val<'a>(&'a self) -> &'a dyn Formula
    where
        Self: Sized,
    {
        self
    }
}

impl Formula for &dyn Formula {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        (*self).formula(buf)
    }
}
//
// pub trait FormulaExt {
//     fn val<'a>(&'a self) -> &'a dyn Formula;
// }
//
// impl FormulaExt for dyn Formula {
//     fn val<'a>(&'a self) -> &'a dyn Formula {
//         self
//     }
// }
//
// impl FormulaExt for &dyn Formula {
//     fn val<'a>(&'a self) -> &'a dyn Formula {
//         *self
//     }
// }

pub trait FormulaTransform
where
    Self: Sized,
{
    fn par(self) -> Parentheses<Self>;
}

impl FormulaTransform for &dyn Formula {
    fn par(self) -> Parentheses<Self> {
        Parentheses { f: self }
    }
}

pub fn val(f: &dyn Formula) -> &dyn Formula {
    f
}

pub fn par<T: Formula>(f: T) -> Parentheses<T> {
    Parentheses { f }
}

/// Varargs
pub trait FunctionArgs {
    fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result;
}

/// Reference and Referencelist
pub trait Reference {
    fn refs(&self, buf: &mut dyn fmt::Write) -> fmt::Result;
}

pub trait ReferenceList {
    fn refs(&self, buf: &mut dyn fmt::Write) -> fmt::Result;
}

pub struct Parentheses<F> {
    f: F,
}

impl<F: Formula> Formula for Parentheses<F> {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        buf.write_char('(')?;
        self.f.formula(buf)?;
        buf.write_char(')')?;
        Ok(())
    }
}

/// formula
pub fn formula<T: Formula>(f: T) -> Result<String, fmt::Error> {
    let mut buf = String::new();
    buf.write_str("of=")?;
    f.formula(&mut buf)?;
    Ok(buf)
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct Sum<A> {
    args: A,
}

impl<A> Formula for Sum<A>
where
    A: FunctionArgs,
{
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "SUM(")?;
        self.args.args(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

pub fn sum<A: FunctionArgs>(args: A) -> Sum<A> {
    Sum { args }
}

pub enum Criterion<F> {
    V(F),
    Eq(F),
    Ne(F),
    Lt(F),
    Gt(F),
    LtEq(F),
    GtEq(F),
}

impl<F> Criterion<F>
where
    F: Formula,
{
    pub fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
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

pub struct CountIf<A, G> {
    args: A,
    c: Criterion<G>,
}

impl<F: Formula, G: Formula> Formula for CountIf<F, G> {
    fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
        write!(buf, "COUNTIF(")?;
        self.args.args(buf)?;
        write!(buf, ";")?;
        self.c.formula(buf)?;
        write!(buf, ")")?;
        Ok(())
    }
}

pub fn count_if<A: Formula, C: Formula>(f: A, c: Criterion<C>) -> CountIf<A, C> {
    CountIf { args: f, c }
}

// -----------------------------------------------------------------------
// ops
// -----------------------------------------------------------------------

pub struct AddFormula<T, O> {
    t: T,
    o: O,
}

impl<T: Formula, O: Formula> Formula for AddFormula<T, O> {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        self.t.formula(buf)?;
        buf.write_char('+')?;
        self.o.formula(buf)?;
        Ok(())
    }
}

impl<'a> Add<&'a dyn Formula> for &'a dyn Formula {
    type Output = AddFormula<&'a dyn Formula, &'a dyn Formula>;

    fn add(self, rhs: &'a dyn Formula) -> Self::Output {
        AddFormula { t: self, o: rhs }
    }
}

pub struct SubFormula<T, O> {
    t: T,
    o: O,
}

impl<T: Formula, O: Formula> Formula for SubFormula<T, O> {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        self.t.formula(buf)?;
        buf.write_char('+')?;
        self.o.formula(buf)?;
        Ok(())
    }
}

impl<'a> Sub<&'a dyn Formula> for &'a dyn Formula {
    type Output = SubFormula<&'a dyn Formula, &'a dyn Formula>;

    fn sub(self, rhs: &'a dyn Formula) -> Self::Output {
        SubFormula { t: self, o: rhs }
    }
}

pub struct MulFormula<T, O> {
    t: T,
    o: O,
}

impl<T: Formula, O: Formula> Formula for MulFormula<T, O> {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        self.t.formula(buf)?;
        buf.write_char('+')?;
        self.o.formula(buf)?;
        Ok(())
    }
}

impl<'a> Mul<&'a dyn Formula> for &'a dyn Formula {
    type Output = MulFormula<&'a dyn Formula, &'a dyn Formula>;

    fn mul(self, rhs: &'a dyn Formula) -> Self::Output {
        MulFormula { t: self, o: rhs }
    }
}

pub struct DivFormula<T, O> {
    t: T,
    o: O,
}

impl<T: Formula, O: Formula> Formula for DivFormula<T, O> {
    fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
        self.t.formula(buf)?;
        buf.write_char('+')?;
        self.o.formula(buf)?;
        Ok(())
    }
}

impl<'a> Div<&'a dyn Formula> for &'a dyn Formula {
    type Output = DivFormula<&'a dyn Formula, &'a dyn Formula>;

    fn div(self, rhs: &'a dyn Formula) -> Self::Output {
        DivFormula { t: self, o: rhs }
    }
}

mod refs {
    use crate::{Reference, ReferenceList};
    use spreadsheet_ods::{CellRange, CellRef};
    use std::fmt::Write;

    pub struct Intersect2<A, B> {
        a: A,
        b: B,
    }

    pub fn intersect2<A: Reference, B: Reference>(a: A, b: B) -> Intersect2<A, B> {
        Intersect2 { a, b }
    }

    impl<A, B> ReferenceList for Intersect2<A, B>
    where
        A: Reference,
        B: Reference,
    {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            write!(buf, "(")?;
            self.a.refs(buf)?;
            write!(buf, "!")?;
            self.b.refs(buf)?;
            write!(buf, ")")?;
            Ok(())
        }
    }

    impl Reference for CellRef {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl Reference for &CellRef {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl Reference for CellRange {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl Reference for &CellRange {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl ReferenceList for () {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl<A> ReferenceList for (A,)
    where
        A: Reference,
    {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }

    impl<A, B> ReferenceList for (A, B)
    where
        A: Reference,
        B: Reference,
    {
        fn refs(&self, buf: &mut dyn Write) -> std::fmt::Result {
            todo!()
        }
    }
}

mod args {
    use crate::{Formula, FunctionArgs};
    use std::fmt;

    impl FunctionArgs for () {
        fn args(&self, _buf: &mut dyn fmt::Write) -> fmt::Result {
            Ok(())
        }
    }

    impl<A> FunctionArgs for (A,)
    where
        A: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B> FunctionArgs for (A, B)
    where
        A: Formula,
        B: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C> FunctionArgs for (A, B, C)
    where
        A: Formula,
        B: Formula,
        C: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D> FunctionArgs for (A, B, C, D)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E> FunctionArgs for (A, B, C, D, E)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E, F> FunctionArgs for (A, B, C, D, E, F)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
        F: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            buf.write_char(';')?;
            self.5.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E, F, G> FunctionArgs for (A, B, C, D, E, F, G)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
        F: Formula,
        G: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            buf.write_char(';')?;
            self.5.formula(buf)?;
            buf.write_char(';')?;
            self.6.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E, F, G, H> FunctionArgs for (A, B, C, D, E, F, G, H)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
        F: Formula,
        G: Formula,
        H: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            buf.write_char(';')?;
            self.5.formula(buf)?;
            buf.write_char(';')?;
            self.6.formula(buf)?;
            buf.write_char(';')?;
            self.7.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E, F, G, H, I> FunctionArgs for (A, B, C, D, E, F, G, H, I)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
        F: Formula,
        G: Formula,
        H: Formula,
        I: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            buf.write_char(';')?;
            self.5.formula(buf)?;
            buf.write_char(';')?;
            self.6.formula(buf)?;
            buf.write_char(';')?;
            self.7.formula(buf)?;
            buf.write_char(';')?;
            self.8.formula(buf)?;
            Ok(())
        }
    }

    impl<A, B, C, D, E, F, G, H, I, J> FunctionArgs for (A, B, C, D, E, F, G, H, I, J)
    where
        A: Formula,
        B: Formula,
        C: Formula,
        D: Formula,
        E: Formula,
        F: Formula,
        G: Formula,
        H: Formula,
        I: Formula,
        J: Formula,
    {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            self.0.formula(buf)?;
            buf.write_char(';')?;
            self.1.formula(buf)?;
            buf.write_char(';')?;
            self.2.formula(buf)?;
            buf.write_char(';')?;
            self.3.formula(buf)?;
            buf.write_char(';')?;
            self.4.formula(buf)?;
            buf.write_char(';')?;
            self.5.formula(buf)?;
            buf.write_char(';')?;
            self.6.formula(buf)?;
            buf.write_char(';')?;
            self.7.formula(buf)?;
            buf.write_char(';')?;
            self.8.formula(buf)?;
            buf.write_char(';')?;
            self.9.formula(buf)?;
            Ok(())
        }
    }

    impl<F: Formula, const N: usize> FunctionArgs for [F; N] {
        fn args(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
            for (i, a) in self.iter().enumerate() {
                if i > 0 {
                    buf.write_char(';')?;
                }
                a.formula(buf)?;
            }
            Ok(())
        }
    }
}

mod values {
    use crate::Formula;
    use spreadsheet_ods::{CellRange, CellRef, ColRange, RowRange};
    use std::fmt;
    use std::fmt::Write;

    impl Formula for CellRef {
        fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
            write!(buf, "{}", self.to_formula())
        }
    }

    impl Formula for CellRange {
        fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
            write!(buf, "{}", self.to_formula())
        }
    }

    impl Formula for ColRange {
        fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
            write!(buf, "{}", self.to_formula())
        }
    }

    impl Formula for RowRange {
        fn formula(&self, buf: &mut dyn Write) -> fmt::Result {
            write!(buf, "{}", self.to_formula())
        }
    }

    macro_rules! value {
        ($t:ty) => {
            impl Formula for $t {
                fn formula(&self, buf: &mut dyn fmt::Write) -> fmt::Result {
                    write!(buf, "{}", self)
                }
            }
        };
    }

    value!(i8);
    value!(i16);
    value!(i32);
    value!(i64);
    value!(i128);
    value!(isize);
    value!(u8);
    value!(u16);
    value!(u32);
    value!(u64);
    value!(u128);
    value!(usize);
    value!(f32);
    value!(f64);

    impl Formula for &str {
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

    impl Formula for String {
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
}
