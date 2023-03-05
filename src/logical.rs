use crate::{func1, func2, func3, Any, FAny, FLogical, Logical, Sequence};

///  Compute logical AND of all parameters.
#[inline]
pub fn and(list: impl Sequence) -> FLogical {
    FLogical(func1("AND", &list))
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_then_else(condition: impl Logical, if_true: impl Any, if_false: impl Any) -> FAny {
    FAny(func3("IF", &condition, &if_true, &if_false))
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_then(condition: impl Logical, if_true: impl Any) -> FAny {
    FAny(func2("IF", &condition, &if_true))
}

/// Return one of two values, depending on a condition.
#[inline]
pub fn if_else(condition: impl Logical, if_false: impl Any) -> FAny {
    FAny(func3("IF", &condition, &(), &if_false))
}

/// Return X unless it is an Error, in which case return an alternative value.
#[inline]
pub fn iferror(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func2("IFERROR", &x, &alternative))
}

/// Return X unless it is #N/A, in which case return an alternative value.
#[inline]
pub fn ifna(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func2("IFNA", &x, &alternative))
}

/// Compute logical NOT.
#[inline]
pub fn not(logical: impl Logical) -> FLogical {
    FLogical(func1("NOT", &logical))
}

/// Compute logical OR of all parameters.
#[inline]
pub fn or(logical: impl Sequence) -> FLogical {
    FLogical(func1("OR", &logical))
}

/// Compute logical OR of all parameters.
#[inline]
pub fn xor(logical: impl Sequence) -> FLogical {
    FLogical(func1("XOR", &logical))
}
