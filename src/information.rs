use crate::{
    func, Any, Criterion, FCriterion, FLogical, FNumber, FReference, FText, Number, Reference,
    Sequence, Text,
};

pub fn areas<R: Reference>(refs: R) -> FNumber {
    FNumber(func("AREAS", &[&refs]))
}

pub fn cell_col<R: Reference>(refs: R) -> FNumber {
    FNumber(func("CELL", &[&"COL", &refs]))
}

pub fn cell_row<R: Reference>(refs: R) -> FNumber {
    FNumber(func("CELL", &[&"ROW", &refs]))
}

pub fn cell_sheet<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"SHEET", &refs]))
}

pub fn cell_address<R: Reference>(refs: R) -> FReference {
    FReference(func("CELL", &[&"ADDRESS", &refs]))
}

pub fn cell_filename<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"FILENAME", &refs]))
}

pub fn cell_contents<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"CONTENTS", &refs]))
}

pub fn cell_colored<R: Reference>(refs: R) -> FLogical {
    FLogical(func("CELL", &[&"COLOR", &refs]))
}

pub fn cell_format<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"FORMAT", &refs]))
}

pub fn cell_type<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"TYPE", &refs]))
}

pub fn cell_width<R: Reference>(refs: R) -> FNumber {
    FNumber(func("CELL", &[&"WIDTH", &refs]))
}

pub fn cell_protect<R: Reference>(refs: R) -> FLogical {
    FLogical(func("CELL", &[&"PROTECT", &refs]))
}

pub fn cell_parentheses<R: Reference>(refs: R) -> FLogical {
    FLogical(func("CELL", &[&"PARENTHESES", &refs]))
}

pub fn cell_prefix<R: Reference>(refs: R) -> FText {
    FText(func("CELL", &[&"PREFIX", &refs]))
}

pub fn column<R: Reference>(refs: R) -> FNumber {
    FNumber(func("COLUMN", &[&refs]))
}

pub fn columns<R: Reference>(refs: R) -> FNumber {
    FNumber(func("COLUMNS", &[&refs]))
}

pub fn count<S: Sequence>(seq: S) -> FNumber {
    FNumber(func("COUNT", &[&seq]))
}

pub fn counta<S: Sequence>(seq: S) -> FNumber {
    FNumber(func("COUNTA", &[&seq]))
}

pub fn countblank<S: Sequence>(seq: S) -> FNumber {
    FNumber(func("COUNTBLANK", &[&seq]))
}

pub fn countif<S: Sequence, C: Criterion>(seq: S, criterion: C) -> FNumber {
    FNumber(func("COUNTBLANK", &[&seq, &criterion]))
}

pub fn countifs(list: &[(FReference, FCriterion)]) -> FNumber {
    let mut buf = String::new();
    buf.push_str(name);
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

pub fn error_type<A: Any>(error: A) -> FNumber {
    FNumber(func("ERROR.TYPE", &[&error]))
}

pub fn formula<R: Reference>(refs: R) -> FText {
    FText(func("FORMULA", &[&refs]))
}

pub fn info_directory() -> FText {
    FText(func("INFO", &[&"directory"]))
}

pub fn info_memavail() -> FNumber {
    FNumber(func("INFO", &[&"memavail"]))
}

pub fn info_memused() -> FNumber {
    FNumber(func("INFO", &[&"memused"]))
}

pub fn info_numfile() -> FNumber {
    FNumber(func("INFO", &[&"numfile"]))
}

pub fn info_osversion() -> FText {
    FText(func("INFO", &[&"osversion"]))
}

pub fn info_origin() -> FText {
    FText(func("INFO", &[&"origin"]))
}

pub fn info_recal() -> FText {
    FText(func("INFO", &[&"recalc"]))
}

pub fn info_release() -> FText {
    FText(func("INFO", &[&"release"]))
}

pub fn info_system() -> FText {
    FText(func("INFO", &[&"system"]))
}

pub fn info_totmem() -> FNumber {
    FNumber(func("INFO", &[&"totmem"]))
}

pub fn isblank<A: Any>(x: A) -> FLogical {
    FLogical(func("ISBLANK", &[&x]))
}

pub fn iserr<A: Any>(x: A) -> FLogical {
    FLogical(func("ISERR", &[&x]))
}

pub fn iserror<A: Any>(x: A) -> FLogical {
    FLogical(func("ISERROR", &[&x]))
}

pub fn iseven<N: Number>(number: N) -> FLogical {
    FLogical(func("ISEVEN", &[&number]))
}

pub fn isformula<R: Reference>(refs: R) -> FLogical {
    FLogical(func("ISFORMULA", &[&refs]))
}

pub fn islogical<A: Any>(x: A) -> FLogical {
    FLogical(func("ISLOGICAL", &[&x]))
}

pub fn isna<A: Any>(x: A) -> FLogical {
    FLogical(func("ISNA", &[&x]))
}

pub fn isnontext<A: Any>(x: A) -> FLogical {
    FLogical(func("ISNONTEXT", &[&x]))
}

pub fn isnumber<A: Any>(x: A) -> FLogical {
    FLogical(func("ISNUMBER", &[&x]))
}

pub fn isodd<A: Any>(x: A) -> FLogical {
    FLogical(func("ISODD", &[&x]))
}

pub fn isref<A: Any>(x: A) -> FLogical {
    FLogical(func("ISREF", &[&x]))
}

pub fn istext<A: Any>(x: A) -> FLogical {
    FLogical(func("ISTEXT", &[&x]))
}

pub fn n<A: Any>(x: A) -> FLogical {
    FLogical(func("N", &[&x]))
}

pub fn na() -> FLogical {
    FLogical(func("NA", &[]))
}

pub fn numbervalue<T: Text, T2: Text, T3: Text>(
    text: T,
    dec_sep: Option<T>,
    group_sep: Option<T>,
) -> FNumber {
    if let Some(dec_sep) = dec_sep {
        if let Some(group_sep) = group_sep {
            FNumber(func("NUMBERVALUE", &[text, dec_sep, group_sep]))
        } else {
            FNumber(func("NUMBERVALUE", &[text, dec_sep]))
        }
    } else {
        if let Some(group_sep) = group_sep {
            FNumber(func("NUMBERVALUE", &[text, &(), group_sep]))
        } else {
            FNumber(func("NUMBERVALUE", &[text]))
        }
    }
}

pub fn row<R: Reference>(refs: R) -> FNumber {
    FNumber(func("ROW", &[&refs]))
}

pub fn rows<R: Reference>(refs: R) -> FNumber {
    FNumber(func("ROWS", &[&refs]))
}

pub fn sheet<R: Reference>(refs: R) -> FNumber {
    FNumber(func("SHEET", &[&refs]))
}

pub fn sheets<R: Reference>(refs: R) -> FNumber {
    FNumber(func("SHEETS", &[&refs]))
}

pub fn value_type<A: Any>(a: A) -> FNumber {
    FNumber(func("TYPE", &[&a]))
}

pub fn value<T: Text>(text: T) -> FNumber {
    FNumber(func("VALUE", &[&text]))
}
