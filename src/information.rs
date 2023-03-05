use crate::{
    create_param, func, func0, func1, func2, func3, param_assume_init, Any, Criterion, FCriterion,
    FLogical, FNumber, FReference, FText, Number, Reference, Sequence, Text,
};

///  Returns the number of areas in a given list of references.
#[inline]
pub fn areas(refs: impl Reference) -> FNumber {
    FNumber(func1("AREAS", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_col(refs: impl Reference) -> FNumber {
    FNumber(func2("CELL", &"COL", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_row(refs: impl Reference) -> FNumber {
    FNumber(func2("CELL", &"ROW", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_sheet(refs: impl Reference) -> FText {
    FText(func2("CELL", &"SHEET", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_address(refs: impl Reference) -> FReference {
    FReference(func2("CELL", &"ADDRESS", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_filename(refs: impl Reference) -> FText {
    FText(func2("CELL", &"FILENAME", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_contents(refs: impl Reference) -> FText {
    FText(func2("CELL", &"CONTENTS", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_colored(refs: impl Reference) -> FLogical {
    FLogical(func2("CELL", &"COLOR", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_format(refs: impl Reference) -> FText {
    FText(func2("CELL", &"FORMAT", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_type(refs: impl Reference) -> FText {
    FText(func2("CELL", &"TYPE", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_width(refs: impl Reference) -> FNumber {
    FNumber(func2("CELL", &"WIDTH", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_protect(refs: impl Reference) -> FLogical {
    FLogical(func2("CELL", &"PROTECT", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_parentheses(refs: impl Reference) -> FLogical {
    FLogical(func2("CELL", &"PARENTHESES", &refs))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_prefix(refs: impl Reference) -> FText {
    FText(func2("CELL", &"PREFIX", &refs))
}

/// Returns the column number(s) of a reference
#[inline]
pub fn column(refs: impl Reference) -> FNumber {
    FNumber(func1("COLUMN", &refs))
}

/// Returns the number of columns in a given range.
#[inline]
pub fn columns(refs: impl Reference) -> FNumber {
    FNumber(func1("COLUMNS", &refs))
}

/// Count the number of Numbers provided
#[inline]
pub fn count(seq: impl Sequence) -> FNumber {
    FNumber(func1("COUNT", &seq))
}

/// Count the number of non-empty values.
#[inline]
pub fn counta(seq: impl Sequence) -> FNumber {
    FNumber(func1("COUNTA", &seq))
}

/// Count the number of blank cells
#[inline]
pub fn countblank(seq: impl Sequence) -> FNumber {
    FNumber(func1("COUNTBLANK", &seq))
}

/// Count the number of cells in a range that meet a criteria.
#[inline]
pub fn countif(seq: impl Sequence, criterion: impl Criterion) -> FNumber {
    FNumber(func2("COUNTBLANK", &seq, &criterion))
}

/// Count the number of cells that meet multiple criteria in multiple ranges.
#[inline]
pub fn countifs(list: &[(FReference, FCriterion)]) -> FNumber {
    let mut param = create_param(list.len() * 2);
    for (i, (r, c)) in list.iter().enumerate() {
        param[2 * i].write(r);
        param[2 * i + 1].write(c);
    }
    let param = unsafe { param_assume_init(param) };

    FNumber(func("COUNTIFS", param.as_ref()))
}

/// Returns Number representing the specific Error type
#[inline]
pub fn error_type(error: impl Any) -> FNumber {
    FNumber(func1("ERROR.TYPE", &error))
}

///  Returns formula at given reference as text.
#[inline]
pub fn cell_formula(refs: impl Reference) -> FText {
    FText(func1("FORMULA", &refs))
}

/// Returns information about the environment
#[inline]
pub fn info_directory() -> FText {
    FText(func1("INFO", &"directory"))
}

/// Returns information about the environment
#[inline]
pub fn info_memavail() -> FNumber {
    FNumber(func1("INFO", &"memavail"))
}

/// Returns information about the environment
#[inline]
pub fn info_memused() -> FNumber {
    FNumber(func1("INFO", &"memused"))
}

/// Returns information about the environment
#[inline]
pub fn info_numfile() -> FNumber {
    FNumber(func1("INFO", &"numfile"))
}

/// Returns information about the environment
#[inline]
pub fn info_osversion() -> FText {
    FText(func1("INFO", &"osversion"))
}

/// Returns information about the environment
#[inline]
pub fn info_origin() -> FText {
    FText(func1("INFO", &"origin"))
}

/// Returns information about the environment
#[inline]
pub fn info_recal() -> FText {
    FText(func1("INFO", &"recalc"))
}

/// Returns information about the environment
#[inline]
pub fn info_release() -> FText {
    FText(func1("INFO", &"release"))
}

/// Returns information about the environment
#[inline]
pub fn info_system() -> FText {
    FText(func1("INFO", &"system"))
}

/// Returns information about the environment
#[inline]
pub fn info_totmem() -> FNumber {
    FNumber(func1("INFO", &"totmem"))
}

/// Return TRUE if the referenced cell is blank, else return FALSE
#[inline]
pub fn isblank(x: impl Any) -> FLogical {
    FLogical(func1("ISBLANK", &x))
}

/// Return TRUE if the parameter has type Error and is not #N/A, else return FALSE.
#[inline]
pub fn iserr(x: impl Any) -> FLogical {
    FLogical(func1("ISERR", &x))
}

/// Return TRUE if the parameter has type Error, else return FALSE.
#[inline]
pub fn iserror(x: impl Any) -> FLogical {
    FLogical(func1("ISERROR", &x))
}

/// Return TRUE if the value is even, else return FALSE
#[inline]
pub fn iseven(number: impl Number) -> FLogical {
    FLogical(func1("ISEVEN", &number))
}

/// Return TRUE if the reference refers to a formula, else return FALSE.
#[inline]
pub fn isformula(refs: impl Reference) -> FLogical {
    FLogical(func1("ISFORMULA", &refs))
}

/// Return TRUE if the parameter has type Logical, else return FALSE
#[inline]
pub fn islogical(x: impl Any) -> FLogical {
    FLogical(func1("ISLOGICAL", &x))
}

/// Return TRUE if the parameter has type Error and is #N/A, else return FALSE.
#[inline]
pub fn isna(x: impl Any) -> FLogical {
    FLogical(func1("ISNA", &x))
}

///  Return TRUE if the parameter does not have type Text, else return FALSE.
#[inline]
pub fn isnontext(x: impl Any) -> FLogical {
    FLogical(func1("ISNONTEXT", &x))
}

///  Return TRUE if the parameter has type Number, else return FALSE
#[inline]
pub fn isnumber(x: impl Any) -> FLogical {
    FLogical(func1("ISNUMBER", &x))
}

/// Return TRUE if the value is even, else return FALSE.
#[inline]
pub fn isodd(x: impl Any) -> FLogical {
    FLogical(func1("ISODD", &x))
}

/// Return TRUE if the parameter is of type reference, else return FALSE.
#[inline]
pub fn isref(x: impl Any) -> FLogical {
    FLogical(func1("ISREF", &x))
}

/// Return TRUE if the parameter has type Text, else return FALSE.
#[inline]
pub fn istext(x: impl Any) -> FLogical {
    FLogical(func1("ISTEXT", &x))
}

/// Return the number of a value.
#[inline]
pub fn n(x: impl Any) -> FLogical {
    FLogical(func1("N", &x))
}

/// Return the constant Error value #N/A.
#[inline]
pub fn na() -> FLogical {
    FLogical(func0("NA"))
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue(text: impl Text) -> FNumber {
    FNumber(func1("NUMBERVALUE", &text))
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue_dec(text: impl Text, dec_sep: impl Text) -> FNumber {
    FNumber(func2("NUMBERVALUE", &text, &dec_sep))
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue_grp(text: impl Text, group_sep: impl Text) -> FNumber {
    FNumber(func3("NUMBERVALUE", &text, &(), &group_sep))
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue_sep(text: impl Text, dec_sep: impl Text, group_sep: impl Text) -> FNumber {
    FNumber(func3("NUMBERVALUE", &text, &dec_sep, &group_sep))
}

/// Returns the row number(s) of a reference.
#[inline]
pub fn row(refs: impl Reference) -> FNumber {
    FNumber(func1("ROW", &refs))
}

///  Returns the number of rows in a given range.
#[inline]
pub fn rows(refs: impl Reference) -> FNumber {
    FNumber(func1("ROWS", &refs))
}

/// Returns the sheet number of the reference or the string representing a sheet name.
#[inline]
pub fn sheet(refs: impl Reference) -> FNumber {
    FNumber(func1("SHEET", &refs))
}

/// Returns the number of sheets in a reference or current document.
#[inline]
pub fn sheets(refs: impl Reference) -> FNumber {
    FNumber(func1("SHEETS", &refs))
}

/// Returns a number indicating the type of the provided value.
#[inline]
pub fn value_type(a: impl Any) -> FNumber {
    FNumber(func1("TYPE", &a))
}

/// Convert text to number
#[inline]
pub fn value(text: impl Text) -> FNumber {
    FNumber(func1("VALUE", &text))
}
