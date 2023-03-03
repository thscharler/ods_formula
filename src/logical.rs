use crate::{func, Any, FAny, FLogical, Logical, Sequence};

///  Compute logical AND of all parameters.
pub fn and(list: impl Sequence) -> FLogical {
    FLogical(func("AND", &[&list]))
}

/// Return one of two values, depending on a condition.
pub fn if_(condition: impl Logical, if_true: Option<impl Any>, if_false: Option<impl Any>) -> FAny {
    FAny(func("IF", &[&condition, &if_true, &if_false]))
}

/// Return X unless it is an Error, in which case return an alternative value.
pub fn iferror(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func("IFERROR", &[&x, &alternative]))
}

/// Return X unless it is #N/A, in which case return an alternative value.
pub fn ifna(x: impl Any, alternative: impl Any) -> FAny {
    FAny(func("IFNA", &[&x, &alternative]))
}

/// Compute logical NOT.
pub fn not(logical: impl Logical) -> FLogical {
    FLogical(func("NOT", &[&logical]))
}

/// Compute logical OR of all parameters.
pub fn or(logical: impl Sequence) -> FLogical {
    FLogical(func("OR", &[&logical]))
}

/// Compute logical OR of all parameters.
pub fn xor(logical: impl Sequence) -> FLogical {
    FLogical(func("XOR", &[&logical]))
}
