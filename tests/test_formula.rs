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

    let v = 35u32.val();
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;

    let v = of::val(35u32);
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;

    let v = &35u32;
    let v0: &dyn Number = &v;

    let v = (&35u32).val();
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;

    let v = of::val(&35u32);
    let v0: &dyn Number = &v;
    let v0: &dyn Integer = &v;
    let v0: &dyn Any = &v;
}

#[test]
fn testadd() {
    let v = (&35u32).val();
    let w = (&35u32).val();

    let v0: &dyn Number = &v;
    let w0: &dyn Number = &w;

    let x = v0.add(w0);
    let y = v.add(w);
}

#[test]
fn test1() {
    test(of::formula(35)).q(Q);
    test(of::formula("35")).q(Q);
    test(of::formula(35i64.val())).q(Q);

    test(of::formula(35u32.add(17u32))).q(Q);
    // test(of::formula(of::par(35i32.val() + 17.val()))).q(Q);
    // test(of::formula(of::par(35.val() * 17.val()))).q(Q);
    // test(of::formula(of::par(
    //     CellRef::local(17, 12).val() * 17.val(),
    // )))
    // .ok()
    // .q(Q);
    //
    // test(of::formula(35.val())).str("of=35").q(Q);
    //
    // test(of::formula(of::count([CellRange::origin_span(
    //     27,
    //     1,
    //     (10, 10),
    // )])))
    // .q(Q);
    //
    // test(of::formula(of::count((CellRange::origin_span(
    //     27,
    //     1,
    //     (10, 10),
    // ),))))
    // .q(Q);
}
