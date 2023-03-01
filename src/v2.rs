use spreadsheet_ods::{CellRange, CellRef};
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;
use std::ops::{Add, BitAnd, Rem};

pub mod prelude {
    pub use super::{Any, Criterion, Logical, Number, Reference, Text};
}

pub trait Any {
    fn formula(&self, buf: &mut String);
}
pub trait Number: Any {
    fn n<'a>(&self) -> FNumber<'a> {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FNumber(Cow::Owned(buf))
    }

    fn add<'a, T: Number>(&self, other: T) -> FNumber<'a> {
        add(self, other)
    }
    fn sub<'a, T: Number>(&self, other: T) -> FNumber<'a> {
        sub(self, other)
    }
    fn mul<'a, T: Number>(&self, other: T) -> FNumber<'a> {
        mul(self, other)
    }
    fn div<'a, T: Number>(&self, other: T) -> FNumber<'a> {
        div(self, other)
    }
}
pub trait Text: Any {
    fn t<'a>(&self) -> FText<'a> {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FText(Cow::Owned(buf))
    }

    fn concat<'a, T: Text>(&self, other: T) -> FText<'a> {
        concat(self, other)
    }
}
pub trait Logical: Any {
    fn b<'a>(&self) -> FLogical<'a> {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FLogical(Cow::Owned(buf))
    }
}
pub trait Reference: Any {
    fn r<'a>(&self) -> FReference<'a> {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FReference(Cow::Owned(buf))
    }

    fn intersect<'a, T: Reference>(&self, other: T) -> FReference<'a> {
        intersect(self, other)
    }
    fn refcat<'a, T: Reference>(&self, other: T) -> FReference<'a> {
        refcat(self, other)
    }
}
pub trait Criterion: Any {}
pub trait TextOrNumber: Any {}

pub struct FNumber<'a>(Cow<'a, str>);
impl<'a> Any for FNumber<'a> {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl<'a> Number for FNumber<'a> {}
impl<'a> Logical for FNumber<'a> {}
impl<'a> TextOrNumber for FNumber<'a> {}

pub struct FText<'a>(Cow<'a, str>);
impl<'a> Any for FText<'a> {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl<'a> Text for FText<'a> {}
impl<'a> TextOrNumber for FText<'a> {}

pub struct FLogical<'a>(Cow<'a, str>);
impl<'a> Any for FLogical<'a> {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl<'a> Logical for FLogical<'a> {}
impl<'a> Number for FLogical<'a> {}
impl<'a> TextOrNumber for FLogical<'a> {}

pub struct FReference<'a>(Cow<'a, str>);
impl<'a> Any for FReference<'a> {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl<'a> Reference for FReference<'a> {}
impl<'a> Number for FReference<'a> {}
impl<'a> Text for FReference<'a> {}
impl<'a> Logical for FReference<'a> {}
impl<'a> TextOrNumber for FReference<'a> {}

pub struct FCriterion<'a>(Cow<'a, str>);
impl<'a> Any for FCriterion<'a> {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl<'a> Criterion for FCriterion<'a> {}

/// formula
pub fn formula<T: Any>(f: T) -> String {
    let mut buf = String::new();
    buf.push_str("of=");
    let _ = f.formula(&mut buf);
    buf
}

#[inline]
fn func<'a>(name: &str, args: &[&dyn Any]) -> Cow<'a, str> {
    let mut buf = String::new();
    buf.push_str(name);
    buf.push('(');
    for (i, v) in args.iter().enumerate() {
        if i > 0 {
            buf.push(';');
        }
        let _ = v.formula(&mut buf);
    }
    buf.push(')');
    Cow::Owned(buf)
}

#[inline]
fn infix<'a, A: Any, B: Any>(a: A, op: &str, b: B) -> Cow<'a, str> {
    let mut buf = String::new();
    a.formula(&mut buf);
    buf.push_str(op);
    b.formula(&mut buf);
    Cow::Owned(buf)
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

impl<T: Any + ?Sized> Any for &T {
    fn formula(&self, buf: &mut String) {
        (*self).formula(buf);
    }
}
impl<T: Number + Any + ?Sized> Number for &T {}
impl<T: Text + Any + ?Sized> Text for &T {}
impl<T: Logical + Any + ?Sized> Logical for &T {}
impl<T: Reference + Any + ?Sized> Reference for &T {}
impl<T: Criterion + Any + ?Sized> Criterion for &T {}
impl<T: TextOrNumber + Any + ?Sized> TextOrNumber for &T {}

impl<T: Any, const N: usize> Any for [T; N] {
    fn formula(&self, buf: &mut String) {
        for (i, r) in self.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            r.formula(buf);
        }
    }
}
impl<T: Reference, const N: usize> Reference for [T; N] {}
impl<T: Number, const N: usize> Number for [T; N] {}
impl<T: Text, const N: usize> Text for [T; N] {}
impl<T: Logical, const N: usize> Logical for [T; N] {}
impl<T: TextOrNumber, const N: usize> TextOrNumber for [T; N] {}

impl<T: Any> Any for (T,) {
    fn formula(&self, buf: &mut String) {
        buf.push('(');
        self.0.formula(buf);
        buf.push(')');
    }
}
impl<T: Number> Number for (T,) {}
impl<T: Text> Text for (T,) {}
impl<T: Logical> Logical for (T,) {}
impl<T: Reference> Reference for (T,) {}
impl<T: Criterion> Criterion for (T,) {}
impl<T: TextOrNumber> TextOrNumber for (T,) {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

macro_rules! value_number {
    ($t:ty) => {
        impl Any for $t {
            fn formula(&self, buf: &mut String) {
                let _ = write!(buf, "{}", self);
            }
        }
        impl Number for $t {}
        impl Logical for $t {}
        impl TextOrNumber for $t {}
    };
}

value_number!(i8);
value_number!(i16);
value_number!(i32);
value_number!(i64);
value_number!(i128);
value_number!(isize);
value_number!(u8);
value_number!(u16);
value_number!(u32);
value_number!(u64);
value_number!(u128);
value_number!(usize);
value_number!(f32);
value_number!(f64);

impl Any for bool {
    fn formula(&self, buf: &mut String) {
        buf.push_str(if *self { "TRUE()" } else { "FALSE()" });
    }
}
impl Logical for bool {}
impl Number for bool {}

impl Any for &str {
    fn formula(&self, buf: &mut String) {
        if self.contains('"') {
            buf.push('"');
            for (i, s) in self.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(self);
            }
            buf.push('"');
        } else {
            buf.push('"');
            buf.push_str(self);
            buf.push('"');
        }
    }
}
impl Text for &str {}
impl TextOrNumber for &str {}

impl Any for String {
    fn formula(&self, buf: &mut String) {
        if self.contains('"') {
            buf.push('"');
            for (i, s) in self.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(self);
            }
            buf.push('"');
        } else {
            buf.push('"');
            buf.push_str(self);
            buf.push('"');
        }
    }
}
impl Text for String {}
impl TextOrNumber for String {}

pub struct Parentheses<A>(A);
impl<A: Any> Any for Parentheses<A> {
    fn formula(&self, buf: &mut String) {
        buf.push('(');
        self.0.formula(buf);
        buf.push(')');
    }
}
impl<A: Number> Number for Parentheses<A> {}
impl<A: Text> Text for Parentheses<A> {}
impl<A: Logical> Logical for Parentheses<A> {}
impl<A: Reference> Reference for Parentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for Parentheses<A> {}

pub fn par<A: Any>(a: A) -> Parentheses<A> {
    Parentheses(a)
}

impl Any for CellRef {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Reference for CellRef {}
impl Number for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl TextOrNumber for CellRef {}

impl Any for CellRange {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Reference for CellRange {}
impl Number for CellRange {}
impl Text for CellRange {}
impl Logical for CellRange {}
impl TextOrNumber for CellRange {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub fn add<'a, A: Number, B: Number>(a: A, b: B) -> FNumber<'a> {
    FNumber(infix(a, "+", b))
}
pub fn sub<'a, A: Number, B: Number>(a: A, b: B) -> FNumber<'a> {
    FNumber(infix(a, "-", b))
}
pub fn mul<'a, A: Number, B: Number>(a: A, b: B) -> FNumber<'a> {
    FNumber(infix(a, "*", b))
}
pub fn div<'a, A: Number, B: Number>(a: A, b: B) -> FNumber<'a> {
    FNumber(infix(a, "/", b))
}

impl<'a, A: Number> Add<A> for FNumber<'a> {
    type Output = FNumber<'a>;

    fn add(mut self, rhs: A) -> Self::Output {
        let buf = self.0.to_mut();
        buf.push('+');
        let _ = rhs.formula(buf);
        self
    }
}
impl<A: Number> Add<A> for Parentheses<A> {
    type Output = FNumber<'a>;

    fn add(mut self, rhs: A) -> Self::Output {
        FNumber(infix(self, "+", rhs))
    }
}

pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> FLogical<'a> {
    FLogical(infix(a, "=", b))
}

pub fn concat<'a, A: Text, B: Text>(a: A, b: B) -> FText<'a> {
    FText(infix(a, "%", b))
}

impl<'a, A: Text> BitAnd<A> for FText<'a> {
    type Output = FText<'a>;

    fn bitand(mut self, rhs: A) -> Self::Output {
        let buf = self.0.to_mut();
        buf.push('%');
        let _ = rhs.formula(buf);
        self
    }
}

pub fn intersect<'a, A: Reference, B: Reference>(a: A, b: B) -> FReference<'a> {
    FReference(infix(a, "!", b))
}
pub fn refcat<'a, A: Reference, B: Reference>(a: A, b: B) -> FReference<'a> {
    FReference(infix(a, "~", b))
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub fn date<'a, D: Number, M: Number, Y: Number>(day: D, month: M, year: Y) -> FNumber<'a> {
    FNumber(func("DATE", &[&day, &month, &year]))
}

pub fn date_value<'a, A: Text>(a: A) -> FNumber<'a> {
    FNumber(func("DATEVALUE", &[&a]))
}

pub fn count<'a, A: Number>(a: A) -> FNumber<'a> {
    FNumber(func("COUNT", &[&a]))
}

pub fn cos<'a, A: Number>(a: A) -> FNumber<'a> {
    FNumber(func("COS", &[&a]))
}

pub fn sum<'a, A: Number>(a: A) -> FNumber<'a> {
    FNumber(func("SUM", &[&a]))
}
