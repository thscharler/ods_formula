use crate::{func, Criteria, Database, FNumber, Field};

pub fn daverage<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DAVERAGE", &[&database, &field, &criteria]))
}

pub fn dcount<D: Database, F: Field, C: Criteria>(
    database: D,
    field: Option<F>,
    criteria: C,
) -> FNumber {
    if let Some(field) = field {
        FNumber(func("DCOUNT", &[&database, &field, &criteria]))
    } else {
        FNumber(func("DCOUNT", &[&database, &criteria]))
    }
}

pub fn dcounta<D: Database, F: Field, C: Criteria>(
    database: D,
    field: Option<F>,
    criteria: C,
) -> FNumber {
    if let Some(field) = field {
        FNumber(func("DCOUNTA", &[&database, &field, &criteria]))
    } else {
        FNumber(func("DCOUNTA", &[&database, &criteria]))
    }
}

pub fn dget<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DGET", &[&database, &field, &criteria]))
}

pub fn dmax<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DMAX", &[&database, &field, &criteria]))
}

pub fn dmin<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DMIN", &[&database, &field, &criteria]))
}

pub fn dproduct<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DPRODUCT", &[&database, &field, &criteria]))
}

pub fn dstdev<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DSTDEV", &[&database, &field, &criteria]))
}

pub fn dstdevp<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DSTDEVP", &[&database, &field, &criteria]))
}

pub fn dsum<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DSUM", &[&database, &field, &criteria]))
}

pub fn dvar<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DVAR", &[&database, &field, &criteria]))
}

pub fn dvarp<D: Database, F: Field, C: Criteria>(database: D, field: F, criteria: C) -> FNumber {
    FNumber(func("DVARP", &[&database, &field, &criteria]))
}
