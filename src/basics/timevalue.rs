use std::ops::Neg;
use std::time;
use std::fmt;
use chrono::Duration;

use super::*;

/// # A single time value (duration)
///
/// This time value represent a duration and could be infinite.
#[derive(Copy, Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TimeValue(pub(crate) i64);

impl TimeValue {

    #[inline]
    pub fn from_ticks(t:i64) -> Self
    {
        Self(t.max(-INFINITE_TIME_VALUE))
    }

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
    pub fn to_duration(&self) -> Duration { (*self).into() }

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
        debug_assert_ne!( self.0, i64::MIN );
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

impl Into<chrono::Duration> for TimeValue
{
    fn into(self) -> chrono::Duration
    {
        chrono::Duration::nanoseconds(
            ((self.0 as i128 * 1_000_000_000) >> SUBSEC_BITLEN) as i64
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


impl Into<time::Duration> for TimeValue
{
    fn into(self) -> time::Duration
    {
        assert!( self.0 >= 0 , "can’t convert negative time value to duration");
        time::Duration::new(self.as_secs() as u64, self.subsec_nanos() as u32)
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


impl fmt::Debug for TimeValue
{
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
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result
    {
        if self.0 >= 0 {
            if self.is_future_infinite() {
                write!(formatter, "+oo")
            } else {
                write!(formatter, "{:?}", self.to_duration().to_std().unwrap())
            }
        } else {
            if self.is_past_infinite() {
                write!(formatter, "-oo")
            } else {
                write!(formatter, "-{:?}", (-*self).to_duration().to_std().unwrap())
            }
        }
    }
}

impl<T:TimePoint> TimeWindow for T
{
    type TimePoint = Self;
    #[inline]
    fn is_empty(&self) -> bool { self.is_finite() }
    #[inline]
    fn is_singleton(&self) -> bool { self.is_finite() }
    #[inline]
    fn is_bounded(&self) -> bool { self.is_finite() }
    #[inline]
    fn is_low_bounded(&self) -> bool { self.is_finite() }
    #[inline]
    fn is_up_bounded(&self) -> bool { self.is_finite() }
    #[inline]
    fn is_convex(&self) -> bool { true }
    #[inline]
    fn lower_bound(&self) -> Self::TimePoint { *self }
    #[inline]
    fn upper_bound(&self) -> Self::TimePoint { *self }
}

