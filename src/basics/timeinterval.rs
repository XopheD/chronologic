use std::fmt;
use std::iter::{Once, once};
use std::ops::{Add, Neg, Sub};
use crate::error::TimeError;
use super::*;
use crate::*;

/// # An alias for [`TimeInterval<TimeValue>`]
///
/// As time values are discrete, we always have
/// ]a,b[ = [a+1,b-1]
pub type TimeSpan = TimeInterval<TimeValue>;

/// # An alias for [`TimeInterval<Timestamp>`]
pub type TimeSlot = TimeInterval<Timestamp>;

/// # A generic non empty interval between two time points
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimeInterval<T:TimePoint> { pub(crate) lower:T, pub(crate) upper:T }

impl Default for TimeSpan {
    /// The default timespan is defined as `{0}`
    #[inline] fn default() -> Self {
        TimeSpan { lower: TimeValue::default(), upper: TimeValue::default() }
    }
}

impl<T:TimePoint> TimeInterval<T>
{
    #[inline]
    pub fn new(lower: T, upper: T) -> Result<Self, TimeError> {
        if lower < upper {
            Ok(Self { lower, upper })
        } else if lower == upper {
            if lower.is_future_infinite() {
                Err(TimeError::FutureOverflow)
            } else if upper.is_past_infinite() {
                Err(TimeError::PastOverflow)
            } else {
                // in fact, a singleton
                Ok(Self { lower, upper })
            }
        } else {
            Err(TimeError::EmptyInterval)
        }
    }

    #[inline]
    pub fn after(t: T) -> Result<Self, TimeError> {
        if t.is_future_infinite() {
            Err(TimeError::FutureOverflow)
        } else {
            Ok(Self { lower: t, upper: T::INFINITE })
        }
    }

    #[inline]
    pub fn before(t: T) -> Result<Self, TimeError> {
        if t.is_past_infinite() {
            Err(TimeError::PastOverflow)
        } else {
            Ok(Self { lower: -T::INFINITE, upper: t })
        }
    }

    #[inline]
    pub fn singleton(t: T) -> Result<Self, TimeError> {
        if t.is_future_infinite() {
            Err(TimeError::FutureOverflow)
        } else if t.is_past_infinite() {
            Err(TimeError::PastOverflow)
        } else {
            Ok(Self { lower: t, upper: t })
        }
    }

    #[inline]
    pub fn all() -> Self {
        Self { lower: -T::INFINITE, upper: T::INFINITE }
    }

    #[inline]
    pub fn truncate_after(&self, lower: T) -> Option<Self> {
        let lower = lower.max(self.lower);
        if lower > self.upper { None } else { Some(Self { lower, upper: self.upper }) }
    }

    #[inline]
    pub fn truncate_before(&self, upper: T) -> Option<Self> {
        let upper = upper.min(self.upper);
        if self.lower > upper { None } else { Some(Self { lower: self.lower, upper }) }
    }
}

impl<T:TimePoint> TimeInterval<T>
    where T: Add<TimeValue,Output=T> + Sub<TimeValue,Output=T>
{
    #[inline]
    pub fn centered(origin: T, delta: TimeValue) -> Option<Self>
    {
        let lower = origin - delta;
        let upper = origin + delta;
        // if delta is negative, we could be surprised...
        if lower > upper { None } else { Some(Self { lower, upper })}

    }

    #[inline]
    pub fn enlarge(&self, delta: TimeValue) -> Option<Self> {
        let lower = self.lower - delta;
        let upper = self.upper + delta;
        // if delta is negative, we could be surprised...
        if lower > upper { None } else { Some(Self { lower, upper })}
    }

}

impl<T:TimePoint> TimeConvex for TimeInterval<T> {}

impl<T:TimePoint+TimeTranslation> TimeTranslation for TimeInterval<T>
{
    fn translate(&self, t: TimeValue) -> TimeResult<Self>
    {
        let lower = self.lower.translate(t)?;
        let upper = self.upper.translate(t)?;
        if lower.is_future_infinite() {
            Err(TimeError::FutureOverflow)
        } else if upper.is_past_infinite() {
            Err(TimeError::PastOverflow)
        } else {
            Ok(Self{ lower, upper })
        }
    }
}

impl<T:TimePoint> From<T> for TimeInterval<T> {
    #[inline]
    fn from(t: T) -> Self { TimeInterval::singleton(t).unwrap() }
}


impl<T:TimePoint> IntoIterator for TimeInterval<T>
{
    type Item = TimeInterval<T>;
    type IntoIter = Once<TimeInterval<T>>;
    #[inline] fn into_iter(self) -> Self::IntoIter { once(self) }
}

impl<T:TimePoint> TimeWindow for TimeInterval<T>
{
    type TimePoint = T;

    #[inline] fn is_empty(&self) -> bool { false }
    #[inline] fn is_singleton(&self) -> bool { self.lower == self.upper }
    #[inline] fn is_bounded(&self) -> bool { self.is_low_bounded() && self.is_up_bounded() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.lower.is_past_infinite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.upper.is_future_infinite() }
    #[inline] fn is_convex(&self) -> bool { true }

    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.lower }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.upper }

}


impl<T:TimePoint+fmt::Debug> fmt::Debug for TimeInterval<T>
{
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
        if self.is_low_bounded() {
            if self.is_up_bounded() {
                if self.lower == self.upper {
                    write!(formatter, "{{{}}}", self.lower)
                } else {
                    write!(formatter, "[{},{}]", self.lower, self.upper)
                }
            } else {
                write!(formatter, "[{},+oo[", self.lower)
            }
        } else {
            if self.is_up_bounded() {
                write!(formatter, "]-oo,{}]", self.upper)
            } else {
                write!(formatter, "]-oo,+oo[")
            }
        }
    }
}

impl<T:TimePoint> Neg for TimeInterval<T>
{
    type Output = Self;
    #[inline] fn neg(self) -> Self::Output {
        Self { lower: -self.upper, upper: -self.lower }
    }
}
