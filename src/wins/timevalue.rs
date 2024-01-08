use std::iter::Sum;
use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};
use std::time;
use crate::*;


/// # A single time value (duration)
///
/// This time value represent a duration and could be infinite.
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeValue(pub(crate) i64);

impl TimeValue {

    /// Creates a new time value for a number of clock ticks.
    ///
    /// A clock tick is the smallest duration of time taken into account
    /// by this crate.
    ///
    /// If the given number of ticks is less than -[`TimeValue::INFINITE`]
    /// or greater than [`TimeValue::INFINITE`], the time value is
    /// set to infinite.
    ///
    /// If you are sure that the number of ticks is within the finite range
    /// value, using the unsafe version [`Self::from_ticks_unchecked`] could
    /// be considered
    #[inline]
    pub fn from_ticks(t:i64) -> Self
    {
        Self(t.max(-INFINITE_TIME_VALUE))
    }

    /// Creates a new time value for a number of clock ticks.
    ///
    /// A clock tick is the smallest duration of time taken into account
    /// by this crate.
    ///
    /// # Safety
    /// If the given number of ticks is less than -[`TimeValue::INFINITE`]
    /// or greater than [`TimeValue::INFINITE`], the behaviour of the
    /// time value is unpredictable.
    ///
    /// For a safety use, consider [`Self::from_ticks`].
    #[inline]
    pub unsafe fn from_ticks_unchecked(t:i64) -> Self
    {
        Self(t)
    }

    #[inline]
    pub fn from_secs(sec:i64) -> Self
    {
        if sec > MAX_SEC {
            TimeValue::INFINITE
        } else {
            Self(sec << SUBSEC_BITLEN)
        }
    }

    fn from_fract(t:i64, unit: i64) -> Self
    {
        let sec = t/unit;
        let frac = t - sec*unit;
        if sec > MAX_SEC {
            TimeValue::INFINITE
        } else {
            // we should separate in order to deal with overflow
            Self((sec<<SUBSEC_BITLEN) + ((frac << SUBSEC_BITLEN)/unit))
        }
    }

    #[inline]
    pub fn from_years(years:i64) -> Self { TimeValue::from_months(12*years) }

    #[inline]
    pub fn from_months(months:i64) -> Self { TimeValue::from_secs(146097*24*3600/400/12*months) }

    #[inline]
    pub fn from_weeks(weeks:i64) -> Self { TimeValue::from_days(7*weeks) }

    #[inline]
    pub fn from_days(days:i64) -> Self { TimeValue::from_hours(24*days) }

    #[inline]
    pub fn from_hours(hours:i64) -> Self { TimeValue::from_mins(60*hours) }

    #[inline]
    pub fn from_mins(mins:i64) -> Self { TimeValue::from_secs(60*mins) }

    #[inline]
    pub fn from_millis(millis:i64) -> Self { TimeValue::from_fract(millis, 1_000) }

    #[inline]
    pub fn from_micros(micros:i64) -> Self { TimeValue::from_fract(micros, 1_000_000) }

    #[inline]
    pub fn from_nanos(nanos:i64) -> Self { TimeValue::from_fract(nanos, 1_000_000_000) }

    #[inline]
    pub fn as_ticks(&self) -> i64
    {
        self.0
    }

    #[inline]
    pub fn as_secs(&self) -> i64 { self.0 >> SUBSEC_BITLEN }

    #[inline]
    pub fn subsec_millis(&self) -> i32
    {
        self.subsec_nanos() / 1_000_000
    }

    #[inline]
    pub fn subsec_micros(&self) -> i32
    {
        self.subsec_nanos() / 1_000
    }

    #[inline]
    pub fn subsec_nanos(&self) -> i32
    {
        (((self.0 & SUBSEC_BITMASK) * 1_000_000_000) >> SUBSEC_BITLEN) as i32
    }

    #[inline]
    pub fn to_duration(&self) -> chrono::Duration { (*self).into() }

    #[inline]
    pub fn is_zero(&self) -> bool { self.0 == 0 }

    #[inline]
    pub fn is_positive(&self) -> bool { self.0 >= 0 }

    #[inline]
    pub fn is_negative(&self) -> bool { self.0 <= 0 }

    #[inline]
    pub fn is_strictly_positive(&self) -> bool { self.0 > 0 }

    #[inline]
    pub fn is_strictly_negative(&self) -> bool { self.0 < 0 }

    #[inline]
    pub fn floor(self, period:TimeValue) -> Self
    {
        Self(
            if self.0 >= 0 {
                (self.0/period.0)*period.0
            } else {
                (self.0/period.0-1)*period.0
            }
        )
    }

    #[inline]
    pub fn ceil(self, period:TimeValue) -> Self
    {
        Self(
            if self.0 >= 0 {
                ((self.0-1)/period.0+1)*period.0
            } else {
                ((self.0-1)/period.0)*period.0
            }
        )
    }
}


impl TimePoint for TimeValue
{
    const INFINITE: TimeValue = Self(INFINITE_TIME_VALUE);

    #[inline]
    fn is_future_infinite(&self) -> bool
    {
        self.0 == INFINITE_TIME_VALUE
    }

    #[inline]
    fn is_past_infinite(&self) -> bool {
        debug_assert_ne!(self.0, i64::MIN);
        self.0 == -INFINITE_TIME_VALUE
    }

    #[inline]
    fn is_finite(&self) -> bool { self.0.abs() != INFINITE_TIME_VALUE }

    #[inline]
    fn just_after(&self) -> TimeValue
    {
        Self(if self.is_finite() { self.0+1 } else { self.0 })
    }

    #[inline]
    fn just_before(&self) -> TimeValue
    {
        Self(if self.is_finite() { self.0 - 1 } else { self.0 })
    }
}

impl From<TimeValue> for chrono::Duration
{
    #[inline]
    fn from(value: TimeValue) -> Self {
        chrono::Duration::nanoseconds(
            ((value.0 as i128 * 1_000_000_000) >> SUBSEC_BITLEN) as i64
        )
    }
}

impl From<chrono::Duration> for TimeValue
{
    #[inline]
    fn from(t: chrono::Duration) -> Self
    {
        match t.num_nanoseconds() {
            None => TimeValue::INFINITE,
            Some(nanos) => TimeValue::from_nanos(nanos)
        }
    }
}

impl From<TimeValue> for time::Duration
{
    #[inline]
    fn from(value: TimeValue) -> Self {
        assert!( value.0 >= 0 , "can’t convert negative time value to duration");
        time::Duration::new(value.as_secs() as u64, value.subsec_nanos() as u32)
    }
}

impl From<time::Duration> for TimeValue
{
    fn from(t: time::Duration) -> Self
    {
        let sec = t.as_secs();
        if sec > i32::MAX as u64 {
            TimeValue::INFINITE
        } else {
            let ns = t.subsec_nanos() as i64;
            Self( ((sec as i64)<<SUBSEC_BITLEN) + ((ns<<SUBSEC_BITLEN)/1_000_000_000))
        }
    }
}


impl Neg for TimeValue
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output
    {
        Self(- self.0)
    }
}


impl TimeBounds for TimeValue
{
    type TimePoint = Self;
    #[inline] fn is_empty(&self) -> bool { false }
    #[inline] fn is_singleton(&self) -> bool { true }
    #[inline] fn is_bounded(&self) -> bool { self.is_finite() }
    #[inline] fn is_low_bounded(&self) -> bool { self.is_finite() }
    #[inline] fn is_up_bounded(&self) -> bool { self.is_finite() }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { *self }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { *self }
}

impl TimeConvex for TimeValue { }



impl Add for TimeValue {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self
    {
        if self.is_future_infinite() {
            assert!(!other.is_past_infinite(), "time error: +oo + -oo");
            self
        } else if self.is_past_infinite() {
            assert!(!other.is_future_infinite(), "time error: -oo + +oo");
            self
        } else if other.is_finite() {
            Self::from_ticks(self.0.saturating_add(other.0))
        } else {
            other
        }
    }
}

impl AddAssign for TimeValue {
    #[inline] fn add_assign(&mut self, other: TimeValue) { *self = *self + other; }
}

impl Sub for TimeValue {
    type Output = Self;
    #[inline] fn sub(self, v: TimeValue) -> Self { self + (-v) }
}

impl SubAssign for TimeValue {
    #[inline] fn sub_assign(&mut self, v: TimeValue) { *self += -v }
}

impl Sum for TimeValue {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.reduce(|a,b| a+b).unwrap_or(TimeValue::default())
    }
}