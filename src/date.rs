use crate::{
    func0, func1, func2, func3, func4, DateTimeParam, FLogical, FNumber, Logical, Number, Param,
    Sequence, Text,
};

/// Constructs a date from year, month, and day of month.
#[inline]
pub fn date(day: impl Number, month: impl Number, year: impl Number) -> FNumber {
    FNumber(func3("DATE", &day, &month, &year))
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
impl Param for DateDifMethod {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            DateDifMethod::Years => "y",
            DateDifMethod::Months => "m",
            DateDifMethod::Days => "d",
            DateDifMethod::DaysIgnoreMonthsYears => "md",
            DateDifMethod::MonthsIgnoreYears => "ym",
            DateDifMethod::DaysIgnoreYears => "yd",
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
    FNumber(func3("DATEDIF", &start_date, &end_date, &format.as_param()))
}

/// Returns the date serial number from given text.
#[inline]
pub fn date_value(txt: impl Text) -> FNumber {
    FNumber(func1("DATEVALUE", &txt))
}

/// Returns the day from a date.
#[inline]
pub fn day(date: impl DateTimeParam) -> FNumber {
    FNumber(func1("DAY", &date))
}

/// Returns the number of days between two dates
#[inline]
pub fn days(end_date: impl DateTimeParam, start_date: impl DateTimeParam) -> FNumber {
    FNumber(func2("DAYS", &end_date, &start_date))
}

/// Method for DAYS360()
pub enum Days360Method {
    /// NASD Method
    USNasd,
    /// European Method
    Europe,
}
impl Param for Days360Method {
    type ParamType<'a> = FLogical;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            Days360Method::USNasd => FLogical("FALSE()".into()),
            Days360Method::Europe => FLogical("TRUE()".into()),
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
    FNumber(func3("DAYS360", &start_date, &end_date, &method.as_param()))
}

/// Returns the serial number of a given date when MonthAdd months is added
#[inline]
pub fn edate(start_date: impl DateTimeParam, month_add: impl Number) -> FNumber {
    FNumber(func2("EDATE", &start_date, &month_add))
}

/// Returns the serial number of the end of a month, given date plus MonthAdd months
#[inline]
pub fn eomonth(start_date: impl DateTimeParam, month_add: impl Number) -> FNumber {
    FNumber(func2("EOMONTH", &start_date, &month_add))
}

/// Extracts the hour (0 through 23) from a time.
#[inline]
pub fn hour(time: impl DateTimeParam) -> FNumber {
    FNumber(func1("HOUR", &time))
}

/// Determines the ISO week number of the year for a given date
#[inline]
pub fn isoweeknum(date: impl DateTimeParam) -> FNumber {
    FNumber(func1("ISOWEEKNUM", &date))
}

/// Extracts the minute (0 through 59) from a time
#[inline]
pub fn minute(time: impl DateTimeParam) -> FNumber {
    FNumber(func1("MINUTE", &time))
}

/// Extracts the month from a date.
#[inline]
pub fn month(date: impl DateTimeParam) -> FNumber {
    FNumber(func1("MONTH", &date))
}

/// Returns the whole number of work days between two dates.
#[inline]
pub fn networkdays(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    holidays: impl Sequence,
    workdays: [[impl Logical; 7]; 1],
) -> FNumber {
    FNumber(func4(
        "NETWORKDAYS",
        &start_date,
        &end_date,
        &holidays,
        &workdays,
    ))
}

/// Returns the serial number of the current date and time.
#[inline]
pub fn now() -> FNumber {
    FNumber(func0("NOW"))
}

/// Extracts the second (the integer 0 through 59) from a time. This function presumes
/// that leap seconds never exist.
#[inline]
pub fn second(time: impl DateTimeParam) -> FNumber {
    FNumber(func1("SECOND", &time))
}

/// Constructs a time value from hours, minutes, and seconds.
#[inline]
pub fn time(hours: impl Number, minutes: impl Number, seconds: impl Number) -> FNumber {
    FNumber(func3("TIME", &hours, &minutes, &seconds))
}

/// Returns a time serial number from given text
#[inline]
pub fn timevalue(text: impl Text) -> FNumber {
    FNumber(func1("TIMEVALUE", &text))
}

/// Returns the serial number of today.
#[inline]
pub fn today() -> FNumber {
    FNumber(func0("TODAY"))
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
impl Param for WeekdayMethod {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            WeekdayMethod::Monday0 => "3",
            WeekdayMethod::Monday1 => "11",
            WeekdayMethod::Tuesday1 => "12",
            WeekdayMethod::Wednesday1 => "13",
            WeekdayMethod::Thursday1 => "14",
            WeekdayMethod::Friday1 => "15",
            WeekdayMethod::Saturday1 => "16",
            WeekdayMethod::Sunday1 => "17",
        }
    }
}

/// Extracts the day of the week from a date; if text, uses current locale to convert to a
/// date.
#[inline]
pub fn weekday(date: impl DateTimeParam, method: WeekdayMethod) -> FNumber {
    FNumber(func2("WEEKDAY", &date, &method.as_param()))
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
impl Param for WeeknumMethod {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            WeeknumMethod::Jan1WeekMonday => "11",
            WeeknumMethod::Jan1WeekTuesday => "12",
            WeeknumMethod::Jan1WeekWednesday => "13",
            WeeknumMethod::Jan1WeekThursday => "14",
            WeeknumMethod::Jan1WeekFriday => "15",
            WeeknumMethod::Jan1WeekSaturday => "16",
            WeeknumMethod::Jan1WeekSunday => "17",
            WeeknumMethod::ISOWeeknum => "21",
        }
    }
}

/// Determines the week number of the year for a given date.
#[inline]
pub fn weeknum(date: impl DateTimeParam, method: WeeknumMethod) -> FNumber {
    FNumber(func2("WEEKNUM", &date, &method.as_param()))
}

/// Returns the date serial number which is a specified number of work days before or
/// after an input date.
#[inline]
pub fn workday(
    date: impl DateTimeParam,
    offset: impl Number,
    holidays: impl Sequence,
    workdays: [[impl Logical; 7]; 1],
) -> FNumber {
    FNumber(func4("WORKDAY", &date, &offset, &holidays, &workdays))
}

/// Extracts the year from a date given in the current locale of the evaluator.
#[inline]
pub fn year(date: impl DateTimeParam) -> FNumber {
    FNumber(func1("YEAR", &date))
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
impl Param for YearFracMethod {
    type ParamType<'a> = &'a str;

    fn as_param(&self) -> Self::ParamType<'_> {
        match self {
            YearFracMethod::USNasd30_360 => "0",
            YearFracMethod::ActualActual => "1",
            YearFracMethod::Actual360 => "2",
            YearFracMethod::Actual365 => "3",
            YearFracMethod::European30_360 => "4",
        }
    }
}

/// Extracts the number of years (including fractional part) between two dates
#[inline]
pub fn yearfrac(
    start_date: impl DateTimeParam,
    end_date: impl DateTimeParam,
    basis: YearFracMethod,
) -> FNumber {
    FNumber(func3("YEARFRAC", &start_date, &end_date, &basis.as_param()))
}
