use crate::{func2, func3, Criteria, Database, FNumber, Field};

///  Finds the average of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn daverage(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DAVERAGE", &database, &field, &criteria))
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain numerical values.
#[inline]
pub fn dcount(database: impl Database, criteria: impl Criteria) -> FNumber {
    FNumber(func2("DCOUNT", &database, &criteria))
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain numerical values.
#[inline]
pub fn dcount_field(
    database: impl Database,
    field: impl Field,
    criteria: impl Criteria,
) -> FNumber {
    FNumber(func3("DCOUNT", &database, &field, &criteria))
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain values (as COUNTA).
#[inline]
pub fn dcounta(database: impl Database, criteria: impl Criteria) -> FNumber {
    FNumber(func2("DCOUNTA", &database, &criteria))
}

/// Counts the number of records (rows) in a database that match a search criteria and
/// contain values (as COUNTA).
#[inline]
pub fn dcounta_field(
    database: impl Database,
    field: impl Field,
    criteria: impl Criteria,
) -> FNumber {
    FNumber(func3("DCOUNTA", &database, &field, &criteria))
}

/// Gets the single value in the field from the single record (row) in a database that
/// matches a search criteria.
#[inline]
pub fn dget(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DGET", &database, &field, &criteria))
}

///  Finds the maximum value in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dmax(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DMAX", &database, &field, &criteria))
}

/// Finds the minimum value in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dmin(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DMIN", &database, &field, &criteria))
}

/// Finds the product of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dproduct(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DPRODUCT", &database, &field, &criteria))
}

/// Finds the sample standard deviation in a given field from the records (rows) in a
/// database that match a search criteria.
#[inline]
pub fn dstdev(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DSTDEV", &database, &field, &criteria))
}

/// Finds the population standard deviation in a given field from the records (rows) in a
/// database that match a search criteria.
#[inline]
pub fn dstdevp(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DSTDEVP", &database, &field, &criteria))
}

/// Finds the sum of values in a given field from the records (rows) in a database that
/// match a search criteria.
#[inline]
pub fn dsum(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DSUM", &database, &field, &criteria))
}

/// Finds the sample variance in a given field from the records (rows) in a database that
/// match a search criteria
#[inline]
pub fn dvar(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DVAR", &database, &field, &criteria))
}

/// Finds the population variance in a given field from the records (rows) in a database
/// that match a search criteria.
#[inline]
pub fn dvarp(database: impl Database, field: impl Field, criteria: impl Criteria) -> FNumber {
    FNumber(func3("DVARP", &database, &field, &criteria))
}
