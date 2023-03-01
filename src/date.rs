use crate::{func, Any, DateTimeParam, FNumber, Number, Sequence, Text};

pub fn date<D: Number, M: Number, Y: Number>(day: D, month: M, year: Y) -> FNumber {
    FNumber(func("DATE", &[&day, &month, &year]))
}

pub enum DateDifMethod {
    Years,
    Months,
    Days,
    DaysIgnoreMonthsYears,
    MonthsIgnoreYears,
    DaysIgnoreYears,
}
impl Any for DateDifMethod {
    fn formula(&self, buf: &mut String) {
        match self {
            DateDifMethod::Years => buf.push('y'),
            DateDifMethod::Months => buf.push('m'),
            DateDifMethod::Days => buf.push('d'),
            DateDifMethod::DaysIgnoreMonthsYears => buf.push_str("md"),
            DateDifMethod::MonthsIgnoreYears => buf.push_str("ym"),
            DateDifMethod::DaysIgnoreYears => buf.push_str("yd"),
        }
    }
}

pub fn datedif<D: DateTimeParam, D2: DateTimeParam>(
    start_date: D,
    end_date: D2,
    format: DateDifMethod,
) -> FNumber {
    FNumber(func("DATEDIF", &[&start_date, &end_date, &format]))
}

pub fn date_value<'a, T: Text>(txt: T) -> FNumber {
    FNumber(func("DATEVALUE", &[&txt]))
}

pub fn day<D: DateTimeParam>(date: D) -> FNumber {
    FNumber(func("DAY", &[&date]))
}

pub fn days<D: DateTimeParam, D2: DateTimeParam>(end_date: D, start_date: D2) -> FNumber {
    FNumber(func("DAYS", &[&end_date, &start_date]))
}

pub enum Days360Method {
    NASD,
    US,
    Europe,
}
impl Any for Days360Method {
    fn formula(&self, buf: &mut String) {
        match self {
            Days360Method::NASD => buf.push_str("FALSE()"),
            Days360Method::US => buf.push_str("FALSE()"),
            Days360Method::Europe => buf.push_str("TRUE()"),
        }
    }
}

pub fn days360<D: DateTimeParam, D2: DateTimeParam>(
    start_date: D,
    end_date: D2,
    method: Days360Method,
) -> FNumber {
    FNumber(func("DAYS360", &[&start_date, &end_date, &method]))
}

pub fn edate<D: DateTimeParam, N: Number>(start_date: D, month_add: N) -> FNumber {
    FNumber(func("EDATE", &[&start_date, &month_add]))
}

pub fn eomonth<D: DateTimeParam, N: Number>(start_date: D, month_add: N) -> FNumber {
    FNumber(func("EOMONTH", &[&start_date, &month_add]))
}

pub fn hour<T: DateTimeParam>(time: T) -> FNumber {
    FNumber(func("HOUR", &[&time]))
}

pub fn isoweeknum<D: DateTimeParam>(date: D) -> FNumber {
    FNumber(func("ISOWEEKNUM", &[&date]))
}

pub fn minute<T: DateTimeParam>(time: T) -> FNumber {
    FNumber(func("MINUTE", &[&time]))
}

pub fn month<D: DateTimeParam>(date: D) -> FNumber {
    FNumber(func("MONTH", &[&date]))
}

pub fn networkdays<D: DateTimeParam, D2: DateTimeParam, DS: Sequence, LS: Sequence>(
    start_date: D,
    end_date: D2,
    holidays: DS,
    workdays: LS,
) -> FNumber {
    FNumber(func(
        "NETWORKDAYS",
        &[&start_date, &end_date, &holidays, &workdays],
    ))
}

pub fn now() -> FNumber {
    FNumber(func("NOW", &[]))
}

pub fn second<T: DateTimeParam>(time: T) -> FNumber {
    FNumber(func("SECOND", &[&time]))
}

pub fn time<H: Number, M: Number, S: Number>(hours: H, minutes: M, seconds: S) -> FNumber {
    FNumber(func("TIME", &[&hours, &minutes, &seconds]))
}

pub fn timevalue<T: Text>(text: T) -> FNumber {
    FNumber(func("TIMEVALUE", &[&text]))
}

pub fn today() -> FNumber {
    FNumber(func("TODAY", &[]))
}

pub enum WeekdayMethod {
    Monday0,
    Monday1,
    Tuesday1,
    Wednesday1,
    Thursday1,
    Friday1,
    Saturday1,
    Sunday1,
}
impl Any for WeekdayMethod {
    fn formula(&self, buf: &mut String) {
        match self {
            WeekdayMethod::Monday0 => buf.push('3'),
            WeekdayMethod::Monday1 => buf.push_str("11"),
            WeekdayMethod::Tuesday1 => buf.push_str("12"),
            WeekdayMethod::Wednesday1 => buf.push_str("13"),
            WeekdayMethod::Thursday1 => buf.push_str("14"),
            WeekdayMethod::Friday1 => buf.push_str("15"),
            WeekdayMethod::Saturday1 => buf.push_str("16"),
            WeekdayMethod::Sunday1 => buf.push_str("17"),
        }
    }
}

pub fn weekday<D: DateTimeParam>(date: D, method: WeekdayMethod) -> FNumber {
    FNumber(func("WEEKDAY", &[&date, &method]))
}

pub enum WeeknumMethod {
    Jan1WeekMonday,
    Jan1WeekTuesday,
    Jan1WeekWednesday,
    Jan1WeekThursday,
    Jan1WeekFriday,
    Jan1WeekSaturday,
    Jan1WeekSunday,
    ISOWeeknum,
}
impl Any for WeeknumMethod {
    fn formula(&self, buf: &mut String) {
        match self {
            WeeknumMethod::Jan1WeekMonday => buf.push_str("11"),
            WeeknumMethod::Jan1WeekTuesday => buf.push_str("12"),
            WeeknumMethod::Jan1WeekWednesday => buf.push_str("13"),
            WeeknumMethod::Jan1WeekThursday => buf.push_str("14"),
            WeeknumMethod::Jan1WeekFriday => buf.push_str("15"),
            WeeknumMethod::Jan1WeekSaturday => buf.push_str("16"),
            WeeknumMethod::Jan1WeekSunday => buf.push_str("17"),
            WeeknumMethod::ISOWeeknum => buf.push_str("21"),
        }
    }
}

pub fn weeknum<D: DateTimeParam>(date: D, method: WeeknumMethod) -> FNumber {
    FNumber(func("WEEKNUM", &[&date, &method]))
}

pub fn workday<D: DateTimeParam, N: Number, DS: Sequence, LS: Sequence>(
    date: D,
    offset: N,
    holidays: DS,
    workdays: LS,
) -> FNumber {
    FNumber(func("WORKDAY", &[&date, &offset, &holidays, &workdays]))
}

pub fn year<D: DateTimeParam>(date: D) -> FNumber {
    FNumber(func("YEAR", &[&date]))
}

pub enum YearFracMethod {
    USNasd30_360,
    ActualActual,
    Actual360,
    Actual365,
    European30_360,
}
impl Any for YearFracMethod {
    fn formula(&self, buf: &mut String) {
        match self {
            YearFracMethod::USNasd30_360 => buf.push('0'),
            YearFracMethod::ActualActual => buf.push('1'),
            YearFracMethod::Actual360 => buf.push('2'),
            YearFracMethod::Actual365 => buf.push('3'),
            YearFracMethod::European30_360 => buf.push('4'),
        }
    }
}

pub fn yearfrac<D: DateTimeParam, D2: DateTimeParam>(
    start_date: D,
    end_date: D2,
    basis: Option<YearFracMethod>,
) -> FNumber {
    if let Some(basis) = basis {
        FNumber(func("YEARFRAC", &[&start_date, &end_date, &basis]))
    } else {
        FNumber(func("YEARFRAC", &[&start_date, &end_date]))
    }
}
