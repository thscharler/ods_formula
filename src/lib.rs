use spreadsheet_ods::{CellRange, CellRef};
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, BitAnd, BitXor, Div, Mul, Neg, Sub};

mod bitop;
mod complex;
mod database;
mod date;
mod extaccess;
mod information;
mod logical;
mod lookup;
mod math;
mod matrix;

pub use bitop::bitand;
pub use complex::*;
pub use database::*;
pub use date::*;
pub use extaccess::*;
pub use information::*;
pub use logical::*;
pub use lookup::*;
pub use math::*;
pub use matrix::*;

/// The traits for this crate.
/// And the function p() for parentheses.
pub mod prelude {
    pub use super::parentheses as p;
    pub use super::{
        Any, Criterion, DateTimeParam, Field, Logical, Matrix, Number, Reference, Scalar, Sequence,
        Text, TextOrNumber,
    };
    pub use super::{AnyOp, LogicalOp, NumberOp, ReferenceOp, TextOp};
}

/// Base trait for output to a String.
pub trait Any {
    fn formula(&self, buf: &mut String);
}
/// A number-like parameter. This is also used for date, time etc.
pub trait Number: Any {
    /// Convert to FNumber.
    fn n(&self) -> FNumber {
        let mut buf = String::new();
        let _ = self.formula(&mut buf);
        FNumber(buf)
    }
}
/// A text-like parameter.
pub trait Text: Any {}
/// A logical parameter.
pub trait Logical: Any {}
/// A reference-like parameter.
pub trait Reference: Any {}
/// A matrix or array as parameter.
pub trait Matrix: Any {}
/// A filter/search criterion
pub trait Criterion: Any {}
/// A sequence of values.
pub trait Sequence: Any {}
/// Text or a number.
pub trait TextOrNumber: Any {}
/// A single scalar value.
pub trait Scalar: Any {}
/// Field denominator for a database.
pub trait Field: Any {}
/// A date or time-like parameter.
pub trait DateTimeParam: Any {}

/// Alias for Matrix
pub use Matrix as Array;
/// Alias for a cell reference. A cell range containing headers and a data set.
pub use Reference as Database;
/// Alias for a cell reference. A cell range containing headers and filters.
pub use Reference as Criteria;

/// Comparision operators
pub trait AnyOp<T> {
    /// equal
    fn eq<U: Any>(&self, other: U) -> FLogical;
    /// not equal
    fn ne<U: Any>(&self, other: U) -> FLogical;
    /// less than
    fn lt<U: Any>(&self, other: U) -> FLogical;
    /// less than or equal
    fn le<U: Any>(&self, other: U) -> FLogical;
    /// greater than
    fn gt<U: Any>(&self, other: U) -> FLogical;
    /// greater than or equal
    fn ge<U: Any>(&self, other: U) -> FLogical;
}
/// Operations on number-like values.
pub trait NumberOp<T> {
    /// add
    fn add<U: Number>(&self, other: U) -> FNumber;
    /// subtract
    fn sub<U: Number>(&self, other: U) -> FNumber;
    /// multiply
    fn mul<U: Number>(&self, other: U) -> FNumber;
    /// divide
    fn div<U: Number>(&self, other: U) -> FNumber;
    /// exponential
    fn pow<U: Number>(&self, other: U) -> FNumber;
    /// as percentage
    fn percent(&self) -> FNumber;
}
/// Operations on text-like values.
pub trait TextOp<T> {
    /// concat text
    fn concat<U: Text>(&self, other: U) -> FText;
}
/// Operations on boolean-like values.
pub trait LogicalOp<T> {
    /// and
    fn and<U: Logical>(&self, other: U) -> FLogical;
    /// or
    fn or<U: Logical>(&self, other: U) -> FLogical;
    /// xor
    fn xor<U: Logical>(&self, other: U) -> FLogical;
}
/// Operations on references.
pub trait ReferenceOp<T> {
    /// intersection of references
    fn intersect<U: Reference>(&self, other: U) -> FReference;
    /// concatenation of references
    fn refcat<U: Reference>(&self, other: U) -> FReference;
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

impl<T: Any> AnyOp<T> for T {
    fn eq<U: Any>(&self, other: U) -> FLogical {
        eq(self, other)
    }

    fn ne<U: Any>(&self, other: U) -> FLogical {
        ne(self, other)
    }

    fn lt<U: Any>(&self, other: U) -> FLogical {
        lt(self, other)
    }

    fn le<U: Any>(&self, other: U) -> FLogical {
        le(self, other)
    }

    fn gt<U: Any>(&self, other: U) -> FLogical {
        gt(self, other)
    }

    fn ge<U: Any>(&self, other: U) -> FLogical {
        ge(self, other)
    }
}

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

    fn pow<U: Number>(&self, other: U) -> FNumber {
        pow(self, other)
    }

    fn percent(&self) -> FNumber {
        percent(self)
    }
}

impl<T: Text> TextOp<T> for T {
    fn concat<U: Text>(&self, other: U) -> FText {
        concat(self, other)
    }
}

impl<T: Logical> LogicalOp<T> for T {
    fn and<U: Logical>(&self, other: U) -> FLogical {
        and((self, other))
    }

    fn or<U: Logical>(&self, other: U) -> FLogical {
        or((self, other))
    }

    fn xor<U: Logical>(&self, other: U) -> FLogical {
        xor((self, other))
    }
}

impl<T: Reference> ReferenceOp<T> for T {
    fn intersect<U: Reference>(&self, other: U) -> FReference {
        intersect(self, other)
    }
    fn refcat<U: Reference>(&self, other: U) -> FReference {
        refcat(self, other)
    }
}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

/// Any value.
#[derive(Debug)]
pub struct FAny(String);
impl Display for FAny {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FAny {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Number for FAny {}
impl Text for FAny {}
impl Logical for FAny {}
impl Sequence for FAny {}
impl TextOrNumber for FAny {}
impl Field for FAny {}
impl Scalar for FAny {}
impl DateTimeParam for FAny {}

/// Number value.
#[derive(Debug)]
pub struct FNumber(String);
impl Display for FNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FNumber {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Number for FNumber {}
impl Logical for FNumber {}
impl Sequence for FNumber {}
impl TextOrNumber for FNumber {}
impl Field for FNumber {}
impl Scalar for FNumber {}
impl DateTimeParam for FNumber {}

/// Text value.
#[derive(Debug)]
pub struct FText(String);
impl Display for FText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FText {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Text for FText {}
impl Sequence for FText {}
impl TextOrNumber for FText {}
impl Field for FText {}
impl Scalar for FText {}
impl DateTimeParam for FText {}

/// Logical value.
#[derive(Debug)]
pub struct FLogical(String);
impl Display for FLogical {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FLogical {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Logical for FLogical {}
impl Number for FLogical {}
impl Sequence for FLogical {}
impl Scalar for FLogical {}
impl TextOrNumber for FLogical {}

/// Matrix value.
#[derive(Debug)]
pub struct FMatrix(String);
impl Display for FMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FMatrix {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Matrix for FMatrix {}

/// Reference value.
#[derive(Debug)]
pub struct FReference(String);
impl Display for FReference {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Any for FReference {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }
}
impl Reference for FReference {}
impl Number for FReference {}
impl Text for FReference {}
impl Logical for FReference {}
impl Matrix for FReference {}
impl Sequence for FReference {}
impl TextOrNumber for FReference {}
impl Field for FReference {}
impl Scalar for FReference {}
impl DateTimeParam for FReference {}

/// Filter criteria.
#[derive(Debug)]
pub enum CriterionCmp {
    Cmp,
    Eq,
    Ne,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl Display for CriterionCmp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CriterionCmp::Cmp => write!(f, ""),
            CriterionCmp::Eq => write!(f, "="),
            CriterionCmp::Ne => write!(f, "<>"),
            CriterionCmp::Lt => write!(f, "<"),
            CriterionCmp::Gt => write!(f, ">"),
            CriterionCmp::LtEq => write!(f, "<="),
            CriterionCmp::GtEq => write!(f, ">="),
        }
    }
}

/// Filter/search
#[derive(Debug)]
pub struct FCriterion(String);
impl FCriterion {
    pub fn new<T: Any>(op: CriterionCmp, f: T) -> Self {
        let mut buf = String::new();
        (op, f).formula(&mut buf);
        Self(buf)
    }
}

impl Display for FCriterion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Any for FCriterion {
    fn formula(&self, buf: &mut String) {
        buf.push_str(self.0.as_str());
    }
}
impl Criterion for FCriterion {}

impl<A: Any> Any for (CriterionCmp, A) {
    fn formula(&self, buf: &mut String) {
        let _ = write!(buf, "\"{}\"", self.0);
        buf.push('&');
        self.1.formula(buf);
    }
}
impl<A: Any> Criterion for (CriterionCmp, A) {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

/// Creates a formula from any formula expression.
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

#[inline]
fn prefix<'a, A: Any>(op: &str, a: A) -> String {
    let mut buf = String::new();
    buf.push_str(op);
    a.formula(&mut buf);
    buf
}

#[inline]
fn postfix<'a, A: Any>(a: A, op: &str) -> String {
    let mut buf = String::new();
    a.formula(&mut buf);
    buf.push_str(op);
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
impl<T: Matrix + Any + ?Sized> Matrix for &T {}
impl<T: TextOrNumber + Any + ?Sized> TextOrNumber for &T {}
impl<T: Field + Any + ?Sized> Field for &T {}
impl<T: DateTimeParam + Any + ?Sized> DateTimeParam for &T {}

impl<T: Any + Sized> Any for Option<T> {
    fn formula(&self, buf: &mut String) {
        if let Some(v) = self {
            v.formula(buf);
        }
    }
}
impl<T: Number + Any + Sized> Number for Option<T> {}
impl<T: Text + Any + Sized> Text for Option<T> {}
impl<T: Logical + Any + Sized> Logical for Option<T> {}
impl<T: Reference + Any + Sized> Reference for Option<T> {}
impl<T: Criterion + Any + Sized> Criterion for Option<T> {}
impl<T: Sequence + Any + Sized> Sequence for Option<T> {}
impl<T: Matrix + Any + Sized> Matrix for Option<T> {}
impl<T: TextOrNumber + Any + Sized> TextOrNumber for Option<T> {}
impl<T: Field + Any + Sized> Field for Option<T> {}
impl<T: DateTimeParam + Any + Sized> DateTimeParam for Option<T> {}

impl<T: Any, const N: usize, const M: usize> Any for [[T; M]; N] {
    fn formula(&self, buf: &mut String) {
        buf.push('{');
        for (i, r) in self.iter().enumerate() {
            if i > 0 {
                buf.push('|');
            }
            for (j, c) in r.iter().enumerate() {
                if j > 0 {
                    buf.push(';');
                }
                c.formula(buf);
            }
        }
        buf.push('}');
    }
}
impl<T: Any, const N: usize, const M: usize> Matrix for [[T; M]; N] {}
impl<T: Any, const N: usize, const M: usize> Sequence for [[T; M]; N] {}

impl Any for () {
    fn formula(&self, _buf: &mut String) {}
}

impl<T: Any> Any for Vec<T> {
    fn formula(&self, buf: &mut String) {
        for (i, v) in self.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            v.formula(buf);
        }
    }
}
impl<T: Any> Sequence for Vec<T> {}

macro_rules! tup {
    ( $tzero:ident $($tname:tt $tnum:tt)* ) => {
        impl<$tzero: Any, $($tname: Any,)*> Any for ($tzero, $($tname,)*) {
            fn formula(&self, buf: &mut String) {
                self.0.formula(buf);
                $(
                    buf.push(';');
                    self . $tnum .formula(buf);
                )*
            }
        }
        impl<$tzero: Any, $($tname: Any,)*> Sequence for ($tzero, $($tname,)*) {}
    }
}

tup!(A);
tup!(A B 1 );
tup!(A B 1 C 2 );
tup!(A B 1 C 2 D 3);
tup!(A B 1 C 2 D 3 E 4);
tup!(A B 1 C 2 D 3 E 4 F 5);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 );
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18 T 19);
tup!(A B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9 K 10 L 11 M 12 N 13 O 14 P 15 Q 16 R 17 S 18 T 19 U 20);

/// An expression in parentheses.
/// Use p() / parentheses() to create one.
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
impl<A: Field> Field for FParentheses<A> {}
impl<A: DateTimeParam> DateTimeParam for FParentheses<A> {}

/// Creates an expression in parentheses. Aliased as p().
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
        impl Field for $t {}
        impl Scalar for $t {}
        impl DateTimeParam for $t {}
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
impl Scalar for bool {}
impl Sequence for bool {}

impl Any for &str {
    fn formula(&self, buf: &mut String) {
        if self.contains('"') {
            buf.push('"');
            for (i, s) in self.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(s);
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
impl Field for &str {}
impl Scalar for &str {}
impl DateTimeParam for &str {}

impl Any for String {
    fn formula(&self, buf: &mut String) {
        if self.contains('"') {
            buf.push('"');
            for (i, s) in self.split('"').enumerate() {
                if i > 0 {
                    buf.push_str("\"\"");
                }
                buf.push_str(s);
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
impl Field for String {}
impl Scalar for String {}
impl DateTimeParam for String {}

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
impl Field for CellRef {}
impl Scalar for CellRef {}
impl DateTimeParam for CellRef {}

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
impl Field for CellRange {}
impl Scalar for CellRange {}
impl DateTimeParam for CellRange {}

// -----------------------------------------------------------------------
// -----------------------------------------------------------------------

/// Adds two numbers. Also available as postfix add() and as operator +.
pub fn add<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "+", b))
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

    fn add(self, rhs: A) -> Self::Output {
        FNumber(infix(self, "+", rhs))
    }
}

/// Subtracts two numbers. Also available as postfix sub() and as operator -.
pub fn sub<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "-", b))
}
impl<'a, A: Number> Sub<A> for FNumber {
    type Output = FNumber;

    fn sub(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('-');
        let _ = rhs.formula(buf);
        self
    }
}
impl<A: Number> Sub<A> for FParentheses<A> {
    type Output = FNumber;

    fn sub(self, rhs: A) -> Self::Output {
        FNumber(infix(self, "-", rhs))
    }
}

/// Multiplies to numbers. Also available as postfix mul() and as operator *;
pub fn mul<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "*", b))
}
impl<'a, A: Number> Mul<A> for FNumber {
    type Output = FNumber;

    fn mul(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('*');
        let _ = rhs.formula(buf);
        self
    }
}
impl<A: Number> Mul<A> for FParentheses<A> {
    type Output = FNumber;

    fn mul(self, rhs: A) -> Self::Output {
        FNumber(infix(self, "*", rhs))
    }
}

/// Divides to numbers. Also available as postfix div() and as operator /.
pub fn div<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "/", b))
}
impl<'a, A: Number> Div<A> for FNumber {
    type Output = FNumber;

    fn div(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('/');
        let _ = rhs.formula(buf);
        self
    }
}
impl<A: Number> Div<A> for FParentheses<A> {
    type Output = FNumber;

    fn div(self, rhs: A) -> Self::Output {
        FNumber(infix(self, "/", rhs))
    }
}

/// Exponential function. Also available as postfix pow() and as operator ^.
pub fn pow<'a, A: Number, B: Number>(a: A, b: B) -> FNumber {
    FNumber(infix(a, "^", b))
}
/// Not bitwise xor but exponential.
impl<'a, A: Number> BitXor<A> for FNumber {
    type Output = FNumber;

    fn bitxor(mut self, rhs: A) -> Self::Output {
        let buf = &mut self.0;
        buf.push('^');
        let _ = rhs.formula(buf);
        self
    }
}
/// Not bitwise xor but exponential.
impl<A: Number> BitXor<A> for FParentheses<A> {
    type Output = FNumber;

    fn bitxor(self, rhs: A) -> Self::Output {
        FNumber(infix(self, "^", rhs))
    }
}

/// Negates as number. Also available as prefix operator -.
pub fn neg<'a, A: Number>(a: A) -> FNumber {
    FNumber(prefix("-", a))
}
impl Neg for FNumber {
    type Output = FNumber;

    fn neg(mut self) -> Self::Output {
        let buf = &mut self.0;
        buf.insert(0, '-');
        self
    }
}
impl<A: Number> Neg for FParentheses<A> {
    type Output = FNumber;

    fn neg(self) -> Self::Output {
        FNumber(prefix("-", self.0))
    }
}

/// equal
pub fn eq<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, "=", b))
}

/// inequal
pub fn ne<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, "<>", b))
}

/// less than
pub fn lt<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, "<", b))
}

/// less than or equal
pub fn le<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, "<=", b))
}

/// greater than
pub fn gt<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, ">", b))
}

/// greater than or equal
pub fn ge<'a, A: Any, B: Any>(a: A, b: B) -> FLogical {
    FLogical(infix(a, ">=", b))
}

/// percentage. Also available as postfix percent()
pub fn percent<'a, A: Number>(a: A) -> FNumber {
    FNumber(postfix(a, "%"))
}

/// concatenates two strings. Also available as postfix concat() and as operator &.
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

#[macro_export]
macro_rules! cell {
    ($row:expr, $col:expr) => {
        CellRef::local($row, $col)
    };
    ($table:expr => $row:expr, $col:expr) => {
        CellRef::remote($table, $row, $col)
    };
}

#[macro_export]
macro_rules! range {
    ($row:expr, $col:expr, $row_to:expr, $col_to:expr) => {
        CellRange::local($row, $col, $row_to, $col_to)
    };
    ($row:expr, $col:expr; + $row_delta:expr, $col_delta:expr) => {
        CellRange::origin_span($row, $col, ($row_delta, $col_delta))
    };
    ($table:expr => $row:expr, $col:expr, $row_to:expr, $col_to:expr) => {
        CellRange::remote($table, $row, $col, $row_to, $col_to)
    };
    ($table:expr => $row:expr, $col:expr; + $row_delta:expr, $col_delta:expr) => {
        CellRange::remote($table, $row, $col, $row + $row_delta, $col + $col_delta)
    };
}

pub fn test0() {
    let _ = cosh(99);
}
