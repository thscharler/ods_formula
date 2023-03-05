use crate::{
    func, Any, Criterion, FCriterion, FLogical, FNumber, FReference, FText, Number, Reference,
    Sequence, Text,
};

///  Returns the number of areas in a given list of references.
#[inline]
pub fn areas(refs: impl Reference) -> FNumber {
    FNumber(func("AREAS", &[&refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_col(refs: impl Reference) -> FNumber {
    FNumber(func("CELL", &[&"COL", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_row(refs: impl Reference) -> FNumber {
    FNumber(func("CELL", &[&"ROW", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_sheet(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"SHEET", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_address(refs: impl Reference) -> FReference {
    FReference(func("CELL", &[&"ADDRESS", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_filename(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"FILENAME", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_contents(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"CONTENTS", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_colored(refs: impl Reference) -> FLogical {
    FLogical(func("CELL", &[&"COLOR", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_format(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"FORMAT", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_type(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"TYPE", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_width(refs: impl Reference) -> FNumber {
    FNumber(func("CELL", &[&"WIDTH", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_protect(refs: impl Reference) -> FLogical {
    FLogical(func("CELL", &[&"PROTECT", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_parentheses(refs: impl Reference) -> FLogical {
    FLogical(func("CELL", &[&"PARENTHESES", &refs]))
}

/// Returns information about position, formatting or contents in a reference.
#[inline]
pub fn cell_prefix(refs: impl Reference) -> FText {
    FText(func("CELL", &[&"PREFIX", &refs]))
}

/// Returns the column number(s) of a reference
#[inline]
pub fn column(refs: impl Reference) -> FNumber {
    FNumber(func("COLUMN", &[&refs]))
}

/// Returns the number of columns in a given range.
#[inline]
pub fn columns(refs: impl Reference) -> FNumber {
    FNumber(func("COLUMNS", &[&refs]))
}

/// Count the number of Numbers provided
#[inline]
pub fn count(seq: impl Sequence) -> FNumber {
    FNumber(func("COUNT", &[&seq]))
}

/// Count the number of non-empty values.
#[inline]
pub fn counta(seq: impl Sequence) -> FNumber {
    FNumber(func("COUNTA", &[&seq]))
}

/// Count the number of blank cells
#[inline]
pub fn countblank(seq: impl Sequence) -> FNumber {
    FNumber(func("COUNTBLANK", &[&seq]))
}

/// Count the number of cells in a range that meet a criteria.
#[inline]
pub fn countif(seq: impl Sequence, criterion: impl Criterion) -> FNumber {
    FNumber(func("COUNTBLANK", &[&seq, &criterion]))
}

/// Count the number of cells that meet multiple criteria in multiple ranges.
#[inline]
pub fn countifs(list: &[(FReference, FCriterion)]) -> FNumber {
    let mut buf = String::new();
    buf.push_str("COUNTIFS");
    buf.push('(');
    for (i, (r, c)) in list.iter().enumerate() {
        if i > 0 {
            buf.push(';');
        }
        r.formula(&mut buf);
        buf.push(';');
        c.formula(&mut buf);
    }
    buf.push(')');
    FNumber(buf)
}

/// Returns Number representing the specific Error type
#[inline]
pub fn error_type(error: impl Any) -> FNumber {
    FNumber(func("ERROR.TYPE", &[&error]))
}

///  Returns formula at given reference as text.
#[inline]
pub fn cell_formula(refs: impl Reference) -> FText {
    FText(func("FORMULA", &[&refs]))
}

/// Returns information about the environment
#[inline]
pub fn info_directory() -> FText {
    FText(func("INFO", &[&"directory"]))
}

/// Returns information about the environment
#[inline]
pub fn info_memavail() -> FNumber {
    FNumber(func("INFO", &[&"memavail"]))
}

/// Returns information about the environment
#[inline]
pub fn info_memused() -> FNumber {
    FNumber(func("INFO", &[&"memused"]))
}

/// Returns information about the environment
#[inline]
pub fn info_numfile() -> FNumber {
    FNumber(func("INFO", &[&"numfile"]))
}

/// Returns information about the environment
#[inline]
pub fn info_osversion() -> FText {
    FText(func("INFO", &[&"osversion"]))
}

/// Returns information about the environment
#[inline]
pub fn info_origin() -> FText {
    FText(func("INFO", &[&"origin"]))
}

/// Returns information about the environment
#[inline]
pub fn info_recal() -> FText {
    FText(func("INFO", &[&"recalc"]))
}

/// Returns information about the environment
#[inline]
pub fn info_release() -> FText {
    FText(func("INFO", &[&"release"]))
}

/// Returns information about the environment
#[inline]
pub fn info_system() -> FText {
    FText(func("INFO", &[&"system"]))
}

/// Returns information about the environment
#[inline]
pub fn info_totmem() -> FNumber {
    FNumber(func("INFO", &[&"totmem"]))
}

/// Return TRUE if the referenced cell is blank, else return FALSE
#[inline]
pub fn isblank(x: impl Any) -> FLogical {
    FLogical(func("ISBLANK", &[&x]))
}

/// Return TRUE if the parameter has type Error and is not #N/A, else return FALSE.
#[inline]
pub fn iserr(x: impl Any) -> FLogical {
    FLogical(func("ISERR", &[&x]))
}

/// Return TRUE if the parameter has type Error, else return FALSE.
#[inline]
pub fn iserror(x: impl Any) -> FLogical {
    FLogical(func("ISERROR", &[&x]))
}

/// Return TRUE if the value is even, else return FALSE
#[inline]
pub fn iseven(number: impl Number) -> FLogical {
    FLogical(func("ISEVEN", &[&number]))
}

/// Return TRUE if the reference refers to a formula, else return FALSE.
#[inline]
pub fn isformula(refs: impl Reference) -> FLogical {
    FLogical(func("ISFORMULA", &[&refs]))
}

/// Return TRUE if the parameter has type Logical, else return FALSE
#[inline]
pub fn islogical(x: impl Any) -> FLogical {
    FLogical(func("ISLOGICAL", &[&x]))
}

/// Return TRUE if the parameter has type Error and is #N/A, else return FALSE.
#[inline]
pub fn isna(x: impl Any) -> FLogical {
    FLogical(func("ISNA", &[&x]))
}

///  Return TRUE if the parameter does not have type Text, else return FALSE.
#[inline]
pub fn isnontext(x: impl Any) -> FLogical {
    FLogical(func("ISNONTEXT", &[&x]))
}

///  Return TRUE if the parameter has type Number, else return FALSE
#[inline]
pub fn isnumber(x: impl Any) -> FLogical {
    FLogical(func("ISNUMBER", &[&x]))
}

/// Return TRUE if the value is even, else return FALSE.
#[inline]
pub fn isodd(x: impl Any) -> FLogical {
    FLogical(func("ISODD", &[&x]))
}

/// Return TRUE if the parameter is of type reference, else return FALSE.
#[inline]
pub fn isref(x: impl Any) -> FLogical {
    FLogical(func("ISREF", &[&x]))
}

/// Return TRUE if the parameter has type Text, else return FALSE.
#[inline]
pub fn istext(x: impl Any) -> FLogical {
    FLogical(func("ISTEXT", &[&x]))
}

/// Return the number of a value.
#[inline]
pub fn n(x: impl Any) -> FLogical {
    FLogical(func("N", &[&x]))
}

/// Return the constant Error value #N/A.
#[inline]
pub fn na() -> FLogical {
    FLogical(func("NA", &[]))
}

/// Convert text to number, in a locale-independent way.
#[inline]
pub fn numbervalue(
    text: impl Text,
    dec_sep: Option<impl Text>,
    group_sep: Option<impl Text>,
) -> FNumber {
    if let Some(dec_sep) = dec_sep {
        if let Some(group_sep) = group_sep {
            FNumber(func("NUMBERVALUE", &[&text, &dec_sep, &group_sep]))
        } else {
            FNumber(func("NUMBERVALUE", &[&text, &dec_sep]))
        }
    } else {
        if let Some(group_sep) = group_sep {
            FNumber(func("NUMBERVALUE", &[&text, &(), &group_sep]))
        } else {
            FNumber(func("NUMBERVALUE", &[&text]))
        }
    }
}

/// Returns the row number(s) of a reference.
#[inline]
pub fn row(refs: impl Reference) -> FNumber {
    FNumber(func("ROW", &[&refs]))
}

///  Returns the number of rows in a given range.
#[inline]
pub fn rows(refs: impl Reference) -> FNumber {
    FNumber(func("ROWS", &[&refs]))
}

/// Returns the sheet number of the reference or the string representing a sheet name.
#[inline]
pub fn sheet(refs: impl Reference) -> FNumber {
    FNumber(func("SHEET", &[&refs]))
}

/// Returns the number of sheets in a reference or current document.
#[inline]
pub fn sheets(refs: impl Reference) -> FNumber {
    FNumber(func("SHEETS", &[&refs]))
}

/// Returns a number indicating the type of the provided value.
#[inline]
pub fn value_type(a: impl Any) -> FNumber {
    FNumber(func("TYPE", &[&a]))
}

/// Convert text to number
#[inline]
pub fn value(text: impl Text) -> FNumber {
    FNumber(func("VALUE", &[&text]))
}
