use std::fmt;
use std::ops::{Neg, Not};
use crate::*;

/// A union of time intervals (timevalues)
pub type TimeWindow = TimeSet<TimeValue>;

/// A union of time slots (timestamp)
pub type TimeSlots = TimeSet<Timestamp>;

/// # A union of time ranges
#[derive(Clone, Eq, PartialEq, Hash, Default)]
pub struct TimeSet<T:TimePoint>(pub(crate) Vec<TimeRange<T>>);

impl<T:TimePoint> TimeSet<T>
{
    #[inline]
    pub fn all() -> Self { Self(vec![TimeRange::all()]) }

    #[inline]
    pub fn convex(lower: T, upper: T) -> Self {
        Self(TimeRange::new(lower, upper).into_iter().collect())
    }

    #[inline]
    pub fn singleton(t: T) -> Self {
        Self(TimeRange::singleton(t).into_iter().collect())
    }

    #[inline]
    pub fn empty() -> Self { Self(vec![]) }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_,TimeRange<T>> { self.0.iter() }
}


impl<T:TimePoint> TimeSpan for TimeSet<T>
{
    type TimePoint = T;

    #[inline]
    fn is_empty(&self) -> bool { self.0.is_empty() }

    #[inline]
    fn is_singleton(&self) -> bool {
        self.is_convex() && unsafe { self.0.get_unchecked(0).is_singleton() }
    }

    #[inline]
    fn is_bounded(&self) -> bool {
        self.is_low_bounded() && self.is_up_bounded()
    }

    #[inline]
    fn is_low_bounded(&self) -> bool {
        self.0.first().map(|s| s.is_low_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_up_bounded(&self) -> bool {
        self.0.last().map(|s| s.is_up_bounded()).unwrap_or(false)
    }

    #[inline]
    fn is_convex(&self) -> bool { self.0.len() == 1 }

    #[inline]
    fn lower_bound(&self) -> Self::TimePoint {
        self.0.first().expect("empty interval").lower_bound()
    }

    #[inline]
    fn upper_bound(&self) -> Self::TimePoint {
        self.0.last().expect("empty interval").upper_bound()
    }
}


impl<T:TimePoint> Neg for TimeSet<T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut neg = self.clone();
        neg.0.iter_mut().for_each(|t| {
            let tmp = t.upper;
            t.upper = -t.lower;
            t.lower = -tmp
        });
        neg.0.reverse();
        neg
    }
}

impl<T:TimePoint> IntoIterator for TimeSet<T>
{
    type Item = TimeRange<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T,TW> FromIterator<TW> for TimeSet<T>
    where
        T:TimePoint,
        TW:TimeConvex+TimeSpan<TimePoint=T>+Not<Output=Self>
{
    fn from_iter<I: IntoIterator<Item=TW>>(iter: I) -> Self {
        iter.into_iter().fold(Self::empty(), |r,i| r|i)
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
