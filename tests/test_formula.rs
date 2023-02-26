use crate::result_test::{test, ReportDump, ReportValues};
use ods_formula as of;
use ods_formula::FormulaExt;
use spreadsheet_ods::CellRef;

mod result_test;

const Q: ReportValues = ReportValues;

#[test]
fn test1() {
    test(of::formula(35)).str("of=35").q(Q);
    test(of::formula("35")).str("of=\"35\"").q(Q);
    test(of::formula(35i64.val() + 17.val())).ok().q(Q);
    test(of::formula(of::val(&35) + of::val(&17))).ok().q(Q);
    test(of::formula(of::par(35.val() + 17.val()))).ok().q(Q);
    test(of::formula(of::par(35.val() * 17.val()))).ok().q(Q);
    test(of::formula(of::par(
        CellRef::local(17, 12).val() * 17.val(),
    )))
    .ok()
    .q(Q);
}
