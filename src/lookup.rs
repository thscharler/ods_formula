use crate::{
    func, func1, func2, func3, func4, func5, Any, Array, FAny, FReference, FText, Logical, Number,
    Param, Reference, Scalar, Sequence, Text,
};

pub enum AddressAbs {
    RowAbsColAbs,
    RowAbsColRel,
    RowRelColAbs,
    RowRelColRel,
}
impl Param for AddressAbs {
    type ParamType<'a> = u32;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            AddressAbs::RowAbsColAbs => 1,
            AddressAbs::RowAbsColRel => 2,
            AddressAbs::RowRelColAbs => 3,
            AddressAbs::RowRelColRel => 4,
        }
    }
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address(row: impl Number, col: impl Number, abs: AddressAbs) -> FText {
    FText(func4("ADDRESS", &row, &col, &abs.as_param(), &true))
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_in_sheet(
    row: impl Number,
    col: impl Number,
    abs: AddressAbs,
    sheet: impl Text,
) -> FText {
    FText(func5("ADDRESS", &row, &col, &abs.as_param(), &true, &sheet))
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_rc(row: impl Number, col: impl Number, abs: AddressAbs) -> FText {
    FText(func4("ADDRESS", &row, &col, &abs.as_param(), &false))
}

/// Returns a cell address (reference) as text.
#[inline]
pub fn address_rc_in_sheet(
    row: impl Number,
    col: impl Number,
    abs: AddressAbs,
    sheet: impl Text,
) -> FText {
    FText(func5(
        "ADDRESS",
        &row,
        &col,
        &abs.as_param(),
        &false,
        &sheet,
    ))
}

///  Uses an index to return a value from a list of values
#[inline]
pub fn choose(idx: impl Number, values: impl Sequence) -> FAny {
    FAny(func2("CHOOSE", &idx, &values))
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata<F: Text, S: Scalar>(datafield: impl Text, table: impl Reference) -> FAny {
    FAny(func2("GETPIVOTDATA", &datafield, &table))
}

/// Return a value from a data pilot table.
#[inline]
pub fn getpivotdata_fields<F: Text, S: Scalar>(
    datafield: impl Text,
    table: impl Reference,
    fields: &[(F, S)],
) -> FAny {
    let mut param: Vec<&dyn Any> = Vec::new();
    param.push(&datafield);
    param.push(&table);
    for (n, sc) in fields.iter() {
        param.push(n);
        param.push(sc);
    }
    FAny(func("GETPIVOTDATA", param.as_ref()))
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
    FAny(func4("HLOOKUP", &lookup, &data_source, &row, &range_lookup))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index(data_source: impl Array, row: impl Number, column: impl Number) -> FAny {
    FAny(func3("INDEX", &data_source, &row, &column))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index_row(data_source: impl Array, row: impl Number) -> FAny {
    FAny(func3("INDEX", &data_source, &row, &()))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index_col(data_source: impl Array, column: impl Number) -> FAny {
    FAny(func3("INDEX", &data_source, &(), &column))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index_area(
    data_source: impl Array,
    row: impl Number,
    column: impl Number,
    area_number: impl Number,
) -> FAny {
    FAny(func4("INDEX", &data_source, &row, &column, &area_number))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index_area_row(data_source: impl Array, row: impl Number, area_number: impl Number) -> FAny {
    FAny(func4("INDEX", &data_source, &row, &(), &area_number))
}

/// Returns a value using a row and column index value (and optionally an area index)
#[inline]
pub fn index_area_col(
    data_source: impl Array,
    column: impl Number,
    area_number: impl Number,
) -> FAny {
    FAny(func4("INDEX", &data_source, &(), &column, &area_number))
}

/// Return a reference given a string representation of a reference.
#[inline]
pub fn indirect(refs: impl Text) -> FReference {
    FReference(func1("INDIRECT", &refs))
}

/// Return a reference given a string representation of a reference.
#[inline]
pub fn indirect_rc(refs: impl Text) -> FReference {
    FReference(func2("INDIRECT", &refs, &false))
}

/// Look for criterion in an already-sorted array, and return a corresponding result.
#[inline]
pub fn lookup(find: impl Any, searched: impl Array) -> FAny {
    FAny(func2("LOOKUP", &find, &searched))
}

/// Look for criterion in an already-sorted array, and return a corresponding result.
#[inline]
pub fn lookup_map(find: impl Any, searched: impl Array, results: impl Array) -> FAny {
    FAny(func3("LOOKUP", &find, &searched, &results))
}

/// Finds a Search item in a sequence, and returns its position (starting from 1)
#[inline]
pub fn match_(search: impl Scalar, search_region: impl Array, match_type: impl Number) -> FAny {
    FAny(func3("MATCH", &search, &search_region, &match_type))
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
    FAny(func5(
        "MULTIPLE.OPERATIONS",
        &formula_cell,
        &row_cell,
        &row_replacement,
        &column_cell,
        &column_replacement,
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
    FReference(func5(
        "OFFSET",
        &refs,
        &row_offset,
        &column_offset,
        &new_height,
        &new_width,
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
    FAny(func4(
        "VLOOKUP",
        &lookup,
        &data_source,
        &column,
        &range_lookup,
    ))
}
