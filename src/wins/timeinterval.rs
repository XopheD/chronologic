use std::cmp::Ordering;
use std::fmt;
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
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TimeInterval<T:TimePoint> { pub(crate) lower:T, pub(crate) upper:T }

impl<T:TimePoint> PartialOrd for TimeInterval<T>
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        if self.lt(other){
            Some(Ordering::Less)
        } else if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.upper < other.lower
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self.eq(other)
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        other.lt(self)
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        other.le(self)
    }
}

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
    #[inline]
    pub fn new(lower: T, upper: T) -> Self {
        if (lower <= upper) && !lower.is_future_infinite() && !upper.is_past_infinite() {
            Self { lower, upper }
        } else {
            Self::empty()
        }
    }

    #[inline]
    pub fn empty() -> Self {
        Self { lower: T::INFINITE, upper: -T::INFINITE }
    }

    #[inline]
    pub fn singleton(t: T) -> Self {
        if t.is_finite() {
            Self { lower: t, upper: t }
        } else {
            Self::empty()
        }
    }

    #[inline]
    pub fn after(t: T) -> Self
    {
        if t.is_future_infinite() {
            Self::empty()
        } else {
            Self { lower: t, upper: T::INFINITE }
        }
    }

    #[inline]
    pub fn before(t: T) -> Self
    {
        if t.is_past_infinite() {
            Self::empty()
        } else {
            Self { lower: -T::INFINITE, upper: t }
        }
    }

    #[inline]
    pub fn all() -> Self {
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

impl<T:TimePoint> From<T> for TimeInterval<T> {
    #[inline] fn from(t: T) -> Self { TimeInterval::singleton(t) }
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


impl<T:TimePoint> TimeConvex for TimeInterval<T>
{

}
/*
impl<T:TimePoint> TimeConvexIterator<T> for TimeInterval<T>
{
    #[inline] fn convex_iter(&self) -> Self::Iter { std::iter::once(*self) }
    type Iter = std::iter::Once<Self>;
}
*/

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
        if self.is_singleton() {
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

impl<T:TimePoint> Neg for TimeInterval<T>
{
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self { lower: -self.upper, upper: -self.lower }
    }
}

