use crate::result_test::{test, ReportDump, ReportValues};
use ods_formula as of;
use ods_formula::prelude::*;
use spreadsheet_ods::{CellRange, CellRef};

mod result_test;

const Q: ReportValues = ReportValues;

#[test]
fn test_val() {
    let v = 35u32;
    let v0: &dyn Number = &v;

    let v = 35u32;
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;

    let v = 35u32;
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;
}

#[test]
fn test_str() {
    test(of::formula("boing")).q(Q);
    test(of::formula("zack".append("crack"))).q(Q);
    test(of::formula("zack".append(CellRef::local(5, 5)))).q(Q);
}

#[test]
fn test_add() {
    test(of::formula(35.add(17))).q(Q);
    test(of::formula(35.sub(17))).q(Q);
    test(of::formula(35.mul(17))).q(Q);
    test(of::formula(35.div(17))).q(Q);

    test(of::formula(35.21221.div(17))).q(Q);
}

#[test]
fn test1() {
    test(of::formula(35)).q(Q);
    test(of::formula("35")).q(Q);
    test(of::formula(35i64)).q(Q);

    test(of::formula(35.add(17))).q(Q);
    test(of::formula(of::par(35.add(17)))).q(Q);
    test(of::formula(of::par(35.mul(17)))).q(Q);
    test(of::formula(of::par(CellRef::local(17, 12).mul(17))))
        .ok()
        .q(Q);

    test(of::formula(35)).q(Q);

    test(of::formula(of::count([
        CellRange::origin_span(27, 1, (10, 10)),
        CellRange::local(4, 5, 25, 10),
    ])))
    .q(Q);
}
