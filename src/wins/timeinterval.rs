use std::cmp::Ordering;
use std::ops::{Add, Neg, Sub};
use super::*;
use crate::*;


/// # An alias for [`TimeInterval<TimeValue>`]
///
/// As time values are discrete, we always have
/// ]a,b[ = [a+1,b-1]
pub type TimeSpan = TimeInterval<TimeValue>;

/// # An alias for [`TimeInterval<Timestamp>`]
pub type TimeSlot = TimeInterval<Timestamp>;

/// # A generic interval defined by its two time bounds
#[derive(Copy, Clone, Eq, Hash)]
pub struct TimeInterval<T:TimePoint> { pub(crate) lower:T, pub(crate) upper:T }


impl<T:TimePoint> Default for TimeInterval<T> {
    /// The default is defined as empty
    #[inline] fn default() -> Self { Self::empty() }
}

impl TimeSlot {
    #[inline]
    pub fn duration(&self) -> TimeValue {
        if self.upper <= self.lower { TimeValue::default() } else { self.upper - self.lower }
    }
}

impl<T:TimePoint> TimeInterval<T>
{
    /// Interval is empty if the first bound is greater than the second one.
    #[inline]
    pub fn new(lower: T, upper: T) -> Self
    {
        match lower.cmp(&upper) {
            Ordering::Less => Self { lower, upper },
            Ordering::Equal => Self::singleton(lower),
            Ordering::Greater => Self::empty()
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self {
            lower: T::INFINITE,
            upper: -T::INFINITE
        }
    }

    /// Returns `[t,t]`
    ///
    /// Interval is empty if `t` is not finite
    #[inline]
    pub fn singleton(t: T) -> Self
    {
        if t.is_finite() {
            Self { lower: t, upper: t }
        } else {
            Self::empty()
        }
    }

    /// Returns `[t,+oo[`
    ///
    /// Interval is empty if `t = +oo`
    #[inline]
    pub fn after(t: T) -> Self
    {
        if t.is_future_infinite() {
            Self::empty()
        } else {
            Self { lower: t, upper: T::INFINITE }
        }
    }

    /// Returns `]-oo,t]`
    ///
    /// Interval is empty if `t = -oo`
    #[inline]
    pub fn before(t: T) -> Self
    {
        if t.is_past_infinite() {
            Self::empty()
        } else {
            Self { lower: -T::INFINITE, upper: t }
        }
    }

    /// Returns `]-oo,+oo[`
    #[inline]
    pub fn all() -> Self
    {
        Self { lower: -T::INFINITE, upper: T::INFINITE }
    }

    /// Returns `true` if something changed
    pub fn truncate_before(&mut self, lower: T) -> bool
    {
        if lower <= self.lower {
            false
        } else {
            if self.upper < lower {
                *self = Self::empty();
            } else {
                self.lower = lower;
            }
            true
        }
    }

    /// Returns `true` if something changed
    pub fn truncate_after(&mut self, upper: T) -> bool
    {
        if self.upper <= upper {
            false
        } else {
            if upper < self.lower {
                *self = Self::empty();
            } else {
                self.upper = upper;
            }
            true
        }
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


impl<T:TimePoint> TimeBounds for TimeInterval<T>
{
    type TimePoint = T;
    #[inline] fn is_empty(&self) -> bool { self.lower.is_future_infinite() }
    #[inline] fn is_singleton(&self) -> bool { self.lower == self.upper }
    #[inline] fn is_bounded(&self) -> bool { self.is_low_bounded() && self.is_up_bounded() }
    #[inline] fn is_low_bounded(&self) -> bool { !self.lower.is_past_infinite() }
    #[inline] fn is_up_bounded(&self) -> bool { !self.upper.is_future_infinite() }
    #[inline] fn lower_bound(&self) -> Self::TimePoint { self.lower }
    #[inline] fn upper_bound(&self) -> Self::TimePoint { self.upper }
}


impl<T:TimePoint> TimeConvex for TimeInterval<T> { }


impl<T:TimePoint> Neg for TimeInterval<T>
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self { lower: -self.upper, upper: -self.lower }
    }
}



impl<T:TimePoint> From<T> for TimeInterval<T> {
    #[inline] fn from(t: T) -> Self { TimeInterval::singleton(t) }
}
