use std::fmt;
use crate::*;
use chrono::format::*;

pub trait TimeSetFormat {
    fn format_timeset(&self, timefmt: &str) -> String;
}

pub trait TimePointFormat {
    fn format_timepoint(self, timefmt: &str) -> String;
}

impl TimePointFormat for TimeValue {

    fn format_timepoint(self, _timefmt: &str) -> String {
        if self.is_positive() {
            format_duration(self.as_ticks())
        } else {
            format!("- {}", &format_duration(-self.as_ticks()))
        }
    }
}

impl TimePointFormat for Timestamp {

    fn format_timepoint(self, timefmt: &str) -> String {
        format_timestamp(self, timefmt).to_string()
    }
}

fn format_duration(t: i64) -> String
{
    assert! ( t >= 0 );
    let mut nanos = TimeValue::from_ticks(t).subsec_nanos();
    let _remaining_ticks = t - TimeValue::from_nanos(nanos as i64).as_ticks();

    fn concat_unit<F:Fn(i64)->TimeValue>(ticks: (i64,String), convert:F, unit: &str) -> (i64, String)
    {
        let x = ticks.0 / (convert)(1).as_ticks();
        if x == 0 {
            ticks
         } else {
            (ticks.0 - (convert)(x).as_ticks(), format!("{}{}{} ", &ticks.1, x, unit))
        }
    }
    if t == 0 { return "0".to_string(); }
    let ticks = (t, String::new());
    let ticks = concat_unit(ticks, TimeValue::from_years, "y");
    let ticks = concat_unit(ticks, TimeValue::from_months, "mo");
    let ticks = concat_unit(ticks, TimeValue::from_days, "d");
    let ticks = concat_unit(ticks, TimeValue::from_hours, "h");
    let ticks = concat_unit(ticks, TimeValue::from_mins, "min");
    let ticks = concat_unit(ticks, TimeValue::from_secs, "s");

    let mut str = ticks.1;
    if nanos > 1_000_000 {
        str = format!("{}{}ms ", str, nanos/1_000_000);
        nanos %= 1_000_000;
    }
    if nanos > 1_000 {
        str = format!("{}{}us ", str, nanos/1_000);
        nanos %= 1_000;
    }
    if nanos > 0 {
        str = format!("{}{}ns ", str, nanos);
    }
    /*
    todo: displaying type with format %t
    if remaining_ticks != 0 {
        str = format!("{}{}t ", str, remaining_ticks);
    }
    */

    if str.pop().is_none() {
        "0".to_string()
    } else {
        str
    }
}

fn format_timestamp(t: Timestamp, timefmt: &str) -> DelayedFormat<StrftimeItems<'_>> {
    t.to_datetime().format(timefmt)
}

fn format_timeslot<TW:TimeConvex>(tw: &TW, timefmt: &str) -> String
    where TW::TimePoint: TimePointFormat
{
    if tw.is_empty() {
        "{{}}".to_string()

    } else if tw.is_singleton() {
        format!("{{{}}}", tw.lower_bound().format_timepoint(timefmt))

    } else if tw.is_low_bounded() {
        if tw.is_up_bounded() {
            format!("[{},{}]",tw.lower_bound().format_timepoint(timefmt), tw.upper_bound().format_timepoint(timefmt))
        } else {
            format!("[{},+oo[", tw.lower_bound().format_timepoint(timefmt))
        }
    } else if tw.is_up_bounded() {
        format!("]-oo,{}]", tw.upper_bound().format_timepoint(timefmt))
    } else {
        "]-oo,+oo[".to_string()
    }
}

impl<TW:TimeWindow> TimeSetFormat for TW
    where TW::TimePoint: TimePointFormat
{
    fn format_timeset(&self, timefmt: &str) -> String
    {
        let mut iter = self.iter();
        if let Some(first) = iter.next() {
            iter.fold(format_timeslot(&first, timefmt),
                      |s,i| s + "U" + &format_timeslot(&i,timefmt))
        } else {
           "{{}}".to_string() /* empty set */
        }
    }
}



impl Debug for TimeValue
{
    #[allow(clippy::collapsible_else_if)]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.0 >= 0 {
            if self.is_future_infinite() {
                write!(formatter, "+oo")
            } else {
                write!(formatter, "{:?}", self.0)
            }
        } else {
            if self.is_past_infinite() {
                write!(formatter, "-oo")
            } else {
                write!(formatter, "{:?}", self.0)
            }
        }
    }
}


impl fmt::Display for TimeValue
{
    #[allow(clippy::collapsible_else_if)]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.0 >= 0 {
            if self.is_future_infinite() {
                write!(formatter, "+oo")
            } else {
                write!(formatter, "{}", format_duration(self.as_ticks()))
            }
        } else {
            if self.is_past_infinite() {
                write!(formatter, "-oo")
            } else {
                write!(formatter, "- {}", format_duration(-self.as_ticks()))
            }
        }
    }
}

impl fmt::Debug for Timestamp
{
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "t={:?}", self.0)
    }
}

impl fmt::Display for Timestamp
{
    #[allow(clippy::collapsible_else_if)]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.0.is_positive() {
            if self.is_future_infinite() {
                write!(formatter, "+oo")
            } else {
                write!(formatter, "{}", self.to_datetime())
            }
        } else {
            if self.is_past_infinite() {
                write!(formatter, "-oo")
            } else {
                write!(formatter, "{} before 1970-01-01 00:00:00 UTC", -self.0)
            }
        }
    }
}


impl<T:TimePoint+fmt::Debug> fmt::Debug for TimeInterval<T>
{
    #[allow(clippy::collapsible_else_if)]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.is_low_bounded() {
            if self.is_up_bounded() {
                if self.lower == self.upper {
                    write!(formatter, "{{{:?}}}", self.lower)
                } else {
                    write!(formatter, "[{:?},{:?}]", self.lower, self.upper)
                }
            } else {
                write!(formatter, "[{:?},+oo[", self.lower)
            }
        } else {
            if self.is_up_bounded() {
                write!(formatter, "]-oo,{:?}]", self.upper)
            } else {
                write!(formatter, "]-oo,+oo[")
            }
        }
    }
}


impl<T:TimePoint+fmt::Display> fmt::Display for TimeInterval<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.is_empty() {
            write!(formatter, "{{}}")

        } else if self.is_singleton() {
            write!(formatter, "{{{}}}", self.lower)

        } else if self.is_low_bounded() {
            if self.is_up_bounded() {
                write!(formatter, "[{},{}]", self.lower, self.upper)
            } else {
                write!(formatter, "[{},+oo[", self.lower)
            }
        } else if self.is_up_bounded() {
            write!(formatter, "]-oo,{}]", self.upper)
        } else {
            write!(formatter, "]-oo,+oo[")
        }
    }
}



impl<T:TimePoint+fmt::Debug> fmt::Debug for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{:?}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{:?}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}

impl<T:TimePoint+fmt::Display> fmt::Display for TimeSet<T>
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(formatter, "{}", first)?;
            iter.try_for_each(|tw| write!(formatter, "U{}", tw))
        } else {
            write!(formatter, "{{}}") /* empty set */
        }
    }
}