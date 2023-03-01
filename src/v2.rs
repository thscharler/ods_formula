use spreadsheet_ods::{CellRange, CellRef};
use std::fmt;
use std::fmt::Write;
use std::ops::{Add, BitAnd, Rem};

pub mod prelude {
    pub use super::parentheses as p;
    pub use super::{Any, Criterion, Logical, Number, Reference, Text};
    pub use super::{CriterionOp, LogicalOp, NumberOp, ReferenceOp, TextOp};
}

pub trait Any {
    fn formula(&self, buf: &mut String);
}
pub trait Number: Any {
    fn n(&self) -> FNumber {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FNumber(buf)
    }
}
pub trait Text: Any {
    fn t(&self) -> FText {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FText(buf)
    }
}
pub trait Logical: Any {
    fn b(&self) -> FLogical {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FLogical(buf)
    }
}
pub trait Reference: Any {
    fn r(&self) -> FReference {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FReference(buf)
    }
}
pub trait Criterion: Any {}
pub trait Sequence: Any {}
pub trait TextOrNumber: Any {}

pub trait NumberOp<T> {
    fn add<U: Number>(&self, other: U) -> FNumber;
    fn sub<U: Number>(&self, other: U) -> FNumber;
    fn mul<U: Number>(&self, other: U) -> FNumber;
    fn div<U: Number>(&self, other: U) -> FNumber;
}

pub trait TextOp<T> {
    fn concat<U: Text>(&self, other: U) -> FText;
}

pub trait LogicalOp<T> {}

pub trait ReferenceOp<T> {
    fn intersect<U: Reference>(&self, other: U) -> FReference;
    fn refcat<U: Reference>(&self, other: U) -> FReference;
}

pub trait CriterionOp<T> {}
pub trait SequenceOp<T> {}
pub trait TextOrNumberOp<T> {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

impl<T: Number> NumberOp<T> for T {
    fn add<U: Number>(&self, other: U) -> FNumber {
        add(self, other)
    }

    fn sub<U: Number>(&self, other: U) -> FNumber {
        sub(self, other)
    }

    fn mul<U: Number>(&self, other: U) -> FNumber {
        mul(self, other)
    }

    fn div<U: Number>(&self, other: U) -> FNumber {
        div(self, other)
    }
}

impl<T: Text> TextOp<T> for T {
    fn concat<U: Text>(&self, other: U) -> FText {
        concat(self, other)
    }
}

impl<T: Logical> LogicalOp<T> for T {}

impl<T: Reference> ReferenceOp<T> for T {
    fn intersect<U: Reference>(&self, other: U) -> FReference {
        intersect(self, other)
    }
    fn refcat<U: Reference>(&self, other: U) -> FReference {
        refcat(self, other)
    }
}

impl<T: Criterion> CriterionOp<T> for T {}
impl<T: Sequence> SequenceOp<T> for T {}
impl<T: TextOrNumber> TextOrNumberOp<T> for T {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub struct FNumber(String);
impl Any for FNumber {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Number for FNumber {}
impl Logical for FNumber {}
impl Sequence for FNumber {}
impl TextOrNumber for FNumber {}

pub struct FText(String);
impl Any for FText {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Text for FText {}
impl Sequence for FText {}
impl TextOrNumber for FText {}

pub struct FLogical(String);
impl Any for FLogical {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Logical for FLogical {}
impl Number for FLogical {}
impl Sequence for FLogical {}
impl TextOrNumber for FLogical {}

pub struct FReference(String);
impl Any for FReference {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Reference for FReference {}
impl Number for FReference {}
impl Text for FReference {}
impl Logical for FReference {}
impl Sequence for FReference {}
impl TextOrNumber for FReference {}

pub enum FCriterion<F> {
    V(F),
    Eq(F),
    Ne(F),
    Lt(F),
    Gt(F),
    LtEq(F),
    GtEq(F),
}

impl<F: Any> Any for FCriterion<F> {
    fn formula(&self, buf: &mut String) {
        match self {
            FCriterion::V(f) => {
                f.formula(buf);
            }
            FCriterion::Eq(f) => {
                buf.push_str("\"=\"&");
                f.formula(buf);
            }
            FCriterion::Ne(f) => {
                buf.push_str("\"<>\"&");
                f.formula(buf);
            }
            FCriterion::Lt(f) => {
                buf.push_str("\"<\"&");
                f.formula(buf);
            }
            FCriterion::Gt(f) => {
                buf.push_str("\">\"&");
                f.formula(buf);
            }
            FCriterion::LtEq(f) => {
                buf.push_str("\"<=\"&");
                f.formula(buf);
            }
            FCriterion::GtEq(f) => {
                buf.push_str("\">=\"&");
                f.formula(buf);
            }
        }
    }
}
impl<F: Any> Criterion for FCriterion<F> {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

/// formula
pub fn formula<T: Any>(f: T) -> String {
    let mut buf = String::new();
    buf.push_str("of=");
    let _ = f.formula(&mut buf);
    buf
}

#[inline]
fn func(name: &str, args: &[&dyn Any]) -> String {
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
    buf
}

#[inline]
fn infix<'a, A: Any, B: Any>(a: A, op: &str, b: B) -> String {
    let mut buf = String::new();
    a.formula(&mut buf);
    buf.push_str(op);
    b.formula(&mut buf);
    buf
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
impl<T: Sequence + Any + ?Sized> Sequence for &T {}
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
impl<T: Any, const N: usize> Sequence for [T; N] {}

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
impl<T: Sequence> Sequence for (T,) {}
impl<T: TextOrNumber> TextOrNumber for (T,) {}

pub struct FParentheses<A>(A);
impl<A: Any> Any for FParentheses<A> {
    fn formula(&self, buf: &mut String) {
        buf.push('(');
        self.0.formula(buf);
        buf.push(')');
    }
}
impl<A: Number> Number for FParentheses<A> {}
impl<A: Text> Text for FParentheses<A> {}
impl<A: Logical> Logical for FParentheses<A> {}
impl<A: Reference> Reference for FParentheses<A> {}
impl<A: Sequence> Sequence for FParentheses<A> {}
impl<A: TextOrNumber> TextOrNumber for FParentheses<A> {}

pub fn parentheses<A: Any>(a: A) -> FParentheses<A> {
    FParentheses(a)
}

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
        impl Sequence for $t {}
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
impl Sequence for bool {}

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
impl Sequence for &str {}
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
impl Sequence for String {}
impl TextOrNumber for String {}

impl Any for CellRef {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.to_formula().as_str())
    }
}
impl Reference for CellRef {}
impl Number for CellRef {}
impl Text for CellRef {}
impl Logical for CellRef {}
impl Sequence for CellRef {}
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
impl Sequence for CellRange {}
impl TextOrNumber for CellRange {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub fn add<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "+", b))
}
pub fn sub<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "-", b))
}
pub fn mul<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "*", b))
}
pub fn div<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "/", b))
}

impl<'a, A: Number> Add<A> for FNumber {
    type Output = FNumber;

    fn add(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('+');
        let _ = rhs.formula(buf);
        self
    }
}
impl<A: Number> Add<A> for FParentheses<A> {
    type Output = FNumber;

    fn add(mut self, rhs: A) -> Self::Output {
        FNumber(infix(self, "+", rhs))
    }
}

pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, "=", b))
}

pub fn concat<'a, A: Text, B: Text>(a: A, b: B) -> FText {
    FText(infix(a, "&", b))
}

impl<'a, A: Text> BitAnd<A> for FText {
    type Output = FText;

    fn bitand(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('&');
        let _ = rhs.formula(buf);
        self
    }
}

pub fn intersect<'a, A: Reference, B: Reference>(a: A, b: B) -> FReference {
    FReference(infix(a, "!", b))
}
pub fn refcat<'a, A: Reference, B: Reference>(a: A, b: B) -> FReference {
    FReference(infix(a, "~", b))
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

pub fn date<'a, D: Number, M: Number, Y: Number>(day: D, month: M, year: Y) -> FNumber {
    FNumber(func("DATE", &[&day, &month, &year]))
}

pub fn date_value<'a, A: Text>(a: A) -> FNumber {
    FNumber(func("DATEVALUE", &[&a]))
}

pub fn count<'a, A: Number>(a: A) -> FNumber {
    FNumber(func("COUNT", &[&a]))
}

pub fn cos<'a, A: Number>(a: A) -> FNumber {
    FNumber(func("COS", &[&a]))
}

pub fn sum<'a, A: Sequence>(a: A) -> FNumber {
    FNumber(func("SUM", &[&a]))
}
