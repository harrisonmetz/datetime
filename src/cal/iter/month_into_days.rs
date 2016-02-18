use std::ops::{Range, RangeFrom, RangeTo, RangeFull};

use cal::compound::YearMonth;
use cal::local;


/// Trait for types that contain multiple dates.
pub trait DaysIter {

    /// Returns an iterator over a continuous span of days in this month,
    /// returning `local::Date` values.
    ///
    /// ### Examples
    ///
    /// ```
    /// use datetime::cal::iter::DaysIter;
    /// use datetime::cal::unit::Month::September;
    /// use datetime::cal::unit::Year;
    ///
    /// let ym = Year::from(1999).month(September);
    /// assert_eq!(ym.days(..).count(), 30);
    /// assert_eq!(ym.days(10 ..).count(), 21);
    /// assert_eq!(ym.days(10 .. 20).count(), 10);
    /// assert_eq!(ym.days(.. 20).count(), 19);
    /// ```
    fn days<S: DaySpan>(&self, span: S) -> MonthDays;
}


impl DaysIter for YearMonth {
    fn days<S: DaySpan>(&self, span: S) -> MonthDays {
        MonthDays {
            ym: *self,
            range: span.get_range(self)
        }
    }
}

/// A span of days, which gets used to construct a `MonthDays` iterator.
pub trait DaySpan {

    /// Returns a `Range` of the day numbers specified for the given year-month pair.
    fn get_range(&self, ym: &YearMonth) -> Range<i8>;
}

impl DaySpan for RangeFull {
    fn get_range(&self, ym: &YearMonth) -> Range<i8> {
        1 .. ym.day_count() + 1
    }
}

impl DaySpan for RangeFrom<i8> {
    fn get_range(&self, ym: &YearMonth) -> Range<i8> {
        self.start .. ym.day_count() + 1
    }
}

impl DaySpan for RangeTo<i8> {
    fn get_range(&self, _ym: &YearMonth) -> Range<i8> {
        1 .. self.end
    }
}

impl DaySpan for Range<i8> {
    fn get_range(&self, _ym: &YearMonth) -> Range<i8> {
        self.clone()
    }
}


/// An iterator over a continuous span of days in a month.
///
/// Use the `days` method on `YearMonth` to create instances of this iterator.
#[derive(PartialEq, Debug)]
pub struct MonthDays {
    ym: YearMonth,
    range: Range<i8>,
}

impl Iterator for MonthDays {
    type Item = local::Date;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().and_then(|d| local::Date::ymd(self.ym.year, self.ym.month, d).ok())
    }
}

impl DoubleEndedIterator for MonthDays {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().and_then(|d| local::Date::ymd(self.ym.year, self.ym.month, d).ok())
    }
}
