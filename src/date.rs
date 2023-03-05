use crate::{func, Any, DateTimeParam, FNumber, Number, Sequence, Text};

/// Constructs a date from year, month, and day of month.
#[inline]
pub fn date(day: impl Number, month: impl Number, year: impl Number) -> FNumber {
    FNumber(func("DATE", &[&day, &month, &year]))
}

/// Method for DATEDIF()
pub enum DateDifMethod {
    /// Years
    Years,
    /// Months
    Months,
    /// Days
    Days,
    /// Days, ignoring months and years.
    DaysIgnoreMonthsYears,
    /// Months, ignoring years.
    MonthsIgnoreYears,
    /// Days, ignoring years.
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

/// Returns the difference in years, months, or days of two date numbers.
#[inline]
pub fn datedif(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    format: DateDifMethod,
) -> FNumber {
    FNumber(func("DATEDIF", &[&start_date, &end_date, &format]))
}

/// Returns the date serial number from given text.
#[inline]
pub fn date_value(txt: impl Text) -> FNumber {
    FNumber(func("DATEVALUE", &[&txt]))
}

/// Returns the day from a date.
#[inline]
pub fn day(date: impl DateTimeParam) -> FNumber {
    FNumber(func("DAY", &[&date]))
}

/// Returns the number of days between two dates
#[inline]
pub fn days(end_date: impl DateTimeParam, start_date: impl DateTimeParam) -> FNumber {
    FNumber(func("DAYS", &[&end_date, &start_date]))
}

/// Method for DAYS360()
pub enum Days360Method {
    /// NASD Method
    USNasd,
    /// European Method
    Europe,
}
impl Any for Days360Method {
    fn formula(&self, buf: &mut String) {
        match self {
            Days360Method::USNasd => buf.push_str("FALSE()"),
            Days360Method::Europe => buf.push_str("TRUE()"),
        }
    }
}

/// Returns the number of days between two dates using the 360-day year
#[inline]
pub fn days360(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    method: Days360Method,
) -> FNumber {
    FNumber(func("DAYS360", &[&start_date, &end_date, &method]))
}

/// Returns the serial number of a given date when MonthAdd months is added
#[inline]
pub fn edate(start_date: impl DateTimeParam, month_add: impl Number) -> FNumber {
    FNumber(func("EDATE", &[&start_date, &month_add]))
}

/// Returns the serial number of the end of a month, given date plus MonthAdd months
#[inline]
pub fn eomonth(start_date: impl DateTimeParam, month_add: impl Number) -> FNumber {
    FNumber(func("EOMONTH", &[&start_date, &month_add]))
}

/// Extracts the hour (0 through 23) from a time.
#[inline]
pub fn hour(time: impl DateTimeParam) -> FNumber {
    FNumber(func("HOUR", &[&time]))
}

/// Determines the ISO week number of the year for a given date
#[inline]
pub fn isoweeknum(date: impl DateTimeParam) -> FNumber {
    FNumber(func("ISOWEEKNUM", &[&date]))
}

/// Extracts the minute (0 through 59) from a time
#[inline]
pub fn minute(time: impl DateTimeParam) -> FNumber {
    FNumber(func("MINUTE", &[&time]))
}

/// Extracts the month from a date.
#[inline]
pub fn month(date: impl DateTimeParam) -> FNumber {
    FNumber(func("MONTH", &[&date]))
}

/// Returns the whole number of work days between two dates.
#[inline]
pub fn networkdays(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    holidays: impl Sequence,
    workdays: impl Sequence,
) -> FNumber {
    FNumber(func(
        "NETWORKDAYS",
        &[&start_date, &end_date, &holidays, &workdays],
    ))
}

/// Returns the serial number of the current date and time.
#[inline]
pub fn now() -> FNumber {
    FNumber(func("NOW", &[]))
}

/// Extracts the second (the integer 0 through 59) from a time. This function presumes
/// that leap seconds never exist.
#[inline]
pub fn second(time: impl DateTimeParam) -> FNumber {
    FNumber(func("SECOND", &[&time]))
}

/// Constructs a time value from hours, minutes, and seconds.
#[inline]
pub fn time(hours: impl Number, minutes: impl Number, seconds: impl Number) -> FNumber {
    FNumber(func("TIME", &[&hours, &minutes, &seconds]))
}

/// Returns a time serial number from given text
#[inline]
pub fn timevalue(text: impl Text) -> FNumber {
    FNumber(func("TIMEVALUE", &[&text]))
}

/// Returns the serial number of today.
#[inline]
pub fn today() -> FNumber {
    FNumber(func("TODAY", &[]))
}

/// Method for WEEKDAY()
pub enum WeekdayMethod {
    /// Monday first, value 0.
    Monday0,
    /// Monday first, value 1.
    Monday1,
    /// Tuesday first, value 1.
    Tuesday1,
    /// Wednesday first, value 1.
    Wednesday1,
    /// Thursday first, value 1.
    Thursday1,
    /// Friday first, value 1.
    Friday1,
    /// Saturday first, value 1.
    Saturday1,
    /// Sunday first, value 1.
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

/// Extracts the day of the week from a date; if text, uses current locale to convert to a
/// date.
#[inline]
pub fn weekday(date: impl DateTimeParam, method: WeekdayMethod) -> FNumber {
    FNumber(func("WEEKDAY", &[&date, &method]))
}

/// Method for WEEKNUM()
pub enum WeeknumMethod {
    /// First week contains Jan 1, week starts on Monday.
    Jan1WeekMonday,
    /// First week contains Jan 1, week starts on Tuesday.
    Jan1WeekTuesday,
    /// First week contains Jan 1, week starts on Wednesday.
    Jan1WeekWednesday,
    /// First week contains Jan 1, week starts on Thursday.
    Jan1WeekThursday,
    /// First week contains Jan 1, week starts on Friday.
    Jan1WeekFriday,
    /// First week contains Jan 1, week starts on Saturday.
    Jan1WeekSaturday,
    /// First week contains Jan 1, week starts on Sunday.
    Jan1WeekSunday,
    /// ISO week number.
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

/// Determines the week number of the year for a given date.
#[inline]
pub fn weeknum(date: impl DateTimeParam, method: WeeknumMethod) -> FNumber {
    FNumber(func("WEEKNUM", &[&date, &method]))
}

/// Returns the date serial number which is a specified number of work days before or
/// after an input date.
#[inline]
pub fn workday(
    date: impl DateTimeParam,
    offset: impl Number,
    holidays: impl Sequence,
    workdays: impl Sequence,
) -> FNumber {
    FNumber(func("WORKDAY", &[&date, &offset, &holidays, &workdays]))
}

/// Extracts the year from a date given in the current locale of the evaluator.
#[inline]
pub fn year(date: impl DateTimeParam) -> FNumber {
    FNumber(func("YEAR", &[&date]))
}

/// Method for YEARFRAC()
pub enum YearFracMethod {
    ///
    USNasd30_360,
    ///
    ActualActual,
    ///
    Actual360,
    ///
    Actual365,
    ///
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

/// Extracts the number of years (including fractional part) between two dates
#[inline]
pub fn yearfrac(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    basis: Option<YearFracMethod>,
) -> FNumber {
    if let Some(basis) = basis {
        FNumber(func("YEARFRAC", &[&start_date, &end_date, &basis]))
    } else {
        FNumber(func("YEARFRAC", &[&start_date, &end_date]))
    }
}
