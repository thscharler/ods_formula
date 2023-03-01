use crate::result_test::{nest, test, ReportDump, ReportValues};
use ods_formula as of;
use ods_formula::prelude::*;
use spreadsheet_ods::{CellRange, CellRef};

mod result_test;

const Q: ReportValues = ReportValues;

#[test]
fn test_val() {}

#[test]
fn test_str() {
    nest(of::formula("boing")).q(Q);
    nest(of::formula("zack".concat("crack"))).q(Q);
    nest(of::formula("zack".concat(CellRef::local(5, 5)))).q(Q);
}

#[test]
fn nest_add() {
    nest(of::formula(35.add(17))).q(Q);
    nest(of::formula(35.sub(17))).q(Q);
    nest(of::formula(35.mul(17))).q(Q);
    nest(of::formula(35.div(17))).q(Q);

    nest(of::formula(35.21221.div(17))).q(Q);
}

#[test]
fn nest1() {
    nest(of::formula(35)).q(Q);
    nest(of::formula("35")).q(Q);
    nest(of::formula(35i64)).q(Q);

    nest(of::formula(35.add(17))).q(Q);
    nest(of::formula(of::par(35.n() + 17) + 9)).q(Q);
    nest(of::formula(of::par(35.mul(17)))).q(Q);
    nest(of::formula(of::par(CellRef::local(17, 12).mul(17))))
        .ok()
        .q(Q);

    nest(of::formula(35)).q(Q);

    nest(of::formula(of::count([
        CellRange::origin_span(27, 1, (10, 10)),
        CellRange::local(4, 5, 25, 10),
    ])))
    .q(Q);
}
