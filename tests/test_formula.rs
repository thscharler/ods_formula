use crate::result_test::{test, test_ok, ReportDump, ReportValues};
use ods_formula as of;
use ods_formula::prelude::*;
use spreadsheet_ods::{CellRange, CellRef};

mod result_test;

const Q: ReportValues = ReportValues;

#[test]
fn test_val() {
    let ff = p(35.n() + 17);
    let x = 9;
    // let xf = ff + x;

    let ff = p(35.n() + 17);
    let x = 9;
    // let xf = x + ff;
}

#[test]
fn test_str() {
    test_ok(of::formula("boing")).q(Q);
    test_ok(of::formula("zack".concat("crack"))).q(Q);
    test_ok(of::formula("zack".concat(CellRef::local(5, 5)))).q(Q);
}

#[test]
fn nest_add() {
    test_ok(of::formula(35.add(17))).q(Q);
    test_ok(of::formula(35.sub(17))).q(Q);
    test_ok(of::formula(35.mul(17))).q(Q);
    test_ok(of::formula(35.div(17))).q(Q);

    test_ok(of::formula(35.21221.div(17))).q(Q);
}

#[test]
fn nest1() {
    test_ok(of::formula(35)).q(Q);
    test_ok(of::formula("35")).q(Q);
    test_ok(of::formula(35i64)).q(Q);

    test_ok(of::formula(35.add(17))).q(Q);
    test_ok(of::formula(p(35.n() + 17).add(9))).q(Q);
    test_ok(of::formula(p(35.n() + 17).n() + 9)).q(Q);
    test_ok(of::formula(p(35.mul(17)))).q(Q);
    test_ok(of::formula(p(CellRef::local(17, 12).mul(17))))
        .ok()
        .q(Q);

    test_ok(of::formula(35)).q(Q);

    // test_ok(of::getpivotdata("aas", CellRef::local(4, 5), None)).q(Q);

    // nest(of::formula(of::count(
    //     CellRange::origin_span(27, 1, (10, 10)).refcat(CellRange::local(4, 5, 25, 10)),
    // )))
    // .q(Q);
    //
    // nest(of::formula(of::sum((1, 2, 3)))).q(Q);
    // nest(of::formula(of::sum((1, 2, 3)))).q(Q);
}
