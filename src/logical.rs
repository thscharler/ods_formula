use crate::{func, Any, FAny, FLogical, Logical, Sequence};

///  Compute logical AND of all parameters.
#[inline]
pub fn and(list: impl Sequence) -> FLogical {
    FLogical(func("AND", &[&list]))
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_(condition: impl Logical, if_true: Option<impl Any>, if_false: Option<impl Any>) -> FAny {
    FAny(func("IF", &[&condition, &if_true, &if_false]))
}

/// Return X unless it is an Error, in which case return an alternative value.
#[inline]
pub fn iferror(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func("IFERROR", &[&x, &alternative]))
}

/// Return X unless it is #N/A, in which case return an alternative value.
#[inline]
pub fn ifna(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func("IFNA", &[&x, &alternative]))
}

/// Compute logical NOT.
#[inline]
pub fn not(logical: impl Logical) -> FLogical {
    FLogical(func("NOT", &[&logical]))
}

/// Compute logical OR of all parameters.
#[inline]
pub fn or(logical: impl Sequence) -> FLogical {
    FLogical(func("OR", &[&logical]))
}

/// Compute logical OR of all parameters.
#[inline]
pub fn xor(logical: impl Sequence) -> FLogical {
    FLogical(func("XOR", &[&logical]))
}
