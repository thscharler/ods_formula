use crate::{
    func, Any, Array, FAny, FReference, FText, Logical, Number, Reference, Scalar, Sequence, Text,
};

/// Returns a cell address (reference) as text.
#[inline]
pub fn address(
    row: impl Number,
    col: impl Number,
    abs: Option<impl Number>,
    sheet: Option<impl Text>,
) -> FText {
    let mut buf = String::new();
    buf.push_str("ADDRESS(");
    row.formula(&mut buf);
    buf.push(';');
    col.formula(&mut buf);
    buf.push(';');
    abs.formula(&mut buf);
    buf.push(';');
    true.formula(&mut buf);
    buf.push(';');
    sheet.formula(&mut buf);
    buf.push(')');
    FText(buf)
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_rc(
    row: impl Number,
    col: impl Number,
    abs: Option<impl Number>,
    sheet: Option<impl Text>,
) -> FText {
    let mut buf = String::new();
    buf.push_str("ADDRESS(");
    row.formula(&mut buf);
    buf.push(';');
    col.formula(&mut buf);
    buf.push(';');
    abs.formula(&mut buf);
    buf.push(';');
    false.formula(&mut buf);
    buf.push(';');
    sheet.formula(&mut buf);
    buf.push(')');
    FText(buf)
}

///  Uses an index to return a value from a list of values
#[inline]
pub fn choose(idx: impl Number, values: impl Sequence) -> FAny {
    FAny(func("CHOOSE", &[&idx, &values]))
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata(
    datafield: impl Text,
    table: impl Reference,
    fields: Option<&[(impl Text, impl Any)]>,
) -> FAny {
    let mut buf = String::new();
    buf.push_str("GETPIVOTDATA(");
    datafield.formula(&mut buf);
    buf.push(';');
    table.formula(&mut buf);
    buf.push(';');

    if let Some(fields) = fields {
        for (i, (n, sc)) in fields.iter().enumerate() {
            if i > 0 {
                buf.push(';');
            }
            n.formula(&mut buf);
            buf.push(';');
            sc.formula(&mut buf);
        }
    }

    FAny(buf)
}

/// Look for a matching value in the first row of the given table, and return the value of the
/// indicated row.
#[inline]
pub fn hlookup(
    lookup: impl Any,
    data_source: impl Array,
    row: impl Number,
    range_lookup: impl Logical,
) -> FAny {
    FAny(func(
        "HLOOKUP",
        &[&lookup, &data_source, &row, &range_lookup],
    ))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index(
    data_source: impl Array,
    row: Option<impl Number>,
    column: Option<impl Number>,
    area_number: Option<impl Number>,
) -> FAny {
    FAny(func("INDEX", &[&data_source, &row, &column, &area_number]))
}

/// Return a reference given a string representation of a reference.
#[inline]
pub fn indirect(refs: impl Text) -> FReference {
    FReference(func("INDIRECT", &[&refs]))
}

/// Return a reference given a string representation of a reference.
#[inline]
pub fn indirect_rc(refs: impl Text) -> FReference {
    FReference(func("INDIRECT", &[&refs, &false]))
}

/// Look for criterion in an already-sorted array, and return a corresponding result.
#[inline]
pub fn lookup(find: impl Any, searched: impl Array, results: Option<impl Array>) -> FAny {
    FAny(func("LOOKUP", &[&find, &searched, &results]))
}

/// Finds a Search item in a sequence, and returns its position (starting from 1)
#[inline]
pub fn match_(search: impl Scalar, search_region: impl Array, match_type: impl Number) -> FAny {
    FAny(func("MATCH", &[&search, &search_region, &match_type]))
}

/// Executes a formula expression while substituting a row reference and a column
/// reference.
#[inline]
pub fn multiple_operations(
    formula_cell: impl Reference,
    row_cell: impl Reference,
    row_replacement: impl Reference,
    column_cell: impl Reference,
    column_replacement: impl Reference,
) -> FAny {
    FAny(func(
        "MULTIPLE.OPERATIONS",
        &[
            &formula_cell,
            &row_cell,
            &row_replacement,
            &column_cell,
            &column_replacement,
        ],
    ))
}

/// Modifies a reference's position and dimension.
#[inline]
pub fn offset(
    refs: impl Reference,
    row_offset: impl Number,
    column_offset: impl Number,
    new_height: impl Number,
    new_width: impl Number,
) -> FReference {
    FReference(func(
        "OFFSET",
        &[&refs, &row_offset, &column_offset, &new_height, &new_width],
    ))
}

/// Look for a matching value in the first column of the given table, and return the value of
/// the indicated column.
#[inline]
pub fn vlookup(
    lookup: impl Any,
    data_source: impl Array,
    column: impl Number,
    range_lookup: impl Logical,
) -> FAny {
    FAny(func(
        "VLOOKUP",
        &[&lookup, &data_source, &column, &range_lookup],
    ))
}
