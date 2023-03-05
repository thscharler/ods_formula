use crate::{func, Criteria, Database, FNumber, Field};

///  Finds the average of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn daverage(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DAVERAGE", &[&database, &field, &criteria]))
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain numerical values.
#[inline]
pub fn dcount(
    database: impl Database,
    field: Option<impl Field>,
    criteria: impl Criteria,
) -> FNumber {
    if let Some(field) = field {
        FNumber(func("DCOUNT", &[&database, &field, &criteria]))
    } else {
        FNumber(func("DCOUNT", &[&database, &criteria]))
    }
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain values (as COUNTA).
#[inline]
pub fn dcounta(
    database: impl Database,
    field: Option<impl Field>,
    criteria: impl Criteria,
) -> FNumber {
    if let Some(field) = field {
        FNumber(func("DCOUNTA", &[&database, &field, &criteria]))
    } else {
        FNumber(func("DCOUNTA", &[&database, &criteria]))
    }
}

/// Gets the single value in the field from the single record (row) in a database that
/// matches a search criteria.
#[inline]
pub fn dget(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DGET", &[&database, &field, &criteria]))
}

///  Finds the maximum value in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dmax(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DMAX", &[&database, &field, &criteria]))
}

/// Finds the minimum value in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dmin(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DMIN", &[&database, &field, &criteria]))
}

/// Finds the product of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dproduct(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DPRODUCT", &[&database, &field, &criteria]))
}

/// Finds the sample standard deviation in a given field from the records (rows) in a
/// database that match a search criteria.
#[inline]
pub fn dstdev(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DSTDEV", &[&database, &field, &criteria]))
}

/// Finds the population standard deviation in a given field from the records (rows) in a
/// database that match a search criteria.
#[inline]
pub fn dstdevp(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DSTDEVP", &[&database, &field, &criteria]))
}

/// Finds the sum of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dsum(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DSUM", &[&database, &field, &criteria]))
}

/// Finds the sample variance in a given field from the records (rows) in a database that
/// match a search criteria
#[inline]
pub fn dvar(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DVAR", &[&database, &field, &criteria]))
}

/// Finds the population variance in a given field from the records (rows) in a database
/// that match a search criteria.
#[inline]
pub fn dvarp(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func("DVARP", &[&database, &field, &criteria]))
}
